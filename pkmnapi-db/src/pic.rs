//! Pkmnapi pic module
//!
//! # Example
//!
//! ```
//! use pkmnapi_db::pic::*;
//!
//! let pic = Pic::new(&[0x55]).unwrap();
//!
//! assert_eq!(
//!     pic,
//!     Pic {
//!         width: 5,
//!         height: 5,
//!         pixels: vec![0x00; 1600]
//!     }
//! );
//! ```

use image::{self, DynamicImage, ImageBuffer, ImageFormat, Luma};
use std::ops::{Add, BitXor};

struct PicBitstream<'a> {
    value: Box<dyn Iterator<Item = u8> + 'a>,
}

impl<'a> PicBitstream<'a> {
    fn from(data: &'a [u8]) -> Self {
        let value = data
            .iter()
            .map(|byte| (0..=7).map(move |i| (*byte & (0x01 << (7 - i))) >> (7 - i)))
            .flatten();

        PicBitstream {
            value: Box::new(value),
        }
    }

    fn get(&mut self, length: u32) -> u32 {
        self.value
            .by_ref()
            .take(length as usize)
            .enumerate()
            .map(|(i, bit)| (bit as u32) << (length - (i as u32) - 1))
            .fold(0, |acc, val| acc | val)
    }

    fn get_until_zero(&mut self) -> (u32, u32) {
        let bits: Vec<u8> = self.value.by_ref().take_while(|bit| *bit != 0x00).collect();
        let length = (bits.len() as u32) + 1;

        let value = bits
            .iter()
            .enumerate()
            .map(|(i, bit)| (*bit as u32) << (length - (i as u32) - 1))
            .fold(0, |acc, val| acc | val);

        (value, length)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct PicBitplane {
    data: Vec<u8>,
}

impl PicBitplane {
    fn from(data: Vec<u8>) -> Self {
        PicBitplane { data }
    }

    fn delta_decode(&mut self, row_size: usize) -> PicBitplane {
        let mut state = 0x00;

        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, bit)| {
                if (i % row_size) == 0x00 {
                    state = 0x00;
                }

                if *bit == 0x00 {
                    state
                } else {
                    state ^= 0x01;

                    state
                }
            })
            .collect();

        PicBitplane { data }
    }
}

impl Add for PicBitplane {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| (a << 0x01) | b)
            .collect();

        PicBitplane { data }
    }
}

impl BitXor for PicBitplane {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self::Output {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a ^ b)
            .collect();

        PicBitplane { data }
    }
}

#[derive(Debug, PartialEq)]
pub enum PicEncodingMethod {
    ONE,
    TWO,
    THREE,
}

impl From<u8> for PicEncodingMethod {
    fn from(encoding_method: u8) -> Self {
        match encoding_method {
            0x02 => PicEncodingMethod::TWO,
            0x03 => PicEncodingMethod::THREE,
            _ => PicEncodingMethod::ONE,
        }
    }
}

/// Representation of an image
///
/// # Example
///
/// ```
/// use pkmnapi_db::pic::*;
///
/// let pic = Pic::new(&[0x55]).unwrap();
///
/// assert_eq!(
///     pic,
///     Pic {
///         width: 5,
///         height: 5,
///         pixels: vec![0x00; 1600]
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct Pic {
    pub width: u8,
    pub height: u8,
    pub pixels: Vec<u8>,
}

impl Pic {
    /// Creates a new pic
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::pic::*;
    ///
    /// let pic = Pic::new(&[0x55]).unwrap();
    ///
    /// assert_eq!(
    ///     pic,
    ///     Pic {
    ///         width: 5,
    ///         height: 5,
    ///         pixels: vec![0x00; 1600]
    ///     }
    /// );
    /// ```
    pub fn new(data: &[u8]) -> Result<Self, String> {
        let mut bitstream = PicBitstream::from(data);

        let width = bitstream.get(4) as u8;
        let height = bitstream.get(4) as u8;

        let primary_buffer = bitstream.get(1);
        let mut bitplane_1 = Pic::decode_bitplane(width, height, &mut bitstream);
        let encoding_method = PicEncodingMethod::from({
            if bitstream.get(1) == 0x00 {
                0x01
            } else {
                if bitstream.get(1) == 0x00 {
                    0x02
                } else {
                    0x03
                }
            }
        });
        let mut bitplane_0 = Pic::decode_bitplane(width, height, &mut bitstream);

        match encoding_method {
            PicEncodingMethod::ONE => {
                bitplane_1 = bitplane_1.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.delta_decode(width as usize * 8);
            }
            PicEncodingMethod::TWO => {
                bitplane_1 = bitplane_1.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.clone() ^ bitplane_1.clone();
            }
            PicEncodingMethod::THREE => {
                bitplane_1 = bitplane_1.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.clone() ^ bitplane_1.clone();
            }
        };

        let pixels = {
            if primary_buffer == 0x00 {
                bitplane_0 + bitplane_1
            } else {
                bitplane_1 + bitplane_0
            }
        };

        Ok(Pic {
            width,
            height,
            pixels: pixels.data,
        })
    }

    fn from(data: Vec<u8>, format: ImageFormat) -> Result<Self, String> {
        let raw = match image::load_from_memory_with_format(&data, format) {
            Ok(img) => img,
            Err(_) => return Err("Could not read image".to_string()),
        };

        let img = match raw.as_luma8() {
            Some(img) => img,
            None => return Err("Could not read image".to_string()),
        };

        let width = ((img.width() as f32) / 8.0) as u8;
        let height = ((img.height() as f32) / 8.0) as u8;
        let pixels: Vec<u8> = img
            .enumerate_pixels()
            .map(|(_, _, pixel)| {
                let pixel = 3 - (((pixel.0[0] as f32) / 85.0) as u8);

                pixel
            })
            .collect();

        Ok(Pic {
            width,
            height,
            pixels,
        })
    }

    pub fn from_png(data: Vec<u8>) -> Result<Self, String> {
        Pic::from(data, ImageFormat::Png)
    }

    fn decode_bitplane(width: u8, height: u8, bitstream: &mut PicBitstream) -> PicBitplane {
        let mut pairs: Vec<u8> = vec![];
        let mut packet_type = bitstream.get(1);

        loop {
            if pairs.len() >= (width as u32 * height as u32 * 32) as usize {
                break;
            }

            if packet_type == 0x00 {
                // RLE
                let (length, bits) = bitstream.get_until_zero();
                let amount = bitstream.get(bits);
                let run = length + amount + 1;

                for _ in 0..run {
                    pairs.push(0x00);
                }

                packet_type = 0x01;
            } else {
                // DATA
                loop {
                    if pairs.len() >= (width as u32 * height as u32 * 32) as usize {
                        break;
                    }

                    let pair = bitstream.get(2);

                    if pair == 0x00 {
                        break;
                    }

                    pairs.push(pair as u8);
                }

                packet_type = 0x00;
            }
        }

        let mut bitplane = vec![0x00; pairs.len() * 2];

        pairs.iter().enumerate().for_each(|(i, pair)| {
            let x = (i / (height as usize * 8)) * 2;
            let y = i % (height as usize * 8);
            let new_i = (y * (height as usize * 8)) + x;

            bitplane[new_i] = (pair & 0x02) >> 0x01;
            bitplane[new_i + 1] = pair & 0x01;
        });

        PicBitplane::from(bitplane)
    }

    fn to_img(&self, format: ImageFormat) -> Result<Vec<u8>, String> {
        let width = self.width as u32 * 8;
        let height = self.height as u32 * 8;

        let img = ImageBuffer::from_fn(width, height, |x, y| {
            let i = x + (y * width);
            let pixel = (3 - self.pixels[i as usize]) * 0x55;

            Luma([pixel])
        });

        let img = DynamicImage::ImageLuma8(img);
        let mut buf = Vec::new();

        match img.write_to(&mut buf, format) {
            Ok(_) => {}
            Err(_) => return Err("Could not write image".to_string()),
        }

        Ok(buf)
    }

    pub fn to_png(&self) -> Result<Vec<u8>, String> {
        self.to_img(ImageFormat::Png)
    }

    pub fn to_jpeg(&self) -> Result<Vec<u8>, String> {
        self.to_img(ImageFormat::Jpeg)
    }
}
