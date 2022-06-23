use crate::common::{cursor::Cursor, error::AppError};

use super::Header;

const ROS_TYPE: &'static str = "sensor_msgs/Image";

/// This message contains an uncompressed image. (0, 0) is at top-left corner of image
///
/// Struct definition from:
/// http://docs.ros.org/en/noetic/api/sensor_msgs/html/msg/Image.html
#[derive(Debug)]
pub struct Image<'a> {
    /// Header timestamp should be acquisition time of image.
    pub header: Header<'a>,

    /// Image height, that is, number of rows
    pub height: u32,

    /// Image width, that is, number of columns  
    pub width: u32,

    /// Encoding of pixels -- channel meaning, ordering, size
    ///
    /// The legal values for encoding are in file src/image_encodings.cpp
    /// If you want to standardize a new string format, join
    /// ros-users@lists.sourceforge.net and send an email proposing a new encoding.
    pub encoding: &'a str,

    /// Is this data bigendian?
    pub is_bigendian: bool,

    /// Full row length in bytes
    pub step: u32,

    /// Actual matrix data, size is (step * rows)
    pub data: &'a [u8],
}

impl<'a> Image<'a> {
    pub fn from_reader(cursor: &mut Cursor<'a>) -> Result<Self, AppError> {
        let header = Header::from_reader(cursor)?;

        let height = cursor.next_u32()?;
        let width = cursor.next_u32()?;
        let encoding =
            std::str::from_utf8(cursor.next_chunk()?).map_err(|_| AppError::InvalidUtf8String)?;

        let is_bigendian = cursor.next_u8()? != 0u8;
        let step = cursor.next_u32()?;

        // I don't know where I've lost four bytes, but now it works
        cursor.next_u32().unwrap();

        let data = cursor.next_bytes(height as u64 * step as u64)?;

        Ok(Self {
            header,
            height,
            width,
            encoding,
            is_bigendian,
            step,
            data,
        })
    }

    pub fn ros_type() -> &'static str {
        ROS_TYPE
    }
}
