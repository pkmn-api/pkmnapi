//! Pkmnapi img module
//!
//! # Example
//!
//! ```
//! use pkmnapi_db::img::*;
//!
//! let img = Img::new(&16, &16, &vec![vec![0x00; 0x40]; 0x100]).unwrap();
//!
//! assert_eq!(
//!     img,
//!     Img {
//!         width: 16,
//!         height: 16,
//!         pixels: vec![0x00; 16384]
//!     }
//! );
//! ```

use crate::error;
use image::{self, DynamicImage, ImageBuffer, ImageFormat, Luma};

/// Representation of an img
///
/// # Example
///
/// ```
/// use pkmnapi_db::img::*;
///
/// let img = Img::new(&16, &16, &vec![vec![0x00; 0x40]; 0x100]).unwrap();
///
/// assert_eq!(
///     img,
///     Img {
///         width: 16,
///         height: 16,
///         pixels: vec![0x00; 16384]
///     }
/// );
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Img {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Img {
    /// Creates a new img
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::img::*;
    ///
    /// let img = Img::new(&16, &16, &vec![vec![0x00; 0x40]; 0x100]).unwrap();
    ///
    /// assert_eq!(
    ///     img,
    ///     Img {
    ///         width: 16,
    ///         height: 16,
    ///         pixels: vec![0x00; 16384]
    ///     }
    /// );
    /// ```
    pub fn new(width: &u32, height: &u32, map_tiles: &Vec<Vec<u8>>) -> Result<Self, error::Error> {
        let pixels: Vec<u8> = (0..(height * 8))
            .map(|y| {
                (0..(width * 8))
                    .map(|x| {
                        let map_tile_x = ((x as f32) / 8.0) as u32;
                        let map_tile_y = ((y as f32) / 8.0) as u32;
                        let map_tile_index = ((map_tile_y * width) + map_tile_x) as usize;
                        let map_tile = map_tiles[map_tile_index].to_vec();

                        map_tile[(((y % 8) * 8) + (x % 8)) as usize]
                    })
                    .collect::<Vec<u8>>()
            })
            .flatten()
            .collect();

        Ok(Img {
            width: *width,
            height: *height,
            pixels,
        })
    }

    fn to_img(&self, format: ImageFormat) -> Result<Vec<u8>, error::Error> {
        let width = self.width * 8;
        let height = self.height * 8;

        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let i = x + (y * width);
            let pixel = (3 - self.pixels[i as usize]) * 0x55;

            Luma([pixel])
        });

        let img = DynamicImage::ImageLuma8(img);
        let mut buf = Vec::new();

        match img.write_to(&mut buf, format) {
            Ok(_) => {}
            Err(_) => return Err(error::Error::MapCouldNotWrite),
        }

        Ok(buf)
    }

    pub fn to_png(&self) -> Result<Vec<u8>, error::Error> {
        self.to_img(ImageFormat::Png)
    }

    pub fn to_jpeg(&self) -> Result<Vec<u8>, error::Error> {
        self.to_img(ImageFormat::Jpeg)
    }
}
