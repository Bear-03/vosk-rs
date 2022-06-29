use std::os::raw::c_int;
use vosk_sys::*;

#[derive(Debug, Clone, Copy)]
/// Log level for Kaldi messages.
pub enum LogLevel {
    /// Print Error, Info and Debug messages.
    ErrorInfoDebug,

    /// Print Error and Info, but not Debug messages (default).
    ErrorInfo,

    /// Only print Error messages.
    Error,
}

impl LogLevel {
    pub(self) fn to_c_int(self) -> c_int {
        match self {
            Self::ErrorInfo => 0,
            Self::Error => -1,
            Self::ErrorInfoDebug => 1,
        }
    }
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::ErrorInfo
    }
}

/// Set log level for Kaldi messages.
///
/// Default: [`LogLevel::ErrorInfo`].
pub fn set_log_level(log_level: LogLevel) {
    unsafe { vosk_set_log_level(log_level.to_c_int()) }
}
