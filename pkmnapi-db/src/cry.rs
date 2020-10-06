use byteorder::{LittleEndian, ReadBytesExt};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::cmp;
use std::io::Cursor;

use crate::error;

#[derive(Debug, PartialEq)]
pub struct Cry {
    pub base: u8,
    pub pitch: u8,
    pub length: u8,
    pub pulse0: CryChannel,
    pub pulse1: CryChannel,
    pub noise: CryChannel,
}

impl Default for Cry {
    fn default() -> Self {
        Cry {
            base: 0,
            pitch: 0,
            length: 0,
            pulse0: CryChannel { commands: vec![] },
            pulse1: CryChannel { commands: vec![] },
            noise: CryChannel { commands: vec![] },
        }
    }
}

impl Cry {
    pub const SAMPLES_PER_FRAME: u32 = 17556;
    pub const SAMPLE_RATE: u32 = 1048576;

    pub fn sample(bin: u16, volume: i32) -> f64 {
        ((2.0 * (bin as f64)) - 1.0) * (((volume as f64) * -1.0) / 16.0)
    }

    pub fn calc_duty(duty: u8, perc: f64) -> u16 {
        match duty {
            0 => {
                if perc >= 0.5 && perc < 0.625 {
                    1
                } else {
                    0
                }
            }
            1 => {
                if perc >= 0.5 && perc < 0.75 {
                    1
                } else {
                    0
                }
            }
            2 => {
                if perc >= 0.25 && perc < 0.75 {
                    1
                } else {
                    0
                }
            }
            3 => {
                if perc < 0.5 || perc >= 0.75 {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    pub fn generate_pulse(&self, channel: &CryChannel) -> (Vec<f64>, u32) {
        let pitch = self.pitch;
        let cry_length = (self.length as i32) - 0x80;
        let mut duty = 0;
        let mut leftovers = 0;
        let mut perc: f64 = 0.0;
        let mut samples: Vec<f64> = vec![];
        let mut total_sample_count = 0;
        let loop_count = channel
            .commands
            .iter()
            .filter_map(|command| match command {
                CryChannelCommand::Loop(loop_count) => Some(loop_count),
                _ => None,
            })
            .next()
            .unwrap_or(&1);

        let commands = channel.commands.iter().filter(|command| match command {
            CryChannelCommand::Loop(_) => false,
            _ => true,
        });

        let commands: Vec<&CryChannelCommand> = std::iter::repeat(commands)
            .take(*loop_count as usize)
            .flatten()
            .collect::<Vec<&CryChannelCommand>>();

        commands.iter().enumerate().for_each(|(i, command)| {
            let is_last = i == commands.len() - 1;

            match command {
                CryChannelCommand::DutyCycle(_duty) => {
                    duty = *_duty;

                    return;
                }
                CryChannelCommand::DutyCyclePattern(pattern) => {
                    duty = *pattern;

                    return;
                }
                CryChannelCommand::SquareNote {
                    length,
                    volume,
                    fade,
                    frequency,
                } => {
                    let subframes =
                        (((cry_length + 0x100) as u32) * ((*length as u32) + 1)) + leftovers;
                    let sample_count = Cry::SAMPLES_PER_FRAME * (subframes >> 8);
                    let period: f64 = (Cry::SAMPLE_RATE as f64)
                        * ((2048 - (((*frequency as u32) + (pitch as u32)) & 0x7FF)) as f64)
                        / 131072.0;
                    let fade = *fade as i32;
                    let mut volume = *volume as i32;

                    leftovers = subframes & 0xFF;
                    total_sample_count += sample_count;

                    for i in 0..2500000 {
                        if is_last {
                            if volume == 0 {
                                break;
                            }
                        } else {
                            if i >= sample_count {
                                break;
                            }
                        }

                        let sample = Cry::sample(Cry::calc_duty(duty & 0x03, perc), volume);

                        samples.push(sample);

                        perc += 1.0 / period;
                        perc = if perc >= 1.0 { perc - 1.0 } else { perc };

                        if (i < sample_count)
                            && (samples.len() % (Cry::SAMPLES_PER_FRAME as usize)) == 0
                        {
                            duty = ((duty & 0x3F) << 2) | ((duty & 0xC0) >> 6);
                        }

                        if fade != 0
                            && (i + 1) % (Cry::SAMPLES_PER_FRAME * (fade.abs() as u32)) == 0
                        {
                            volume += if fade < 0 { 1 } else { -1 };

                            volume = cmp::min(cmp::max(volume, 0), 0x0F);
                        }
                    }
                }
                _ => unreachable!(),
            };
        });

        (samples, total_sample_count)
    }

    pub fn generate_noise(&self, channel: &CryChannel, cutoff: &u32) -> (Vec<f64>, u32) {
        let pitch = self.pitch as u16;
        let mut leftovers = 0;
        let mut samples: Vec<f64> = vec![];
        let mut total_sample_count = 0;

        channel
            .commands
            .iter()
            .enumerate()
            .for_each(|(i, command)| {
                let is_last = i == channel.commands.len() - 1;
                let mut noise_buffer: u16 = 0x7FFF;

                match command {
                    CryChannelCommand::NoiseNote {
                        length,
                        volume,
                        fade,
                        frequency,
                    } => {
                        let subframes = (0x100 * ((*length as u32) + 1)) + leftovers;
                        let sample_count = Cry::SAMPLES_PER_FRAME * (subframes >> 8);
                        let params = (frequency + {
                            if samples.len() >= (*cutoff as usize) {
                                0
                            } else {
                                pitch
                            }
                        }) & 0xFF;
                        let shift = {
                            let shift = (params >> 4) & 0x0F;

                            if shift > 0x0D {
                                shift & 0x0D
                            } else {
                                shift
                            }
                        };
                        let divider = params & 0x07;
                        let width = (params & 0x08) == 0x08;
                        let fade = *fade as i32;
                        let mut volume = *volume as i32;

                        leftovers = subframes & 0xFF;
                        total_sample_count += sample_count;

                        for i in 0..2500000 {
                            if is_last {
                                if volume == 0 {
                                    break;
                                }
                            } else {
                                if i >= sample_count {
                                    break;
                                }
                            }

                            let bit0 = noise_buffer & 0x01;

                            samples.push(Cry::sample(0x01 ^ bit0, volume));

                            let sub_divider =
                                (if divider == 0x00 { 1 } else { divider * 2 }) as usize;

                            if samples.len() % (sub_divider * (0x01 << (shift + 1))) == 0 {
                                let bit1 = (noise_buffer >> 1) & 0x01;

                                noise_buffer = (noise_buffer >> 1) | ((bit0 ^ bit1) << 14);

                                if width {
                                    noise_buffer = (noise_buffer >> 1) | ((bit0 ^ bit1) << 6);
                                }
                            }

                            if fade != 0
                                && (i + 1) % (Cry::SAMPLES_PER_FRAME * (fade.abs() as u32)) == 0
                            {
                                volume += if fade < 0 { 1 } else { -1 };

                                volume = cmp::min(cmp::max(volume, 0), 0x0F);
                            }
                        }
                    }
                    _ => unreachable!(),
                };
            });

        (samples, total_sample_count)
    }

    pub fn generate(&self, sample_rate: u32) -> Vec<f64> {
        let (pulse0, pulse0_sample_count) = self.generate_pulse(&self.pulse0);
        let (pulse1, pulse1_sample_count) = self.generate_pulse(&self.pulse1);
        let max_sample_count = cmp::max(pulse0_sample_count, pulse1_sample_count);
        let noise_cutoff = max_sample_count - Cry::SAMPLES_PER_FRAME;
        let (noise, _) = self.generate_noise(&self.noise, &noise_cutoff);

        let sample_len = cmp::max(cmp::max(pulse0.len(), pulse1.len()), noise.len());
        let mut pulse0 = pulse0.iter();
        let mut pulse1 = pulse1.iter();
        let mut noise = noise.iter();

        let samples: Vec<f64> = (0..sample_len)
            .map(|_| {
                (pulse0.next().unwrap_or(&0.0) / 3.0)
                    + (pulse1.next().unwrap_or(&0.0) / 3.0)
                    + (noise.next().unwrap_or(&0.0) / 3.0)
            })
            .collect();

        let ratio = (Cry::SAMPLE_RATE as f64) / (sample_rate as f64);
        let volume = 50.0;

        let new_samples: Vec<f64> = (0..(((samples.len() as f64) / ratio).ceil() as usize))
            .map(|i| {
                let i = i as f64;
                let pt = (i * ratio).floor();
                let frac = i * ratio - pt;
                let sample = (volume / 256.0)
                    * ((1.0 - frac) * samples[pt as usize] + frac * samples[(pt as usize) + 1]);

                return sample;
            })
            .collect();

        new_samples
    }

    pub fn to_wav(&self, sample_rate: u32) -> Result<Vec<u8>, error::Error> {
        let spec = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let mut buf = Cursor::new(Vec::<u8>::new());
        let mut writer = match WavWriter::new(&mut buf, spec) {
            Ok(writer) => writer,
            Err(_) => return Err(error::Error::CryCouldNotCreate),
        };

        match self
            .generate(sample_rate)
            .iter()
            .map(
                |sample| match writer.write_sample((*sample * 65535.0) as i16) {
                    Ok(_) => Ok(()),
                    Err(_) => return Err(error::Error::CryCouldNotWriteSample),
                },
            )
            .collect::<Result<(), error::Error>>()
        {
            Ok(_) => {}
            Err(e) => return Err(e),
        };

        match writer.finalize() {
            Ok(_) => {}
            Err(_) => return Err(error::Error::CryCouldNotFinalize),
        };

        Ok(buf.into_inner())
    }

    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.base, self.pitch, self.length]
    }
}

#[derive(Debug, PartialEq)]
pub struct CryChannel {
    pub commands: Vec<CryChannelCommand>,
}

impl CryChannel {
    pub fn new(data: &Vec<u8>, noise: bool) -> Self {
        let mut commands = vec![];
        let mut i = 0;
        let data_len = data.len();

        while i < data_len {
            commands.push(CryChannelCommand::new(&data[i..], noise));

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

        CryChannel { commands }
    }
}

#[derive(Debug, PartialEq)]
pub enum CryChannelCommand {
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

impl CryChannelCommand {
    fn new(data: &[u8], noise: bool) -> Self {
        match data[0] {
            0xEC => return CryChannelCommand::DutyCycle(data[1]),
            0xFC => return CryChannelCommand::DutyCyclePattern(data[1]),
            0xFE => return CryChannelCommand::Loop(data[1]),
            _ => {}
        }

        if noise {
            let length = data[0] & 0x0F;
            let volume = (data[1] & 0xF0) >> 0x04;
            let fade = data[1] & 0x0F;
            let frequency = data[2] as u16;

            return CryChannelCommand::NoiseNote {
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

        CryChannelCommand::SquareNote {
            length,
            volume,
            fade,
            frequency,
        }
    }
}
