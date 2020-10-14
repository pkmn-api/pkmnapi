use super::channel_command::*;

#[derive(Debug, PartialEq)]
pub struct Channel {
    pub commands: Vec<ChannelCommand>,
}

impl Default for Channel {
    fn default() -> Self {
        Channel { commands: vec![] }
    }
}

impl Channel {
    pub fn new(data: &Vec<u8>, noise: bool) -> Self {
        let mut commands = vec![];
        let mut i = 0;
        let data_len = data.len();

        while i < data_len {
            commands.push(ChannelCommand::new(&data[i..], noise));

            let cmd = data[i];

            if cmd == 0xEC || cmd == 0xFC {
                i += 2;
            } else if cmd == 0xFE {
                i += 4;
            } else if (cmd & 0xF0) == 0x20 {
                if noise {
                    i += 3;
                } else {
                    i += 4;
                }
            } else {
                unreachable!();
            }
        }

        Channel { commands }
    }
}
