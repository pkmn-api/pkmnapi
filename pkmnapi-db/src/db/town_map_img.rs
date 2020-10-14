use crate::error::Result;
use crate::img::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    pub fn get_town_map_img(&self) -> Result<Img> {
        let graphics_offset_base = PkmnapiDB::ROM_PAGE * 0x09;
        let graphics_offset = graphics_offset_base + 0x05A8;

        let graphics_tiles: Vec<Vec<u8>> = (0..(4 * 4))
            .map(|tile_id| {
                let tile_offset = graphics_offset + (tile_id * 0x10);

                self.rom[tile_offset..(tile_offset + 0x10)]
                    .to_vec()
                    .chunks(2)
                    .map(|chunk| {
                        let hi_byte =
                            (0..8).map(|bit| (chunk[0] & (0x01 << (7 - bit))) >> (7 - bit));
                        let lo_byte =
                            (0..8).map(|bit| (chunk[1] & (0x01 << (7 - bit))) >> (7 - bit));

                        hi_byte
                            .zip(lo_byte)
                            .map(|(hi_bit, lo_bit)| (hi_bit << 0x01) | lo_bit)
                            .collect::<Vec<u8>>()
                    })
                    .flatten()
                    .collect()
            })
            .collect();

        let offset_base = PkmnapiDB::ROM_PAGE * 0x38;
        let offset = offset_base + 0x1100;

        let tiles: Vec<Vec<u8>> = self.rom[offset..]
            .iter()
            .take_while(|&x| *x != 0x00)
            .map(|byte| {
                let byte = *byte as usize;
                let tile_id = (byte & 0xF0) >> 0x04;
                let count = byte & 0x0F;

                vec![tile_id; count]
            })
            .flatten()
            .map(|tile_id| graphics_tiles[tile_id].to_vec())
            .collect::<Vec<Vec<u8>>>();

        let town_map = Img::new(&20, &18, &tiles)?;

        Ok(town_map)
    }
}
