use std::os::raw::c_int;
use vosk_sys::*;

#[derive(Debug, Default, Clone, Copy)]
/// Log level for Kaldi messages.
pub enum LogLevel {
    /// Print Error, Warn, and Info (default)
    #[default]
    Info,

    /// Print Error and Warn messages.
    Warn,

    /// Only print Error messages.
    Error,
}

impl LogLevel {
    pub(self) fn to_c_int(self) -> c_int {
        match self {
            Self::Info => 0,
            Self::Warn => -1,
            Self::Error => -2,
        }
    }
}

/// Set log level for Kaldi messages.
///
/// Default: [`LogLevel::Info`].
pub fn set_log_level(log_level: LogLevel) {
    unsafe { vosk_set_log_level(log_level.to_c_int()) }
}
