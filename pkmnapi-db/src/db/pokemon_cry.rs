use crate::cry::*;
use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl PkmnapiDB {
    pub fn get_pokemon_cry(&self, pokedex_id: &u8) -> Result<Cry> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x1446) + ((internal_id as usize) * 0x03);

        let base = self.rom[offset];
        let pitch = self.rom[offset + 1];
        let length = self.rom[offset + 2];

        let offset_base = PkmnapiDB::ROM_PAGE * 0x04;
        let offset = (offset_base + 0x3C) + ((base as usize) * 0x09);

        let cry: Cry = (0..3)
            .map(|i| {
                let cursor_offset = (offset + (i * 3)) + 1;
                let mut cursor = Cursor::new(&self.rom[cursor_offset..(cursor_offset + 2)]);

                cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
            })
            .map(|channel_offset| {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x02;
                let offset = offset_base + channel_offset;

                self.rom[offset..]
                    .iter()
                    .take_while(|&x| *x != 0xFF)
                    .map(|x| *x)
                    .collect::<Vec<u8>>()
            })
            .enumerate()
            .fold(
                Cry {
                    base,
                    pitch,
                    length,
                    ..Default::default()
                },
                |acc, (i, channel_data)| match i {
                    0 => Cry {
                        pulse0: Channel::new(&channel_data, false),
                        ..acc
                    },
                    1 => Cry {
                        pulse1: Channel::new(&channel_data, false),
                        ..acc
                    },
                    _ => Cry {
                        noise: Channel::new(&channel_data, true),
                        ..acc
                    },
                },
            );

        Ok(cry)
    }

    pub fn set_pokemon_cry(&self, pokedex_id: &u8, pokemon_cry: &Cry) -> Result<Patch> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x1446) + ((internal_id as usize) * 0x03);

        let pokemon_cry_data = pokemon_cry.to_raw();

        Ok(Patch::new(&offset, &pokemon_cry_data))
    }
}
