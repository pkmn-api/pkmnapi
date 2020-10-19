use crate::error::{self, Result};
use crate::img::*;
use crate::patch::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    pub fn get_pokemon_logo_img(&self) -> Result<Img> {
        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let offset = offset_base + 0x1380;

        let tiles = self.get_tiles(offset, 16 * 7, true);

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
