use std::io::Read;

use super::Header;

/// This message contains an uncompressed image. (0, 0) is at top-left corner of image
///
/// Struct definition from:
/// http://docs.ros.org/en/noetic/api/sensor_msgs/html/msg/Image.html
#[derive(Debug)]
pub struct Image {
    /// Header timestamp should be acquisition time of image.
    pub header: Header,

    /// Image height, that is, number of rows
    pub height: u32,

    /// Image width, that is, number of columns  
    pub width: u32,

    /// Encoding of pixels -- channel meaning, ordering, size
    ///
    /// The legal values for encoding are in file src/image_encodings.cpp
    /// If you want to standardize a new string format, join
    /// ros-users@lists.sourceforge.net and send an email proposing a new encoding.
    pub encoding: String,

    /// Is this data bigendian?
    pub is_bigendian: bool,

    /// Full row length in bytes
    pub step: u32,

    /// Actual matrix data, size is (step * rows)
    pub data: Vec<u8>,
}

impl Image {
    pub fn from_reader<R: Read>(b: &mut R) -> Self {
        let header = Header::from_reader(b);

        let mut buf_height = [0u8; 4];
        let mut buf_width = [0u8; 4];
        let mut buf_encoding_len = [0u8; 4];
        let mut buf_is_bigendian = [0u8; 1];
        let mut buf_step = [0u8; 4];

        b.read(&mut buf_height).unwrap();
        b.read(&mut buf_width).unwrap();
        b.read(&mut buf_encoding_len).unwrap();

        let height = u32::from_le_bytes(buf_height);
        let width = u32::from_le_bytes(buf_width);
        let encodind_len = u32::from_le_bytes(buf_encoding_len) as usize;
        let encoding = String::from_utf8(
            b.bytes()
                .take(encodind_len)
                .filter_map(Result::ok)
                .collect::<Vec<u8>>(),
        )
        .unwrap();

        b.read(&mut buf_is_bigendian).unwrap();
        b.read(&mut buf_step).unwrap();

        let is_bigendian = buf_is_bigendian[0] != 0;
        let step = u32::from_le_bytes(buf_step);

        let mut data = vec![0u8; (height * step) as usize];
        b.read_exact(&mut data).unwrap();

        Self {
            header,
            height,
            width,
            encoding,
            is_bigendian,
            step,
            data,
        }
    }
}
