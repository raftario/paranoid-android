use std::{
    ffi::CStr,
    fmt,
    io::{self, Write},
    mem::size_of,
    ptr::null,
    sync::atomic::{AtomicUsize, Ordering},
};

use ndk_sys::{
    __android_log_is_loggable, __android_log_message, __android_log_write,
    __android_log_write_log_message, android_get_device_api_level,
};
use once_cell::sync::Lazy;
use sharded_slab::{pool::RefMut, Clear, Pool};
use tracing_core::Metadata;
use tracing_subscriber::fmt::MakeWriter;

use crate::logging::{Buffer, Priority};

pub(crate) struct AndroidLogWriter {
    tag: PooledCString,
    message: PooledCStringBuffer,

    priority: Priority,
    buffer: Buffer,
    location: Option<Location>,

    supports_api_30: bool,
}

#[derive(Debug)]
pub(crate) struct AndroidLogMakeWriter {
    tag: PooledCString,
    buffer: Buffer,
    supports_api_30: bool,
}

struct Location {
    file: PooledCStringBuffer,
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
            message: PooledCStringBuffer::new(),

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
                let mut file_buffer = PooledCStringBuffer::new();
                file_buffer.write(file.as_bytes());
                Some(Location {
                    file: file_buffer,
                    line,
                })
            }
            _ => None,
        };

        AndroidLogWriter {
            tag: self.tag.clone(),
            message: PooledCStringBuffer::new(),

            buffer: self.buffer,
            priority,
            location,

            supports_api_30: self.supports_api_30,
        }
    }
}

impl AndroidLogMakeWriter {
    pub fn new(tag: impl AsRef<[u8]>) -> Self {
        Self {
            tag: PooledCString::new(tag),
            buffer: Buffer::default(),
            supports_api_30: unsafe { android_get_device_api_level() } >= 30,
        }
    }

    pub fn with_buffer(tag: impl AsRef<[u8]>, buffer: Buffer) -> Self {
        Self {
            tag: PooledCString::new(tag),
            buffer,
            supports_api_30: unsafe { android_get_device_api_level() } >= 30,
        }
    }
}

struct PooledCString {
    key: usize,
}

#[derive(Default)]
struct PooledCStringInner {
    buf: Vec<u8>,
    refs: AtomicUsize,
}

struct PooledCStringBuffer {
    buf: RefMut<'static, Vec<u8>>,
}

static C_STRING_POOL: Lazy<Pool<PooledCStringInner>> = Lazy::new(Pool::new);
static C_STRING_BUFFER_POOL: Lazy<Pool<Vec<u8>>> = Lazy::new(Pool::new);

impl PooledCString {
    fn new(data: impl AsRef<[u8]>) -> Self {
        let key = C_STRING_POOL
            .create_with(|PooledCStringInner { refs, buf }| {
                buf.extend_from_slice(data.as_ref());

                if buf.last().copied() != Some(0) {
                    buf.push(0);
                }
                let _ = CStr::from_bytes_with_nul(buf.as_ref()).unwrap();

                refs.fetch_add(1, Ordering::SeqCst);
            })
            .unwrap();

        Self { key }
    }

    fn as_ptr(&self) -> *const u8 {
        C_STRING_POOL.get(self.key).unwrap().buf.as_ptr()
    }
}

impl Drop for PooledCString {
    fn drop(&mut self) {
        let key = self.key;
        if C_STRING_POOL
            .get(key)
            .unwrap()
            .refs
            .fetch_sub(1, Ordering::SeqCst)
            < 1
        {
            C_STRING_POOL.clear(key);
        }
    }
}

impl Clone for PooledCString {
    fn clone(&self) -> Self {
        let key = self.key;
        C_STRING_POOL
            .get(key)
            .unwrap()
            .refs
            .fetch_add(1, Ordering::SeqCst);
        Self { key }
    }
}

impl fmt::Debug for PooledCString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(C_STRING_POOL.get(self.key).unwrap().buf.as_slice(), f)
    }
}

impl Clear for PooledCStringInner {
    fn clear(&mut self) {
        self.refs.store(0, Ordering::SeqCst);
        self.buf.clear();
    }
}

impl PooledCStringBuffer {
    fn new() -> Self {
        Self {
            buf: C_STRING_BUFFER_POOL.create().unwrap(),
        }
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

impl Drop for PooledCStringBuffer {
    fn drop(&mut self) {
        C_STRING_BUFFER_POOL.clear(self.buf.key());
    }
}
