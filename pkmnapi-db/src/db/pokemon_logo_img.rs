use crate::error::{self, Result};
use crate::img::*;
use crate::patch::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    pub fn get_pokemon_logo_img(&self) -> Result<Img> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let offset = offset_base + 0x1380;

        let tiles: Vec<Vec<u8>> = (0..(16 * 7))
            .map(|tile_id| {
                let tile_offset = offset + (tile_id * 0x10);

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
            .collect();

        let pokemon_logo = Img::new(&16, &7, &tiles)?;

        Ok(pokemon_logo)
    }

    pub fn set_pokemon_logo_img(&self, pokemon_logo: &Img) -> Result<Patch> {
        let old_pokemon_logo = self.get_pokemon_logo_img()?;
        let old_pokemon_logo_data = old_pokemon_logo.to_2bpp()?;
        let old_pokemon_logo_data_len = old_pokemon_logo_data.len();
        let pokemon_logo_data = pokemon_logo.to_2bpp()?;
        let pokemon_logo_data_len = pokemon_logo_data.len();

        if old_pokemon_logo_data_len != pokemon_logo_data_len {
            return Err(error::Error::PokemonLogoWrongSize(
                old_pokemon_logo_data_len,
                pokemon_logo_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let offset = offset_base + 0x1380;

        Ok(Patch::new(&offset, &pokemon_logo_data))
    }
}
