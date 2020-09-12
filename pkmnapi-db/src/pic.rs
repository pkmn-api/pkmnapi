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
//!         pixels: vec![0x00; 1600],
//!         bytes: 801,
//!         encoding_method: PicEncodingMethod::ONE(0x00)
//!     }
//! );
//! ```

use image::{self, DynamicImage, ImageBuffer, ImageFormat, Luma};
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

pub struct PicBitstream<'a> {
    pub value: Box<dyn Iterator<Item = u8> + 'a>,
    pub bits: usize,
}

impl<'a> PicBitstream<'a> {
    pub fn from(data: &'a [u8]) -> Self {
        let value = data
            .iter()
            .map(|byte| (0..=7).map(move |i| (*byte & (0x01 << (7 - i))) >> (7 - i)))
            .flatten();

        PicBitstream {
            value: Box::new(value),
            bits: 0,
        }
    }

    pub fn get(&mut self, length: u32) -> u32 {
        let output = self
            .value
            .by_ref()
            .take(length as usize)
            .enumerate()
            .map(|(i, bit)| (bit as u32) << (length - (i as u32) - 1))
            .fold(0, |acc, val| acc | val);

        self.bits += length as usize;

        output
    }

    pub fn get_until_zero(&mut self) -> (u32, u32) {
        let bits: Vec<u8> = self.value.by_ref().take_while(|bit| *bit != 0x00).collect();
        let length = (bits.len() as u32) + 1;

        let value = bits
            .iter()
            .enumerate()
            .map(|(i, bit)| (*bit as u32) << (length - (i as u32) - 1))
            .fold(0, |acc, val| acc | val);

        self.bits += length as usize;

        (value, length)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PicBitplane {
    pub data: Vec<u8>,
}

impl PicBitplane {
    pub fn delta_decode(&mut self, row_size: usize) -> PicBitplane {
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

    pub fn delta_encode(&mut self, row_size: usize) -> PicBitplane {
        let mut state = 0x00;

        let data = self
            .data
            .iter()
            .enumerate()
            .map(|(i, bit)| {
                if (i % row_size) == 0x00 {
                    state = 0x00;
                }

                if *bit == state {
                    0x00
                } else {
                    state ^= 0x01;

                    0x01
                }
            })
            .collect();

        PicBitplane { data }
    }
}

impl From<Vec<u8>> for PicBitplane {
    fn from(data: Vec<u8>) -> Self {
        PicBitplane { data }
    }
}

impl BitOr for PicBitplane {
    type Output = Self;

    fn bitor(self, other: Self) -> Self::Output {
        let data = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(a, b)| a | b)
            .collect();

        PicBitplane { data }
    }
}

impl BitAnd<u8> for PicBitplane {
    type Output = Self;

    fn bitand(self, other: u8) -> Self::Output {
        let data = self.data.iter().map(|a| a & other).collect();

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

impl Shl<u8> for PicBitplane {
    type Output = Self;

    fn shl(self, other: u8) -> Self::Output {
        let data = self.data.iter().map(|a| a << other).collect();

        PicBitplane { data }
    }
}

impl Shr<u8> for PicBitplane {
    type Output = Self;

    fn shr(self, other: u8) -> Self::Output {
        let data = self.data.iter().map(|a| a >> other).collect();

        PicBitplane { data }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PicEncodingMethod {
    ONE(u8),
    TWO(u8),
    THREE(u8),
}

impl PicEncodingMethod {
    pub fn value(&self) -> u8 {
        match self {
            PicEncodingMethod::ONE(primary_buffer) => *primary_buffer,
            PicEncodingMethod::TWO(primary_buffer) => *primary_buffer,
            PicEncodingMethod::THREE(primary_buffer) => *primary_buffer,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        match self {
            PicEncodingMethod::ONE(_) => vec![0x00],
            PicEncodingMethod::TWO(_) => vec![0x01, 0x00],
            PicEncodingMethod::THREE(_) => vec![0x01, 0x01],
        }
    }

    pub fn from(encoding_method: u8, primary_buffer: u8) -> Self {
        match encoding_method {
            0x02 => PicEncodingMethod::TWO(primary_buffer),
            0x03 => PicEncodingMethod::THREE(primary_buffer),
            _ => PicEncodingMethod::ONE(primary_buffer),
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
///         pixels: vec![0x00; 1600],
///         bytes: 801,
///         encoding_method: PicEncodingMethod::ONE(0x00)
///     }
/// );
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Pic {
    pub width: u8,
    pub height: u8,
    pub pixels: Vec<u8>,
    pub bytes: usize,
    pub encoding_method: PicEncodingMethod,
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
    ///         pixels: vec![0x00; 1600],
    ///         bytes: 801,
    ///         encoding_method: PicEncodingMethod::ONE(0x00)
    ///     }
    /// );
    /// ```
    pub fn new(data: &[u8]) -> Result<Self, String> {
        let mut bitstream = PicBitstream::from(data);

        let width = bitstream.get(4) as u8;
        let height = bitstream.get(4) as u8;

        let primary_buffer = bitstream.get(1) as u8;
        let mut bitplane_1 = Pic::decode_bitplane(width, height, &mut bitstream);
        let encoding_method = PicEncodingMethod::from(
            {
                if bitstream.get(1) == 0x00 {
                    0x01
                } else {
                    if bitstream.get(1) == 0x00 {
                        0x02
                    } else {
                        0x03
                    }
                }
            },
            primary_buffer,
        );
        let mut bitplane_0 = Pic::decode_bitplane(width, height, &mut bitstream);

        match encoding_method {
            PicEncodingMethod::ONE(_) => {
                bitplane_1 = bitplane_1.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.delta_decode(width as usize * 8);
            }
            PicEncodingMethod::TWO(_) => {
                bitplane_1 = bitplane_1.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.clone() ^ bitplane_1.clone();
            }
            PicEncodingMethod::THREE(_) => {
                bitplane_1 = bitplane_1.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.delta_decode(width as usize * 8);
                bitplane_0 = bitplane_0.clone() ^ bitplane_1.clone();
            }
        };

        let pixels = {
            if encoding_method.value() == 0x00 {
                (bitplane_0 << 0x01) | bitplane_1
            } else {
                (bitplane_1 << 0x01) | bitplane_0
            }
        };

        let bytes = ((bitstream.bits as f32) / 8.0).ceil() as usize;

        Ok(Pic {
            width,
            height,
            pixels: pixels.data,
            bytes,
            encoding_method,
        })
    }

    pub fn encode(&self, encoding_method: PicEncodingMethod) -> Vec<u8> {
        let width = self.width;
        let height = self.height;

        let (mut bitplane_0b, mut bitplane_1b) = if encoding_method.value() == 0x00 {
            let pixel_bitplane = PicBitplane::from(self.pixels.to_vec());

            (
                (pixel_bitplane.clone() & 0x02) >> 0x01,
                pixel_bitplane.clone() & 0x01,
            )
        } else {
            let pixel_bitplane = PicBitplane::from(self.pixels.to_vec());

            (
                pixel_bitplane.clone() & 0x01,
                (pixel_bitplane.clone() & 0x02) >> 0x01,
            )
        };

        let (bitplane_0a, bitplane_1a) = match encoding_method {
            PicEncodingMethod::ONE(_) => {
                let bitplane_1a = bitplane_1b.delta_encode(width as usize * 8);
                let bitplane_0a = bitplane_0b.delta_encode(width as usize * 8);

                (bitplane_0a, bitplane_1a)
            }
            PicEncodingMethod::TWO(_) => {
                let bitplane_1a = bitplane_1b.delta_encode(width as usize * 8);
                let bitplane_0a = bitplane_0b.clone() ^ bitplane_1b.clone();

                (bitplane_0a, bitplane_1a)
            }
            PicEncodingMethod::THREE(_) => {
                let bitplane_1a = bitplane_1b.delta_encode(width as usize * 8);
                let bitplane_0a = bitplane_0b.delta_encode(width as usize * 8);
                let bitplane_0a = bitplane_0a.clone() ^ bitplane_1a.clone();

                (bitplane_0a, bitplane_1a)
            }
        };

        let bitplane_0a_raw = Pic::encode_bitplane(width, height, bitplane_0a);
        let bitplane_1a_raw = Pic::encode_bitplane(width, height, bitplane_1a);

        let data_bits: Vec<u8> = [
            (0..=3)
                .map(|i| (width & (i << (3 - i))) >> (3 - i))
                .collect(),
            (0..=3)
                .map(|i| (height & (i << (3 - i))) >> (3 - i))
                .collect(),
            vec![encoding_method.value()],
            bitplane_1a_raw,
            encoding_method.to_vec(),
            bitplane_0a_raw,
        ]
        .concat();

        let data_bytes = ((data_bits.len() as f32) / 8.0).ceil() as usize;
        let extra_bits = (data_bytes * 8) - data_bits.len();
        let data_bits = [data_bits, vec![0x00; extra_bits]].concat();

        let data: Vec<u8> = (0..data_bytes)
            .map(|i| {
                let index = i * 8;

                data_bits[index..(index + 8)]
                    .iter()
                    .enumerate()
                    .map(|(i, bit)| bit << (7 - i))
                    .fold(0, |acc, val| acc | val)
            })
            .collect();

        data
    }

    fn from(data: Vec<u8>, format: ImageFormat) -> Result<Self, String> {
        let raw = match image::load_from_memory_with_format(&data, format) {
            Ok(img) => img,
            Err(_) => return Err("Could not read image".to_owned()),
        };

        let img = match raw.as_luma8() {
            Some(img) => img,
            None => return Err("Could not read image".to_owned()),
        };

        if img.width() % 8 != 0 || img.height() % 8 != 0 {
            return Err("Image dimensions must be multiples of 8".to_owned());
        }

        let width = ((img.width() as f32) / 8.0) as u8;
        let height = ((img.height() as f32) / 8.0) as u8;
        let pixels: Vec<u8> = img
            .enumerate_pixels()
            .map(|(_, _, pixel)| {
                let pixel = 3 - (((pixel.0[0] as f32) / 85.0) as u8);

                pixel
            })
            .collect();

        let encoding_method = PicEncodingMethod::from(0x01, 0x00);

        Ok(Pic {
            width,
            height,
            pixels,
            bytes: 0,
            encoding_method,
        })
    }

    pub fn from_png(data: Vec<u8>) -> Result<Self, String> {
        Pic::from(data, ImageFormat::Png)
    }

    pub fn from_jpeg(data: Vec<u8>) -> Result<Self, String> {
        Pic::from(data, ImageFormat::Jpeg)
    }

    pub fn decode_bitplane(width: u8, height: u8, bitstream: &mut PicBitstream) -> PicBitplane {
        let mut pairs: Vec<u8> = vec![];
        let mut packet_type = bitstream.get(1);

        loop {
            if pairs.len() >= (width as u32 * height as u32 * 32) as usize {
                break;
            }

            if packet_type == 0x00 {
                // RLE
                let (length, bits) = bitstream.get_until_zero();
                let value = bitstream.get(bits);
                let run = length + value + 1;

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

    pub fn encode_bitplane(width: u8, height: u8, bitplane: PicBitplane) -> Vec<u8> {
        let mut output = vec![];
        let pixels: Vec<usize> = (0..(width * 8))
            .step_by(2)
            .map(|x| {
                (0..((height as usize) * 8))
                    .map(move |y| (y * ((height as usize) * 8)) + (x as usize))
            })
            .flatten()
            .collect();
        let mut pairs = pixels
            .into_iter()
            .map(|i| (bitplane.data[i] << 0x01) | bitplane.data[i + 1])
            .peekable();

        let mut packet_type = match pairs.peek() {
            Some(&chunk) if chunk == 0x00 => 0x00,
            _ => 0x01,
        };

        output.push(packet_type);

        loop {
            if pairs.peek() == None {
                break;
            }

            if packet_type == 0x00 {
                // RLE

                let mut output_data = vec![];
                let run = {
                    let mut run = 0;

                    while pairs.peek() == Some(&0x00) {
                        pairs.next();
                        run += 1;
                    }

                    run as i32
                };
                let bits = if run == 0x01 {
                    1
                } else {
                    (run as f32 + 1.0).log2().floor() as u32
                };
                let value = (run + 1) & !(1 << bits);
                let length = run - value - 1;

                (0..bits).for_each(|i| {
                    let bit = ((length & (0x01 << (bits - i - 1))) >> (bits - i - 1)) as u8;

                    output_data.push(bit);
                });

                (0..bits).for_each(|i| {
                    let bit = ((value & (0x01 << (bits - i - 1))) >> (bits - i - 1)) as u8;

                    output_data.push(bit);
                });

                output.append(&mut output_data);

                packet_type = 0x01;
            } else {
                // DATA

                let mut output_data = vec![];

                while pairs.peek() != Some(&0x00) {
                    let pair = pairs.next().unwrap();

                    output_data.push((pair & 0x02) >> 0x01);
                    output_data.push(pair & 0x01);

                    if pairs.peek() == None {
                        break;
                    }
                }

                output_data.push(0x00);
                output_data.push(0x00);

                output.append(&mut output_data);

                packet_type = 0x00;
            }
        }

        output
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
            Err(_) => return Err("Could not write image".to_owned()),
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
