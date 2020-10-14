use crate::error::Result;
use crate::img::*;
use crate::PkmnapiDB;
use byteorder::{LittleEndian, ReadBytesExt};
use std::cmp;
use std::io::Cursor;

impl PkmnapiDB {
    fn get_icon_frame(&self, icon_id: &u8, frame_index: &u8) -> Result<Img> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = offset_base + 0x17C0;

        let frame_index = cmp::min(*frame_index as usize, 1);

        let icon_data: Vec<(usize, usize)> = (0..28)
            .map(|i| {
                let data_offset = offset + (i * 0x06);

                let tile_count = self.rom[data_offset + 2] as usize;
                let bank = self.rom[data_offset + 3] as usize;
                let pointer = (bank * (PkmnapiDB::ROM_PAGE * 2)) - (PkmnapiDB::ROM_PAGE * 2) + {
                    let mut cursor = Cursor::new(&self.rom[data_offset..(data_offset + 2)]);

                    cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
                };

                (pointer, tile_count)
            })
            .collect();

        let icon_datum = if *icon_id == 2 {
            vec![((PkmnapiDB::ROM_PAGE * 0x08) + 0x1180, 4)]
        } else if *icon_id < 6 {
            let icon_data_index = if *icon_id >= 3 {
                (*icon_id as usize) - 1
            } else {
                *icon_id as usize
            } + (frame_index * 14);

            vec![icon_data[icon_data_index]]
        } else {
            let icon_data_index = (5 + (((*icon_id as usize) - 6) * 2)) + (frame_index * 14);

            vec![icon_data[icon_data_index], icon_data[icon_data_index + 1]]
        };

        let mut tiles: Vec<Vec<u8>> = icon_datum
            .iter()
            .map(|datum| {
                let (pointer, tile_count) = datum;

                (0..*tile_count)
                    .map(|tile_id| {
                        let tile_offset = pointer + (tile_id * 0x10);

                        self.rom[tile_offset..(tile_offset + 0x10)]
                            .to_vec()
                            .chunks(2)
                            .map(|chunk| {
                                let hi_byte =
                                    (0..8).map(|bit| (chunk[1] & (0x01 << (7 - bit))) >> (7 - bit));
                                let lo_byte =
                                    (0..8).map(|bit| (chunk[0] & (0x01 << (7 - bit))) >> (7 - bit));

                                hi_byte
                                    .zip(lo_byte)
                                    .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                                    .collect::<Vec<u8>>()
                            })
                            .flatten()
                            .collect()
                    })
                    .collect::<Vec<Vec<u8>>>()
            })
            .flatten()
            .collect();

        if *icon_id <= 5 {
            if *icon_id != 2 {
                tiles[1] = tiles[0]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect();

                tiles[3] = tiles[2]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect();
            }
        } else {
            tiles = vec![
                tiles[0].to_vec(),
                tiles[0]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect(),
                tiles[1].to_vec(),
                tiles[1]
                    .chunks(8)
                    .map(|chunk| {
                        let mut chunk = chunk.to_vec();

                        chunk.reverse();

                        chunk
                    })
                    .flatten()
                    .collect(),
            ];
        }

        let mut icon = Img::new(&2, &2, &tiles)?;

        if [1, 2].contains(icon_id) && frame_index == 1 {
            icon.pixels = [icon.pixels[16..].to_vec(), icon.pixels[..16].to_vec()].concat();
        }

        Ok(icon)
    }

    pub fn get_icon(&self, icon_id: &u8) -> Result<Gif> {
        let (_min_id, _max_id) = self.icon_id_validate(icon_id)?;

        let frame_a = self.get_icon_frame(icon_id, &0)?;
        let frame_b = self.get_icon_frame(icon_id, &1)?;

        let gif = Gif::new(&vec![frame_a, frame_b]);

        Ok(gif)
    }
}
