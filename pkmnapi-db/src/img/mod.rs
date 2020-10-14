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

mod gif;

pub use crate::img::gif::*;

use crate::error::{self, Result};
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
    pub fn new(width: &u32, height: &u32, map_tiles: &Vec<Vec<u8>>) -> Result<Self> {
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

    fn from(data: Vec<u8>, format: ImageFormat) -> Result<Self> {
        let raw = match image::load_from_memory_with_format(&data, format) {
            Ok(img) => img,
            Err(_) => return Err(error::Error::ImgCouldNotRead),
        };

        let img = match raw.as_luma8() {
            Some(img) => img,
            None => return Err(error::Error::ImgCouldNotRead),
        };

        let width = ((img.width() as f32) / 8.0) as u32;
        let height = ((img.height() as f32) / 8.0) as u32;
        let pixels: Vec<u8> = img
            .enumerate_pixels()
            .map(|(_, _, pixel)| {
                let pixel = 3 - (((pixel.0[0] as f32) / 85.0) as u8);

                pixel
            })
            .collect();

        Ok(Img {
            width,
            height,
            pixels,
        })
    }

    pub fn from_png(data: Vec<u8>) -> Result<Self> {
        Img::from(data, ImageFormat::Png)
    }

    pub fn from_jpeg(data: Vec<u8>) -> Result<Self> {
        Img::from(data, ImageFormat::Jpeg)
    }

    pub fn to_2bpp(&self) -> Result<Vec<u8>> {
        let tile_indices: Vec<u32> = (0..self.height)
            .map(|tile_y| {
                (0..self.width)
                    .map(|tile_x| (tile_x * 8) + (tile_y * 8 * self.width * 8))
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .collect::<Vec<u32>>();

        let bpp2: Vec<u8> = tile_indices
            .iter()
            .map(|index| {
                let pixels: Vec<u8> = (0..8)
                    .map(|pixel_y| {
                        (0..8)
                            .map(|pixel_x| {
                                let pixel_index =
                                    (index + pixel_x + (pixel_y * self.width * 8)) as usize;

                                self.pixels[pixel_index]
                            })
                            .collect::<Vec<u8>>()
                    })
                    .flatten()
                    .collect::<Vec<u8>>();
                let hi_bytes = pixels
                    .chunks(8)
                    .map(|row| {
                        row.iter()
                            .map(|pixel| (pixel & 0x02) >> 0x01)
                            .enumerate()
                            .fold(0, |acc, (i, bit)| acc | (bit << (7 - i)))
                            as u8
                    })
                    .collect::<Vec<u8>>();
                let lo_bytes = pixels
                    .chunks(8)
                    .map(|row| {
                        row.iter()
                            .map(|pixel| pixel & 0x01)
                            .enumerate()
                            .fold(0, |acc, (i, bit)| acc | (bit << (7 - i)))
                            as u8
                    })
                    .collect::<Vec<u8>>();

                let tile: Vec<u8> = lo_bytes
                    .iter()
                    .zip(hi_bytes.iter())
                    .map(|(lo, hi)| vec![*lo, *hi])
                    .flatten()
                    .collect();

                tile
            })
            .flatten()
            .collect();

        Ok(bpp2)
    }

    fn to_img(&self, format: ImageFormat) -> Result<Vec<u8>> {
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
            Err(_) => return Err(error::Error::ImgCouldNotWrite),
        }

        Ok(buf)
    }

    pub fn to_png(&self) -> Result<Vec<u8>> {
        self.to_img(ImageFormat::Png)
    }

    pub fn to_jpeg(&self) -> Result<Vec<u8>> {
        self.to_img(ImageFormat::Jpeg)
    }
}
