use thiserror::Error;

/// Possible errors that accept_waveform methods might return.
#[derive(Error, Debug)]
pub enum AcceptWaveformError {
    /// Error returned if the user passes in a buffer of a length
    /// that exceeds the maximum supported buffer length.
    #[error(
        "the length of the provided audio buffer was {0} (expected < {})",
        i32::MAX
    )]
    BufferTooLong(usize),
}
