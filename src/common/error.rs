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

    #[error("Time start and end shouldn't be a negative (start={0}, end={1})")]
    ArgsNegativeTime(f64, f64),
    #[error("The end time cannot be earlier than the start time (start={0}, end={1})")]
    ArgsEndBeforeStart(f64, f64),
    #[error("Negative step value (step={0})")]
    ArgsNegativeStep(u32),
    #[error("Negative frames number (step={0})")]
    ArgsNegativeNumber(u32),
}
