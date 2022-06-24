use byteorder::{ByteOrder, LE};

use super::error::AppError;

/// A `Cursor` wraps an in-memory bytes and provides an
/// clear interface to work with them.
///
/// Implementation of this `Cursor` just copied from rosbag crate `Cursor`.
/// The main differences from `io::Cursor`:
/// * you can only read data from an `&[u8]`
/// * you have convenient functions for reading directly
/// into `u8`, `u32`, unix timestamp, `&[u8; N]`.
pub struct Cursor<'a> {
    data: &'a [u8],
    pos: u64,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn len(&self) -> u64 {
        self.data.len() as u64
    }

    /// Get next `n` bytes or [AppError::OutOfBounds]
    ///
    /// # Arguments
    ///
    /// * `n` - number of bytes to read
    pub fn next_bytes(&mut self, n: u64) -> Result<&'a [u8], AppError> {
        if self.pos + n > self.len() {
            return Err(AppError::OutOfBounds);
        }
        let s = self.pos as usize;
        self.pos += n;
        Ok(&self.data[s..self.pos as usize])
    }

    /// Get next bytes chunk (most likely string)
    ///
    /// The function first reads the `u32` number as `N`.
    /// Then reads and returns next `N` bytes.
    pub fn next_chunk(&mut self) -> Result<&'a [u8], AppError> {
        let n = self.next_u32()? as u64;
        self.next_bytes(n)
    }

    /// Get next byte as `u8` integer
    pub fn next_u8(&mut self) -> Result<u8, AppError> {
        Ok(self.next_bytes(1)?[0])
    }

    /// Get next 32 bits as `u32` integer
    pub fn next_u32(&mut self) -> Result<u32, AppError> {
        Ok(LE::read_u32(self.next_bytes(4)?))
    }

    /// Get next 64 bits as UNIX time
    pub fn next_time(&mut self) -> Result<u64, AppError> {
        let s = self.next_u32()? as u64;
        let ns = self.next_u32()? as u64;
        Ok(1_000_000_000 * s + ns)
    }
}
