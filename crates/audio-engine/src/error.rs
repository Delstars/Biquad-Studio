#[derive(Debug, thiserror::Error)]
pub enum AudioEngineError {
    #[error("COM initialization failed: {0}")]
    ComInit(String),
    #[error("No audio device found")]
    NoDevice,
    #[error("Device activation failed: {0}")]
    DeviceActivation(String),
    #[error("Stream initialization failed: {0}")]
    StreamInit(String),
    #[error("Audio format not supported: {0}")]
    FormatNotSupported(String),
    #[error("Engine already running")]
    AlreadyRunning,
    #[error("Engine not running")]
    NotRunning,
    #[error("Windows API error: {0}")]
    Windows(#[from] windows::core::Error),
}
