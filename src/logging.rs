use ndk_sys::{
    android_LogPriority, android_LogPriority_ANDROID_LOG_DEBUG,
    android_LogPriority_ANDROID_LOG_ERROR, android_LogPriority_ANDROID_LOG_FATAL,
    android_LogPriority_ANDROID_LOG_INFO, android_LogPriority_ANDROID_LOG_VERBOSE,
    android_LogPriority_ANDROID_LOG_WARN, log_id, log_id_LOG_ID_CRASH, log_id_LOG_ID_DEFAULT,
    log_id_LOG_ID_EVENTS, log_id_LOG_ID_KERNEL, log_id_LOG_ID_MAIN, log_id_LOG_ID_RADIO,
    log_id_LOG_ID_SECURITY, log_id_LOG_ID_STATS, log_id_LOG_ID_SYSTEM,
};
use tracing_core::Level;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Verbose = android_LogPriority_ANDROID_LOG_VERBOSE,
    Debug = android_LogPriority_ANDROID_LOG_DEBUG,
    Info = android_LogPriority_ANDROID_LOG_INFO,
    Warn = android_LogPriority_ANDROID_LOG_WARN,
    Error = android_LogPriority_ANDROID_LOG_ERROR,
    Fatal = android_LogPriority_ANDROID_LOG_FATAL,
}

/// An [Android log buffer](https://developer.android.com/ndk/reference/group/logging#log_id).
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buffer {
    /// Let the logging function choose the best log target.
    Default = log_id_LOG_ID_DEFAULT,

    /// The main log buffer.
    ///
    /// This is the only log buffer available to apps.
    Main = log_id_LOG_ID_MAIN,

    /// The crash log buffer.
    Crash = log_id_LOG_ID_CRASH,
    /// The statistics log buffer.
    Stats = log_id_LOG_ID_STATS,
    /// The event log buffer.
    Events = log_id_LOG_ID_EVENTS,
    /// The security log buffer.
    Security = log_id_LOG_ID_SECURITY,
    /// The system log buffer.
    System = log_id_LOG_ID_SYSTEM,
    /// The kernel log buffer.
    Kernel = log_id_LOG_ID_KERNEL,
    /// The radio log buffer.
    Radio = log_id_LOG_ID_RADIO,
}

impl Priority {
    pub fn as_raw(self) -> android_LogPriority {
        self as u32
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
        self as u32
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::Default
    }
}
