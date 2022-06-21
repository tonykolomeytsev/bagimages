use crate::common::{cursor::Cursor, error::AppError};

/// Struct definition from:
/// http://docs.ros.org/en/noetic/api/std_msgs/html/msg/Header.html
#[derive(Debug)]
pub struct Header<'a> {
    /// Standard metadata for higher-level stamped data types.
    /// This is generally used to communicate timestamped data
    /// in a particular coordinate frame.
    ///
    /// sequence ID: consecutively increasing ID
    pub seq: u32,

    /// Two-integer timestamp that is expressed as:
    /// * stamp.sec: seconds (stamp_secs) since epoch (in Python the variable is called 'secs')
    /// * stamp.nsec: nanoseconds since stamp_secs (in Python the variable is called 'nsecs')
    ///
    /// time-handling sugar is provided by the client library
    pub stamp: u64,

    /// Frame this data is associated with
    pub frame_id: &'a str,
}

impl<'a> Header<'a> {
    pub fn from_reader(cursor: &mut Cursor<'a>) -> Result<Self, AppError> {
        let seq = cursor.next_u32()?;
        let stamp = cursor.next_time()?;
        let frame_id =
            std::str::from_utf8(cursor.next_chunk()?).map_err(|_| AppError::InvalidUtf8String)?;

        Ok(Header {
            seq,
            stamp,
            frame_id,
        })
    }
}
