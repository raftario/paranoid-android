use std::{
    ffi::{CStr, CString},
    io::{self, Write},
    mem::size_of,
    ptr::null,
    sync::Arc,
};

use lazy_static::lazy_static;
use ndk_sys::{
    __android_log_is_loggable, __android_log_message, __android_log_write,
    __android_log_write_log_message, android_get_device_api_level,
};
use sharded_slab::{pool::RefMut, Pool};
use tracing_core::Metadata;
use tracing_subscriber::fmt::MakeWriter;

use crate::logging::{Buffer, Priority};

pub(crate) struct AndroidLogWriter {
    tag: Arc<CString>,
    message: PooledCString,

    priority: Priority,
    buffer: Buffer,
    location: Option<Location>,

    supports_api_30: bool,
}

#[derive(Debug)]
pub(crate) struct AndroidLogMakeWriter {
    tag: Arc<CString>,
    buffer: Buffer,
    supports_api_30: bool,
}

struct Location {
    file: PooledCString,
    line: u32,
}

impl Write for AndroidLogWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.message.write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let message = match self.message.as_ptr() {
            Some(m) => m,
            _ => return Ok(()),
        };

        let tag = self.tag.as_ptr();
        let priority = self.priority.as_raw() as i32;

        if self.supports_api_30 {
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
        } else {
            unsafe { __android_log_write(priority, tag, message) };
        }

        Ok(())
    }
}

impl Drop for AndroidLogWriter {
    fn drop(&mut self) {
        self.flush().unwrap();
    }
}

impl<'a> MakeWriter<'a> for AndroidLogMakeWriter {
    type Writer = AndroidLogWriter;

    fn make_writer(&'a self) -> Self::Writer {
        AndroidLogWriter {
            tag: self.tag.clone(),
            message: PooledCString::empty(),

            buffer: self.buffer,
            priority: Priority::Info,
            location: None,

            supports_api_30: self.supports_api_30,
        }
    }

    fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer {
        let priority = meta.level().clone().into();

        let location = match (meta.file(), meta.line()) {
            (Some(file), Some(line)) => {
                let file = PooledCString::new(file.as_bytes());
                Some(Location { file, line })
            }
            _ => None,
        };

        AndroidLogWriter {
            tag: self.tag.clone(),
            message: PooledCString::empty(),

            buffer: self.buffer,
            priority,
            location,

            supports_api_30: self.supports_api_30,
        }
    }
}

impl AndroidLogMakeWriter {
    pub fn new(tag: impl Into<Vec<u8>>) -> Self {
        Self {
            tag: Arc::new(CString::new(tag).unwrap()),
            buffer: Buffer::default(),
            supports_api_30: unsafe { android_get_device_api_level() } >= 30,
        }
    }

    pub fn with_buffer(tag: impl Into<Vec<u8>>, buffer: Buffer) -> Self {
        Self {
            tag: Arc::new(CString::new(tag).unwrap()),
            buffer,
            supports_api_30: unsafe { android_get_device_api_level() } >= 30,
        }
    }
}

struct PooledCString {
    buf: RefMut<'static, Vec<u8>>,
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
}

impl Drop for PooledCString {
    fn drop(&mut self) {
        BUFFER_POOL.clear(self.buf.key());
    }
}
