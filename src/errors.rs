// errors.rs

/// Custom error type for audio processing.
#[derive(Debug)]
pub enum AudioError {
    IoError(std::io::Error),
    FfmpegError(String),
    InvalidParameter(String),
    // Other error types as needed
}
