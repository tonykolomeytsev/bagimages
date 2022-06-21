use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Cannot read rosbag file. Cause: {0}")]
    RosBagOpen(String),
    #[error("Invalid chunk in rosbag file. Cause: {0}")]
    RosBagInvalidChunk(String),
    #[error("Invalid message in rosbag file. Cause: {0}")]
    RosBagInvalidMesage(String),
    #[error("Invalid topic type in rosbag file: {0}, expected {1}")]
    RosBagInvalidTopicType(String, String),

    #[error("Out of bounds when reading byte stream")]
    OutOfBounds,
    #[error("Invalid UTF-8 string encountered wher reading byte stream")]
    InvalidUtf8String,
    #[error("Cannot decode frame with encoding {0}")]
    InvalidImageEncoding(String),
    #[error("Cannot save file as `{0}`. Cause: {1}")]
    CannotSave(String, String),
}
