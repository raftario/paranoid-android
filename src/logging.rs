use ndk_sys::{android_LogPriority, log_id};
use tracing_core::Level;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Verbose = android_LogPriority::ANDROID_LOG_VERBOSE.0,
    Debug = android_LogPriority::ANDROID_LOG_DEBUG.0,
    Info = android_LogPriority::ANDROID_LOG_INFO.0,
    Warn = android_LogPriority::ANDROID_LOG_WARN.0,
    Error = android_LogPriority::ANDROID_LOG_ERROR.0,
    Fatal = android_LogPriority::ANDROID_LOG_FATAL.0,
}

/// An [Android log buffer](https://developer.android.com/ndk/reference/group/logging#log_id).
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buffer {
    /// Let the logging function choose the best log target.
    Default = log_id::LOG_ID_DEFAULT.0,

    /// The main log buffer.
    ///
    /// This is the only log buffer available to apps.
    Main = log_id::LOG_ID_MAIN.0,

    /// The crash log buffer.
    Crash = log_id::LOG_ID_CRASH.0,
    /// The statistics log buffer.
    Stats = log_id::LOG_ID_STATS.0,
    /// The event log buffer.
    Events = log_id::LOG_ID_EVENTS.0,
    /// The security log buffer.
    Security = log_id::LOG_ID_SECURITY.0,
    /// The system log buffer.
    System = log_id::LOG_ID_SYSTEM.0,
    /// The kernel log buffer.
    Kernel = log_id::LOG_ID_KERNEL.0,
    /// The radio log buffer.
    Radio = log_id::LOG_ID_RADIO.0,
}

impl Priority {
    pub fn as_raw(self) -> android_LogPriority {
        android_LogPriority(self as u32)
    }
}

impl From<Level> for Priority {
    fn from(l: Level) -> Self {
        match l {
            Level::TRACE => Priority::Verbose,
            Level::DEBUG => Priority::Debug,
            Level::INFO => Priority::Info,
            Level::WARN => Priority::Warn,
            Level::ERROR => Priority::Error,
        }
    }
}

impl From<Priority> for Level {
    fn from(p: Priority) -> Self {
        match p {
            Priority::Verbose => Level::TRACE,
            Priority::Debug => Level::DEBUG,
            Priority::Info => Level::INFO,
            Priority::Warn => Level::WARN,
            Priority::Error | Priority::Fatal => Level::ERROR,
        }
    }
}

impl Buffer {
    pub(crate) fn as_raw(self) -> log_id {
        log_id(self as u32)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::Default
    }
}
