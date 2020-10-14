//! Pkmnapi patch module
//!
//! # Example
//!
//! ```
//! use pkmnapi_db::patch::*;
//!
//! let patch = Patch::new(&0x123456, &vec![0x13, 0x37]);
//!
//! assert_eq!(patch.offset, 0x123456);
//! assert_eq!(patch.length, 0x02);
//! assert_eq!(patch.data, vec![0x13, 0x37]);
//! ```

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Representation of a ROM patch (IPS)
///
/// # Example
///
/// ```
/// use pkmnapi_db::patch::*;
///
/// let patch = Patch::new(&0x123456, &vec![0x13, 0x37]);
///
/// assert_eq!(patch.offset, 0x123456);
/// assert_eq!(patch.length, 0x02);
/// assert_eq!(patch.data, vec![0x13, 0x37]);
/// ```
#[derive(Debug, PartialEq)]
pub struct Patch {
    pub offset: usize,
    pub length: usize,
    pub data: Vec<u8>,
}

impl Patch {
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    ///
    /// let patch = Patch::new(&0x123456, &vec![0x13, 0x37]);
    ///
    /// assert_eq!(patch.offset, 0x123456);
    /// assert_eq!(patch.length, 0x02);
    /// assert_eq!(patch.data, vec![0x13, 0x37]);
    /// ```
    pub fn new(offset: &usize, data: &Vec<u8>) -> Patch {
        let offset = *offset;
        let data = data.to_vec();
        let length = data.len();

        Patch {
            offset,
            length,
            data,
        }
    }

    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    ///
    /// let patch = Patch::new(&0x123456, &vec![0x13, 0x37]);
    ///
    /// assert_eq!(
    ///     patch.to_raw(),
    ///     vec![0x12, 0x34, 0x56, 0x00, 0x02, 0x13, 0x37]
    /// );
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        let mut offset = vec![];
        let mut length = vec![];

        offset.write_u24::<BigEndian>(self.offset as u32).unwrap();
        length
            .write_u16::<BigEndian>(self.data.len() as u16)
            .unwrap();

        let raw = [offset, length, self.data.to_vec()].concat();

        raw
    }
}

impl From<Vec<u8>> for Patch {
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    ///
    /// let patch = Patch::from(vec![0x12, 0x34, 0x56, 0x00, 0x02, 0x13, 0x37]);
    ///
    /// assert_eq!(patch.offset, 0x123456);
    /// assert_eq!(patch.length, 0x02);
    /// assert_eq!(patch.data, vec![0x13, 0x37]);
    /// ```
    fn from(patch: Vec<u8>) -> Self {
        let mut cursor = Cursor::new(&patch);

        let offset = cursor.read_u24::<BigEndian>().unwrap_or(0) as usize;
        let length = cursor.read_u16::<BigEndian>().unwrap_or(0) as usize;
        let data = patch[5..].to_vec();

        Patch {
            offset,
            length,
            data,
        }
    }
}
