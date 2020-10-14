use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Debug, PartialEq)]
pub enum ChannelCommand {
    DutyCycle(u8),
    DutyCyclePattern(u8),
    SquareNote {
        length: u8,
        volume: u8,
        fade: u8,
        frequency: u16,
    },
    NoiseNote {
        length: u8,
        volume: u8,
        fade: u8,
        frequency: u16,
    },
    Loop(u8),
}

impl ChannelCommand {
    pub fn new(data: &[u8], noise: bool) -> Self {
        match data[0] {
            0xEC => return ChannelCommand::DutyCycle(data[1]),
            0xFC => return ChannelCommand::DutyCyclePattern(data[1]),
            0xFE => return ChannelCommand::Loop(data[1]),
            _ => {}
        }

        if noise {
            let length = data[0] & 0x0F;
            let volume = (data[1] & 0xF0) >> 0x04;
            let fade = data[1] & 0x0F;
            let frequency = data[2] as u16;

            return ChannelCommand::NoiseNote {
                length,
                volume,
                fade,
                frequency,
            };
        }

        let length = data[0] & 0x0F;
        let volume = (data[1] & 0xF0) >> 0x04;
        let fade = data[1] & 0x0F;
        let frequency = {
            let mut cursor = Cursor::new(&data[2..]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0)
        };

        ChannelCommand::SquareNote {
            length,
            volume,
            fade,
            frequency,
        }
    }
}
