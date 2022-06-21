use std::io::Read;

/// Struct definition from:
/// http://docs.ros.org/en/noetic/api/std_msgs/html/msg/Header.html
#[derive(Debug)]
pub struct Header {
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
    pub frame_id: String,
}

impl Header {
    pub fn from_reader<R: Read>(b: &mut R) -> Self {
        let mut buf_seq = [0u8; 4];
        let mut buf_stamp = [0u8; 8];
        let mut buf_frame_id_len = [0u8; 4];

        b.read(&mut buf_seq).unwrap();
        b.read(&mut buf_stamp).unwrap();
        b.read(&mut buf_frame_id_len).unwrap();

        let frame_id_len = u32::from_le_bytes(buf_frame_id_len) as usize;
        let bytes = b
            .bytes()
            .take(frame_id_len)
            .filter_map(Result::ok)
            .collect::<Vec<u8>>();

        Header {
            seq: u32::from_le_bytes(buf_seq),
            stamp: u64::from_le_bytes(buf_stamp),
            frame_id: String::from_utf8(bytes).unwrap(),
        }
    }
}
