use crate::error::Result;
use crate::img::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    pub fn get_town_map_img(&self) -> Result<Img> {
        let graphics_offset_base = PkmnapiDB::ROM_PAGE * 0x04;
        let graphics_offset = graphics_offset_base + 0x25A8;

        let graphics_tiles = self.get_tiles(graphics_offset, 4 * 4, false);

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
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
