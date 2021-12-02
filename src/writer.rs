use core::slice;
use std::{
    ffi::{CStr, CString},
    io::{self, Write},
    mem::size_of,
    ptr::null,
};

use lazy_static::lazy_static;
use sharded_slab::{pool::RefMut, Pool};
use smallvec::SmallVec;
use tracing_core::Metadata;
use tracing_subscriber::fmt::MakeWriter;

use crate::logging::{Buffer, Priority};

/// The writer produced by [`AndroidLogMakeWriter`].
#[derive(Debug)]
pub struct AndroidLogWriter<'a> {
    tag: &'a CStr,
    message: PooledCString,

    priority: Priority,
    buffer: Buffer,
    location: Option<Location>,
}

/// A [`MakeWriter`] suitable for writing Android logs.
#[derive(Debug)]
pub struct AndroidLogMakeWriter {
    tag: CString,
    buffer: Buffer,
}

#[derive(Debug)]
struct Location {
    file: PooledCString,
    line: u32,
}

// logd truncates logs at 4096 bytes, so we chunk at 4000 to be conservative
const MAX_LOG_LEN: usize = 4000;

impl Write for AndroidLogWriter<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.message.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut sv = SmallVec::<[PooledCString; 4]>::new();
        let messages = if self.message.as_bytes().len() < MAX_LOG_LEN {
            MessageIter::Single(Some(&mut self.message))
        } else {
            sv.extend(
                self.message
                    .as_bytes()
                    .chunks(MAX_LOG_LEN)
                    .map(PooledCString::new),
            );
            MessageIter::Multi(sv.as_mut().iter_mut())
        }
        .filter_map(PooledCString::as_ptr);

        let tag = self.tag.as_ptr();
        let priority = self.priority.as_raw() as i32;

        #[cfg(feature = "api-30")]
        {
            use ndk_sys::{
                __android_log_is_loggable, __android_log_message, __android_log_write_log_message,
            };

            if unsafe { __android_log_is_loggable(priority, tag, priority) } == 0 {
                return Ok(());
            }

            let buffer = self.buffer.as_raw();
            let (file, line) = match &mut self.location {
                Some(Location { file, line }) => match file.as_ptr() {
                    Some(ptr) => (ptr, *line),
                    None => (null(), 0),
                },
                None => (null(), 0),
            };

            for message in messages {
                let mut message = __android_log_message {
                    struct_size: size_of::<__android_log_message>() as u64,
                    buffer_id: buffer as i32,
                    priority,
                    tag,
                    file,
                    line,
                    message,
                };

                unsafe { __android_log_write_log_message(&mut message) };
            }
        }

        #[cfg(not(feature = "api-30"))]
        {
            use ndk_sys::__android_log_write;

            for message in messages {
                unsafe { __android_log_write(priority, tag, message) };
            }
        }

        Ok(())
    }
}

impl Drop for AndroidLogWriter<'_> {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

impl<'a> MakeWriter<'a> for AndroidLogMakeWriter {
    type Writer = AndroidLogWriter<'a>;

    fn make_writer(&'a self) -> Self::Writer {
        AndroidLogWriter {
            tag: self.tag.as_c_str(),
            message: PooledCString::empty(),

            buffer: self.buffer,
            priority: Priority::Info,
            location: None,
        }
    }

    fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer {
        let priority = (*meta.level()).into();

        let location = match (meta.file(), meta.line()) {
            (Some(file), Some(line)) => {
                let file = PooledCString::new(file.as_bytes());
                Some(Location { file, line })
            }
            _ => None,
        };

        AndroidLogWriter {
            tag: self.tag.as_c_str(),
            message: PooledCString::empty(),

            buffer: self.buffer,
            priority,
            location,
        }
    }
}

impl AndroidLogMakeWriter {
    /// Returns a new [`AndroidLogWriter`] with the given tag.
    pub fn new(tag: String) -> Self {
        Self {
            tag: CString::new(tag).unwrap(),
            buffer: Buffer::default(),
        }
    }

    /// Returns a new [`AndroidLogMakeWriter`] with the given tag and using the
    /// given [Android log buffer](Buffer).
    pub fn with_buffer(tag: String, buffer: Buffer) -> Self {
        Self {
            buffer,
            ..Self::new(tag)
        }
    }
}

#[derive(Debug)]
struct PooledCString {
    buf: RefMut<'static, Vec<u8>>,
}

enum MessageIter<'a> {
    Single(Option<&'a mut PooledCString>),
    Multi(slice::IterMut<'a, PooledCString>),
}

lazy_static! {
    static ref BUFFER_POOL: Pool<Vec<u8>> = Pool::new();
}

impl PooledCString {
    fn empty() -> Self {
        Self {
            buf: BUFFER_POOL.create().unwrap(),
        }
    }

    fn new(data: &[u8]) -> Self {
        let mut this = PooledCString::empty();
        this.write(data);
        this
    }

    fn write(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data);
    }

    fn as_ptr(&mut self) -> Option<*const u8> {
        if self.buf.last().copied() != Some(0) {
            self.buf.push(0);
        }

        CStr::from_bytes_with_nul(self.buf.as_ref())
            .ok()
            .map(CStr::as_ptr)
    }

    fn as_bytes(&self) -> &[u8] {
        self.buf.as_ref()
    }
}

impl Drop for PooledCString {
    fn drop(&mut self) {
        BUFFER_POOL.clear(self.buf.key());
    }
}

impl<'a> Iterator for MessageIter<'a> {
    type Item = &'a mut PooledCString;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            MessageIter::Single(message) => message.take(),
            MessageIter::Multi(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            MessageIter::Single(Some(_)) => (1, Some(1)),
            MessageIter::Single(None) => (0, Some(0)),
            MessageIter::Multi(iter) => iter.size_hint(),
        }
    }
}
