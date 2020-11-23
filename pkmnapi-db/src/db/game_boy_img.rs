use crate::error::Result;
use crate::img::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    pub fn get_game_boy_img(&self) -> Result<Img> {
        let graphics_offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let graphics_offset = graphics_offset_base + 0x29BE;

        let graphics_tiles = self.get_tiles(graphics_offset, 83, true);

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1E;
        let offset = offset_base + 0x1C20;

        let tiles: Vec<Vec<u8>> = self.rom[offset..(offset + 0x30)]
            .iter()
            .map(|tile_id| (*tile_id as usize) - 0x31)
            .map(|tile_id| graphics_tiles[tile_id].to_vec())
            .collect::<Vec<Vec<u8>>>();

        let game_boy = Img::new(&6, &8, &tiles)?;

        Ok(game_boy)
    }
}
