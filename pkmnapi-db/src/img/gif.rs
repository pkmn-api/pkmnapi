use super::Img;
use crate::error::{self, Result};
use gif::{Encoder, Frame, Repeat};

#[derive(Debug, PartialEq)]
pub struct Gif {
    width: u32,
    height: u32,
    frames: Vec<Img>,
}

impl Gif {
    pub fn new(frames: &Vec<Img>) -> Self {
        let width = frames[0].width;
        let height = frames[0].height;

        Gif {
            width,
            height,
            frames: frames.to_vec(),
        }
    }

    pub fn to_gif(&self, delay: u16) -> Result<Vec<u8>> {
        let width = (self.width * 8) as u16;
        let height = (self.height * 8) as u16;

        let mut buf = Vec::new();
        let color_map = &[
            0xFF, 0xFF, 0xFF, 0xAA, 0xAA, 0xAA, 0x55, 0x55, 0x55, 0x00, 0x00, 0x00,
        ];
        let mut encoder = match Encoder::new(&mut buf, width, height, color_map) {
            Ok(encoder) => encoder,
            Err(_) => return Err(error::Error::ImgCouldNotWrite),
        };

        if encoder.set_repeat(Repeat::Infinite).is_err() {
            return Err(error::Error::ImgCouldNotWrite);
        }

        for frame in &self.frames {
            let mut frame = Frame::from_indexed_pixels(width, height, &frame.pixels, None);

            frame.delay = delay;

            if encoder.write_frame(&frame).is_err() {
                return Err(error::Error::ImgCouldNotWrite);
            }
        }

        drop(encoder);

        Ok(buf)
    }
}
