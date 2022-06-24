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
    #[error("Invalid UTF-8 string encountered when reading byte stream")]
    InvalidUtf8String,
    #[error("Cannot decode frame with encoding {0}")]
    InvalidImageEncoding(String),
    #[error("Cannot save file as `{0}`. Cause: {1}")]
    CannotSave(String, String),

    #[error("Start and end times must not be negative (you specified start={0}, end={1})")]
    ArgsNegativeTime(f64, f64),
    #[error("End time is less than start time (you specified start={0}, end={1})")]
    ArgsEndBeforeStart(f64, f64),
    #[error("Step value cannot be {0} (you specified --step {0} or -S{})")]
    ArgsNegativeStep(u32),
    #[error("Number of frames to export cannot be {0} (you specified --number {0} or -n{0})")]
    ArgsNegativeNumber(u32),
    #[error("You have not specified any topic to export. Try running `bagimages --help`")]
    ArgsEmptyTopics,
}
