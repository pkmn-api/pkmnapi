use crate::error::{self, Result};
use crate::patch::*;
use crate::pic::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    /// Get Pokémon pic by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let pokemon_pic = db.get_pokemon_pic(&1, &PokemonPicFace::FRONT).unwrap();
    ///
    /// assert_eq!(pokemon_pic.width, 5);
    /// assert_eq!(pokemon_pic.height, 5);
    /// assert_eq!(pokemon_pic.pixels.len(), 1600);
    /// ```
    pub fn get_pokemon_pic(
        &self,
        pokedex_id: &u8,
        pokemon_pic_face: &PokemonPicFace,
    ) -> Result<Pic> {
        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;
        let (_, max_pokedex_id) = self.pokedex_id_bounds();

        let (offset, bank_offset) = {
            if pokedex_id == &(max_pokedex_id as u8) {
                let offset_base = PkmnapiDB::ROM_PAGE;
                let offset = offset_base + 0x025B;
                let bank_offset = (self.rom[0x163A] - 1) * 0x02;

                (offset, bank_offset as usize)
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
                let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

                let bank_offset = match internal_id {
                    _ if internal_id < self.rom[0x1646] - 1 => self.rom[0x1648],
                    _ if internal_id < self.rom[0x164D] - 1 => self.rom[0x164F],
                    _ if internal_id < self.rom[0x1654] - 1 => self.rom[0x1656],
                    _ if internal_id < self.rom[0x165B] - 1 => self.rom[0x165D],
                    _ => self.rom[0x1661],
                };
                let bank_offset = bank_offset - 1;

                (offset, bank_offset as usize)
            }
        };

        let pointer_front = self.get_pointer(offset + 11);
        let pointer_back = self.get_pointer(offset + 13);

        let offset_base = PkmnapiDB::ROM_PAGE * bank_offset;
        let offset_front = offset_base + pointer_front;
        let offset_back = offset_base + pointer_back;

        let pointer = match pokemon_pic_face {
            PokemonPicFace::FRONT => offset_front,
            PokemonPicFace::BACK => offset_back,
        };

        let pic = Pic::new(&self.rom[pointer..])?;

        Ok(pic)
    }

    /// Set Pokémon pic by Pokédex ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::pic::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let pokemon_pic = Pic::new(&vec![0x55]).unwrap();
    /// let patch = db.set_pokemon_pic(&1, &PokemonPicFace::FRONT, &pokemon_pic, PicEncodingMethod::THREE(0x01)).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x34000,
    ///         length: 0x07,
    ///         data: vec![0x55, 0xBF, 0xD2, 0x1D, 0xFE, 0x90, 0x80]
    ///     }
    /// );
    /// ```
    pub fn set_pokemon_pic(
        &self,
        pokedex_id: &u8,
        pokemon_pic_face: &PokemonPicFace,
        pic: &Pic,
        encoding_method: PicEncodingMethod,
    ) -> Result<Patch> {
        let old_pixels = self.get_pokemon_pic(pokedex_id, pokemon_pic_face)?;
        let pixels = pic.encode(encoding_method);

        if pixels.len() > old_pixels.bytes + 1 {
            return Err(error::Error::PicTooLarge);
        }

        let internal_id = self.pokedex_id_to_internal_id(pokedex_id)?;
        let (_, max_pokedex_id) = self.pokedex_id_bounds();

        let (offset, bank_offset) = {
            if pokedex_id == &(max_pokedex_id as u8) {
                let offset_base = PkmnapiDB::ROM_PAGE;
                let offset = offset_base + 0x025B;
                let bank_offset = (self.rom[0x163A] - 1) * 0x02;

                (offset, bank_offset as usize)
            } else {
                let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
                let offset = (offset_base + 0x03DE) + (((*pokedex_id as usize) - 1) * 0x1C);

                let bank_offset = match internal_id {
                    _ if internal_id < self.rom[0x1646] - 1 => self.rom[0x1648],
                    _ if internal_id < self.rom[0x164D] - 1 => self.rom[0x164F],
                    _ if internal_id < self.rom[0x1654] - 1 => self.rom[0x1656],
                    _ if internal_id < self.rom[0x165B] - 1 => self.rom[0x165D],
                    _ => self.rom[0x1661],
                };
                let bank_offset = bank_offset - 1;

                (offset, bank_offset as usize)
            }
        };

        let pointer_front = self.get_pointer(offset + 11);
        let pointer_back = self.get_pointer(offset + 13);

        let offset_base = PkmnapiDB::ROM_PAGE * bank_offset;
        let offset_front = offset_base + pointer_front;
        let offset_back = offset_base + pointer_back;

        let pointer = match pokemon_pic_face {
            PokemonPicFace::FRONT => offset_front,
            PokemonPicFace::BACK => offset_back,
        };

        Ok(Patch::new(&pointer, &pixels))
    }
}

/// Pokémon pic face
///
/// ```
/// use pkmnapi_db::*;
///
/// let pic_face = PokemonPicFace::from(Some("front".to_owned()));
///
/// assert_eq!(pic_face, PokemonPicFace::FRONT);
///
/// let pic_face = PokemonPicFace::from(Some("back".to_owned()));
///
/// assert_eq!(pic_face, PokemonPicFace::BACK);
///
/// let pic_face = PokemonPicFace::from(Some("BACK".to_owned()));
///
/// assert_eq!(pic_face, PokemonPicFace::BACK);
///
/// let pic_face = PokemonPicFace::from(Some("UnKnOwN".to_owned()));
///
/// assert_eq!(pic_face, PokemonPicFace::FRONT);
///
/// let pic_face = PokemonPicFace::from(None);
///
/// assert_eq!(pic_face, PokemonPicFace::FRONT);
/// ```
#[derive(Debug, PartialEq)]
pub enum PokemonPicFace {
    FRONT,
    BACK,
}

impl From<Option<String>> for PokemonPicFace {
    /// Convert String to PokemonPicFace
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let pic_face = PokemonPicFace::from(Some("front".to_owned()));
    ///
    /// assert_eq!(pic_face, PokemonPicFace::FRONT);
    ///
    /// let pic_face = PokemonPicFace::from(Some("back".to_owned()));
    ///
    /// assert_eq!(pic_face, PokemonPicFace::BACK);
    ///
    /// let pic_face = PokemonPicFace::from(Some("BACK".to_owned()));
    ///
    /// assert_eq!(pic_face, PokemonPicFace::BACK);
    ///
    /// let pic_face = PokemonPicFace::from(Some("UnKnOwN".to_owned()));
    ///
    /// assert_eq!(pic_face, PokemonPicFace::FRONT);
    ///
    /// let pic_face = PokemonPicFace::from(None);
    ///
    /// assert_eq!(pic_face, PokemonPicFace::FRONT);
    /// ```
    fn from(face: Option<String>) -> Self {
        match face {
            Some(face) if face.to_lowercase() == "back" => PokemonPicFace::BACK,
            _ => PokemonPicFace::FRONT,
        }
    }
}
