use crate::error::{self, Result};
use crate::map::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    pub fn get_map_pic(&self, map_id: &u8) -> Result<Map> {
        self.map_id_validate(map_id)?;

        let bank_offset_base = PkmnapiDB::ROM_PAGE * 0x03;
        let bank_offset = (bank_offset_base + 0x23D) + (*map_id as usize);
        let bank_id = self.rom[bank_offset];

        let bank = ((bank_id as usize) - 0x01) * PkmnapiDB::ROM_PAGE;

        if bank == 0x00 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let header_offset = 0x01AE + ((*map_id as usize) * 0x02);
        let header_pointer = bank + self.get_pointer(header_offset);

        let tileset = self.rom[header_pointer];

        if tileset == 0x40 || tileset == 0xC7 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let tileset_bank_pointer = 0xC7BE + ((tileset as usize) * 0x0C);
        let tileset_bank = ((self.rom[tileset_bank_pointer] as usize) - 0x01) * PkmnapiDB::ROM_PAGE;
        let tileset_block_pointer = tileset_bank + self.get_pointer(tileset_bank_pointer + 1);
        let tileset_graphics_pointer = tileset_bank + self.get_pointer(tileset_bank_pointer + 3);

        let height = self.rom[header_pointer + 1] as u32;
        let width = self.rom[header_pointer + 2] as u32;

        if width >= 0xF6 || height >= 0xF0 {
            return Err(error::Error::MapInvalid(*map_id));
        }

        let blocks_pointer = bank + self.get_pointer(header_pointer + 3);

        let blocks_data =
            self.rom[blocks_pointer..(blocks_pointer + ((width * height) as usize))].to_vec();

        let tiles = self.get_tiles(tileset_graphics_pointer, 16 * 6 * 8 * 8, false);

        let map_tiles: Vec<Vec<u8>> = (0..(width * height * 4 * 4))
            .map(|i| {
                let x = i % (width * 4);
                let y = ((i as f32) / (width * 4) as f32) as u32;
                let block_index =
                    (((((y as f32) / 4.0) as u32) * width) + (((x as f32) / 4.0) as u32)) as usize;
                let block = blocks_data[block_index] as usize;
                let block_tile_index = ((i % 4) + ((y % 4) * 4)) as usize;

                let tile_id =
                    (self.rom[tileset_block_pointer + (block * 0x10) + block_tile_index]) as usize;

                tiles[tile_id].to_vec()
            })
            .collect();

        let map = Map::new(&width, &height, &map_tiles)?;

        Ok(map)
    }
}
