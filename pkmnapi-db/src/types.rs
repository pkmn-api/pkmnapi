//! Pkmnapi types module

use crate::string::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::cmp;
use std::io::{Cursor, Read};

/// Type name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let type_name = TypeName::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     TypeName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct TypeName {
    pub name: ROMString,
}

impl From<&[u8]> for TypeName {
    /// Convert &[u8] to TypeName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let type_name = TypeName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     type_name,
    ///     TypeName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        TypeName { name }
    }
}

impl TypeName {
    /// Type name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let type_name = TypeName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = type_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

/// Type effect (attacker, defender, multiplier)
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x01, 0x02, 0x14];
/// let type_effect = TypeEffect::from(&rom[..]);
///
/// assert_eq!(
///     type_effect,
///     TypeEffect {
///         attacking_type_id: 0x01,
///         defending_type_id: 0x02,
///         multiplier: 2.0
///     }
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TypeEffect {
    pub attacking_type_id: u8,
    pub defending_type_id: u8,
    pub multiplier: f32,
}

impl From<&[u8]> for TypeEffect {
    /// Convert &[u8] to TypeEffect
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x14];
    /// let type_effect = TypeEffect::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     TypeEffect {
    ///         attacking_type_id: 0x01,
    ///         defending_type_id: 0x02,
    ///         multiplier: 2.0
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let attacking_type_id = cursor.read_u8().unwrap_or(0);
        let defending_type_id = cursor.read_u8().unwrap_or(0);
        let multiplier = (cursor.read_u8().unwrap_or(0) as f32) / 10.0;

        TypeEffect {
            attacking_type_id,
            defending_type_id,
            multiplier,
        }
    }
}

impl TypeEffect {
    /// Convert type effect to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let type_effect = TypeEffect {
    ///     attacking_type_id: 0x01,
    ///     defending_type_id: 0x02,
    ///     multiplier: 2.0,
    /// };
    ///
    /// let raw = type_effect.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x02, 0x14]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![
            self.attacking_type_id,
            self.defending_type_id,
            cmp::min((self.multiplier * 10.0) as u8, 254),
        ]
    }
}

/// Pokemon stats (HP, attack, defence, etc)
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
/// let stats = PokemonStats::from(&rom[..]);
///
/// assert_eq!(
///     stats,
///     PokemonStats {
///         pokedex_id: 0x01,
///         base_hp: 0x02,
///         base_attack: 0x03,
///         base_defence: 0x04,
///         base_speed: 0x05,
///         base_special: 0x06,
///         type_ids: vec![0x07, 0x08],
///         catch_rate: 0x09,
///         base_exp_yield: 0x0A
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokemonStats {
    pub pokedex_id: u8,
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub type_ids: Vec<u8>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}

impl From<&[u8]> for PokemonStats {
    /// Convert &[u8] to PokemonStats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
    /// let stats = PokemonStats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     stats,
    ///     PokemonStats {
    ///         pokedex_id: 0x01,
    ///         base_hp: 0x02,
    ///         base_attack: 0x03,
    ///         base_defence: 0x04,
    ///         base_speed: 0x05,
    ///         base_special: 0x06,
    ///         type_ids: vec![0x07, 0x08],
    ///         catch_rate: 0x09,
    ///         base_exp_yield: 0x0A
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let pokedex_id = cursor.read_u8().unwrap_or(0);
        let base_hp = cursor.read_u8().unwrap_or(0);
        let base_attack = cursor.read_u8().unwrap_or(0);
        let base_defence = cursor.read_u8().unwrap_or(0);
        let base_speed = cursor.read_u8().unwrap_or(0);
        let base_special = cursor.read_u8().unwrap_or(0);
        let type_ids = {
            let mut type_ids = vec![0x00; 2];

            cursor.read_exact(&mut type_ids).unwrap();

            type_ids
        };
        let catch_rate = cursor.read_u8().unwrap_or(0);
        let base_exp_yield = cursor.read_u8().unwrap_or(0);

        PokemonStats {
            pokedex_id,
            base_hp,
            base_attack,
            base_defence,
            base_speed,
            base_special,
            type_ids,
            catch_rate,
            base_exp_yield,
        }
    }
}

impl PokemonStats {
    /// Convert stats to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let stats = PokemonStats {
    ///     pokedex_id: 0x01,
    ///     base_hp: 0x02,
    ///     base_attack: 0x03,
    ///     base_defence: 0x04,
    ///     base_speed: 0x05,
    ///     base_special: 0x06,
    ///     type_ids: vec![0x07, 0x08],
    ///     catch_rate: 0x09,
    ///     base_exp_yield: 0x0A,
    /// };
    ///
    /// let raw = stats.to_raw();
    ///
    /// assert_eq!(
    ///     raw,
    ///     vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A]
    /// );
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        [
            vec![
                self.pokedex_id,
                self.base_hp,
                self.base_attack,
                self.base_defence,
                self.base_speed,
                self.base_special,
            ],
            self.type_ids.to_vec(),
            vec![self.catch_rate, self.base_exp_yield],
        ]
        .concat()
    }
}

/// Pokémon name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let pokemon_name = PokemonName::from(&rom[..]);
///
/// assert_eq!(
///     pokemon_name,
///     PokemonName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokemonName {
    pub name: ROMString,
}

impl From<&[u8]> for PokemonName {
    /// Convert &[u8] to PokemonName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let pokemon_name = PokemonName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokemon_name,
    ///     PokemonName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        PokemonName { name }
    }
}

impl PokemonName {
    /// Pokémon name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let pokemon_name = PokemonName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = pokemon_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

/// Move stats
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23];
/// let type_name = MoveStats::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     MoveStats {
///         move_id: 0x01,
///         effect: 0x00,
///         power: 0x28,
///         type_id: 0x00,
///         accuracy: 1.0,
///         pp: 0x23
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct MoveStats {
    pub move_id: u8,
    pub effect: u8,
    pub power: u8,
    pub type_id: u8,
    pub accuracy: f32,
    pub pp: u8,
}

impl From<&[u8]> for MoveStats {
    /// Convert &[u8] to MoveStats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23];
    /// let move_stats = MoveStats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     move_stats,
    ///     MoveStats {
    ///         move_id: 0x01,
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: 0x00,
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let move_id = cursor.read_u8().unwrap_or(0);
        let effect = cursor.read_u8().unwrap_or(0);
        let power = cursor.read_u8().unwrap_or(0);
        let type_id = cursor.read_u8().unwrap_or(0);
        let accuracy = (cursor.read_u8().unwrap_or(0) as f32) / 255.0;
        let pp = cursor.read_u8().unwrap_or(0);

        MoveStats {
            move_id,
            effect,
            power,
            type_id,
            accuracy,
            pp,
        }
    }
}

impl MoveStats {
    /// Pokémon name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let move_stats = MoveStats {
    ///     move_id: 0x01,
    ///     effect: 0x00,
    ///     power: 0x28,
    ///     type_id: 0x00,
    ///     accuracy: 1.0,
    ///     pp: 0x23,
    /// };
    ///
    /// let raw = move_stats.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![
            self.move_id,
            self.effect,
            self.power,
            self.type_id,
            (self.accuracy * 255.0) as u8,
            self.pp,
        ]
    }
}

/// Move name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let move_name = MoveName::from(&rom[..]);
///
/// assert_eq!(
///     move_name,
///     MoveName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct MoveName {
    pub name: ROMString,
}

impl From<&[u8]> for MoveName {
    /// Convert &[u8] to MoveName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let move_name = MoveName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     move_name,
    ///     MoveName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        MoveName { name }
    }
}

impl MoveName {
    /// Move name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let move_name = MoveName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = move_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

/// HM
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let hm = HM::from(0x01);
///
/// assert_eq!(hm, HM { move_id: 0x01 });
/// ```
#[derive(Debug, PartialEq)]
pub struct HM {
    pub move_id: u8,
}

impl From<u8> for HM {
    /// Convert u8 to HM
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let hm = HM::from(0x01);
    ///
    /// assert_eq!(hm, HM { move_id: 0x01 });
    /// ```
    fn from(move_id: u8) -> Self {
        HM { move_id }
    }
}

impl HM {
    /// HM to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let hm = HM::from(0x01);
    ///
    /// let raw = hm.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.move_id]
    }
}

/// TM
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let tm = TM::from(0x01);
///
/// assert_eq!(tm, TM { move_id: 0x01 });
/// ```
#[derive(Debug, PartialEq)]
pub struct TM {
    pub move_id: u8,
}

impl From<u8> for TM {
    /// Convert u8 to TM
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let tm = TM::from(0x01);
    ///
    /// assert_eq!(tm, TM { move_id: 0x01 });
    /// ```
    fn from(move_id: u8) -> Self {
        TM { move_id }
    }
}

impl TM {
    /// TM to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let tm = TM::from(0x01);
    ///
    /// let raw = tm.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.move_id]
    }
}

/// TM price
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let tm = TMPrice::from(0x01);
///
/// assert_eq!(tm, TMPrice { value: 1000 });
/// ```
#[derive(Debug, PartialEq)]
pub struct TMPrice {
    pub value: u32,
}

impl From<u8> for TMPrice {
    /// Convert u8 to TMPrice
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let tm = TMPrice::from(0x01);
    ///
    /// assert_eq!(tm, TMPrice { value: 1000 });
    /// ```
    fn from(tm_price: u8) -> Self {
        TMPrice {
            value: (tm_price as u32) * 1000,
        }
    }
}

impl TMPrice {
    /// TM price to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let tm_price = TMPrice::from(0x01);
    ///
    /// let raw = tm_price.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![(self.value as f32 / 1000.0) as u8]
    }
}

/// Pokédex entry
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let rom = vec![
///     0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
/// ];
/// let pokedex_entry = PokedexEntry::from(&rom[..]);
///
/// assert_eq!(
///     pokedex_entry,
///     PokedexEntry {
///         species: ROMString::from("DRILL"),
///         height: 75,
///         weight: 2650,
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokedexEntry {
    pub species: ROMString,
    pub height: u32,
    pub weight: u32,
}

impl From<&[u8]> for PokedexEntry {
    /// Convert &[u8] to PokedexEntry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![
    ///     0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
    /// ];
    /// let pokedex_entry = PokedexEntry::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokedex_entry,
    ///     PokedexEntry {
    ///         species: ROMString::from("DRILL"),
    ///         height: 75,
    ///         weight: 2650,
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let species_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let species = ROMString::new(&rom[..species_end_index]);

        let mut cursor = Cursor::new(&rom[(species_end_index + 1)..]);

        let height_ft = cursor.read_u8().unwrap_or(0) as u32;
        let height_in = cursor.read_u8().unwrap_or(0) as u32;
        let height = (height_ft * 12) + height_in;
        let weight = cursor.read_u16::<LittleEndian>().unwrap_or(0) as u32;

        PokedexEntry {
            species,
            height,
            weight,
        }
    }
}

impl PokedexEntry {
    /// Pokédex entry to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let pokedex_entry = PokedexEntry {
    ///     species: ROMString::from("DRILL"),
    ///     height: 75,
    ///     weight: 2650,
    /// };
    ///
    /// let raw = pokedex_entry.to_raw();
    ///
    /// assert_eq!(
    ///     raw,
    ///     vec![0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A]
    /// );
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        let species_string = ROMString::from(self.species.to_string());
        let height_ft = ((self.height as f32) / 12.0) as u32;
        let height_in = (self.height - (height_ft * 12)) as u32;

        [
            species_string.value,
            vec![0x50],
            vec![height_ft as u8, height_in as u8],
            (self.weight as u16).to_le_bytes().to_vec(),
        ]
        .concat()
    }
}

/// Pokédex entry text
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F];
/// let pokedex_text = PokedexText::from(&rom[..]);
///
/// assert_eq!(
///     pokedex_text,
///     PokedexText {
///         text: ROMString::from("DRILL"),
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokedexText {
    pub text: ROMString,
}

impl From<&[u8]> for PokedexText {
    /// Convert &[u8] to PokedexText
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F];
    /// let pokedex_entry = PokedexText::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokedex_entry,
    ///     PokedexText {
    ///         text: ROMString::from("DRILL"),
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let text_end_index = rom.iter().position(|&r| r == 0x5F).unwrap_or(rom.len());

        let text = ROMString::new(&rom[1..text_end_index]);

        PokedexText { text }
    }
}

impl PokedexText {
    /// Pokédex entry text to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let pokedex_text = PokedexText {
    ///     text: ROMString::from("DRILL"),
    /// };
    ///
    /// let raw = pokedex_text.to_raw();
    ///
    /// assert_eq!(raw, vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        [vec![0x00], self.text.value.to_vec(), vec![0x5F]].concat()
    }
}

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
    /// use pkmnapi_db::types::*;
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

/// Trainer name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let trainer_name = TrainerName::from(&rom[..]);
///
/// assert_eq!(
///     trainer_name,
///     TrainerName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct TrainerName {
    pub name: ROMString,
}

impl From<&[u8]> for TrainerName {
    /// Convert &[u8] to TrainerName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let trainer_name = TrainerName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     trainer_name,
    ///     TrainerName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        TrainerName { name }
    }
}

impl TrainerName {
    /// Trainer name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let trainer_name = TrainerName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = trainer_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

/// Item name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let item_name = ItemName::from(&rom[..]);
///
/// assert_eq!(
///     item_name,
///     ItemName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct ItemName {
    pub name: ROMString,
}

impl From<&[u8]> for ItemName {
    /// Convert &[u8] to ItemName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let item_name = ItemName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     item_name,
    ///     ItemName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = ROMString::new(&rom[..name_end_index]);

        ItemName { name }
    }
}

impl ItemName {
    /// Item name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let item_name = ItemName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = item_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

/// Save player name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let sav = vec![0x80, 0x81, 0x82, 0x50];
/// let save_player_name = SavePlayerName::from(&sav[..]);
///
/// assert_eq!(
///     save_player_name,
///     SavePlayerName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct SavePlayerName {
    pub name: ROMString,
}

impl From<&[u8]> for SavePlayerName {
    /// Convert &[u8] to SavePlayerName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let sav = vec![0x80, 0x81, 0x82, 0x50];
    /// let save_player_name = SavePlayerName::from(&sav[..]);
    ///
    /// assert_eq!(
    ///     save_player_name,
    ///     SavePlayerName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(sav: &[u8]) -> Self {
        let name_end_index = sav.iter().position(|&r| r == 0x50).unwrap_or(sav.len());

        let name = ROMString::new(&sav[..name_end_index]);

        SavePlayerName { name }
    }
}

impl SavePlayerName {
    /// Save player name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let save_player_name = SavePlayerName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = save_player_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

/// Save item
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let sav = vec![0x01, 0x02];
/// let save_item = SaveItem::from(&sav[..]);
///
/// assert_eq!(
///     save_item,
///     SaveItem {
///         item_id: 0x01,
///         amount: 0x02
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct SaveItem {
    pub item_id: u8,
    pub amount: u8,
}

impl From<&[u8]> for SaveItem {
    /// Convert &[u8] to SaveItem
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let sav = vec![0x01, 0x02];
    /// let save_item = SaveItem::from(&sav[..]);
    ///
    /// assert_eq!(
    ///     save_item,
    ///     SaveItem {
    ///         item_id: 0x01,
    ///         amount: 0x02
    ///     }
    /// );
    /// ```
    fn from(sav: &[u8]) -> Self {
        let item_id = sav[0];
        let amount = sav[1];

        SaveItem { item_id, amount }
    }
}

impl SaveItem {
    /// Save item to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let save_item = SaveItem {
    ///     item_id: 0x01,
    ///     amount: 0x02,
    /// };
    ///
    /// let raw = save_item.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x02]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.item_id, self.amount]
    }
}

/// Save rival name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::types::*;
///
/// let sav = vec![0x80, 0x81, 0x82, 0x50];
/// let save_rival_name = SaveRivalName::from(&sav[..]);
///
/// assert_eq!(
///     save_rival_name,
///     SaveRivalName {
///         name: ROMString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct SaveRivalName {
    pub name: ROMString,
}

impl From<&[u8]> for SaveRivalName {
    /// Convert &[u8] to SaveRivalName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let sav = vec![0x80, 0x81, 0x82, 0x50];
    /// let save_rival_name = SaveRivalName::from(&sav[..]);
    ///
    /// assert_eq!(
    ///     save_rival_name,
    ///     SaveRivalName {
    ///         name: ROMString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(sav: &[u8]) -> Self {
        let name_end_index = sav.iter().position(|&r| r == 0x50).unwrap_or(sav.len());

        let name = ROMString::new(&sav[..name_end_index]);

        SaveRivalName { name }
    }
}

impl SaveRivalName {
    /// Save rival name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let save_rival_name = SaveRivalName {
    ///     name: ROMString::from("ABC"),
    /// };
    ///
    /// let raw = save_rival_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }
}

#[derive(Debug, PartialEq)]
pub enum SaveTextSpeed {
    SLOW,
    MEDIUM,
    FAST,
}

impl From<&u8> for SaveTextSpeed {
    /// Convert &u8 to SaveTextSpeed
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let save_text_speed = SaveTextSpeed::from(&0x01);
    ///
    /// assert_eq!(save_text_speed, SaveTextSpeed::FAST);
    ///
    /// let save_text_speed = SaveTextSpeed::from(&0x03);
    ///
    /// assert_eq!(save_text_speed, SaveTextSpeed::MEDIUM);
    ///
    /// let save_text_speed = SaveTextSpeed::from(&0x05);
    ///
    /// assert_eq!(save_text_speed, SaveTextSpeed::SLOW);
    /// ```
    fn from(save_text_speed: &u8) -> Self {
        match save_text_speed {
            0x01 => SaveTextSpeed::FAST,
            0x05 => SaveTextSpeed::SLOW,
            _ => SaveTextSpeed::MEDIUM,
        }
    }
}

impl SaveTextSpeed {
    pub fn value(&self) -> u8 {
        match self {
            SaveTextSpeed::FAST => 0x01,
            SaveTextSpeed::MEDIUM => 0x03,
            SaveTextSpeed::SLOW => 0x05,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SaveBattleAnimation {
    ON,
    OFF,
}

impl From<&u8> for SaveBattleAnimation {
    /// Convert &u8 to SaveBattleAnimation
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let save_battle_animation = SaveBattleAnimation::from(&0x00);
    ///
    /// assert_eq!(save_battle_animation, SaveBattleAnimation::ON);
    ///
    /// let save_battle_animation = SaveBattleAnimation::from(&0x01);
    ///
    /// assert_eq!(save_battle_animation, SaveBattleAnimation::OFF);
    /// ```
    fn from(save_battle_animation: &u8) -> Self {
        match save_battle_animation {
            0x01 => SaveBattleAnimation::OFF,
            _ => SaveBattleAnimation::ON,
        }
    }
}

impl SaveBattleAnimation {
    pub fn value(&self) -> u8 {
        match self {
            SaveBattleAnimation::ON => 0x00,
            SaveBattleAnimation::OFF => 0x01,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum SaveBattleStyle {
    SHIFT,
    SET,
}

impl From<&u8> for SaveBattleStyle {
    /// Convert &u8 to SaveBattleStyle
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let save_battle_style = SaveBattleStyle::from(&0x00);
    ///
    /// assert_eq!(save_battle_style, SaveBattleStyle::SHIFT);
    ///
    /// let save_battle_style = SaveBattleStyle::from(&0x01);
    ///
    /// assert_eq!(save_battle_style, SaveBattleStyle::SET);
    /// ```
    fn from(save_battle_style: &u8) -> Self {
        match save_battle_style {
            0x01 => SaveBattleStyle::SET,
            _ => SaveBattleStyle::SHIFT,
        }
    }
}

impl SaveBattleStyle {
    pub fn value(&self) -> u8 {
        match self {
            SaveBattleStyle::SHIFT => 0x00,
            SaveBattleStyle::SET => 0x01,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SaveOptions {
    pub text_speed: SaveTextSpeed,
    pub battle_animation: SaveBattleAnimation,
    pub battle_style: SaveBattleStyle,
}

impl From<&u8> for SaveOptions {
    fn from(sav: &u8) -> Self {
        SaveOptions {
            text_speed: SaveTextSpeed::from(&(sav & 0x0F)),
            battle_animation: SaveBattleAnimation::from(&((sav & 0x40) >> 6)),
            battle_style: SaveBattleStyle::from(&((sav & 0x80) >> 7)),
        }
    }
}

impl SaveOptions {
    pub fn to_raw(&self) -> Vec<u8> {
        let value = {
            self.text_speed.value()
                | (self.battle_animation.value() << 6)
                | (self.battle_style.value() << 7)
        };

        vec![value]
    }
}

#[derive(Debug, PartialEq)]
pub struct Party {
    pub level_type: PartyLevelType,
    pub pokemon: Vec<PartyPokemon>,
}

impl From<&[u8]> for Party {
    fn from(party: &[u8]) -> Self {
        let level_type = match party.iter().next() {
            Some(level) if *level != 0xFF => PartyLevelType::SAME(*level),
            _ => PartyLevelType::DIFFERENT,
        };

        let pokemon: Vec<PartyPokemon> = match level_type {
            PartyLevelType::SAME(level) => party[1..(party.len() - 1)]
                .iter()
                .take_while(|&x| x != &0x00)
                .map(|internal_id| PartyPokemon {
                    level,
                    internal_id: *internal_id - 1,
                    pokedex_id: 0,
                })
                .collect(),
            PartyLevelType::DIFFERENT => party[1..(party.len() - 1)]
                .chunks(2)
                .take_while(|&chunk| chunk[0] != 0x00)
                .map(|chunk| PartyPokemon {
                    level: chunk[0],
                    internal_id: chunk[1] - 1,
                    pokedex_id: 0,
                })
                .collect(),
        };

        Party {
            level_type,
            pokemon,
        }
    }
}

impl Party {
    pub fn new(party_pokemon: &Vec<PartyPokemon>) -> Self {
        let levels: Vec<u8> = party_pokemon.iter().map(|pokemon| pokemon.level).collect();
        let level_min = levels.iter().min().unwrap();
        let level_max = levels.iter().max().unwrap();
        let level_type = if level_min == level_max {
            PartyLevelType::SAME(*level_min)
        } else {
            PartyLevelType::DIFFERENT
        };

        Party {
            level_type,
            pokemon: party_pokemon.to_vec(),
        }
    }

    pub fn to_raw(&self) -> Vec<u8> {
        match self.level_type {
            PartyLevelType::SAME(level) => [
                vec![level],
                self.pokemon
                    .iter()
                    .map(|pokemon| pokemon.internal_id)
                    .collect(),
                vec![0x00],
            ]
            .concat(),
            PartyLevelType::DIFFERENT => [
                vec![0xFF],
                self.pokemon
                    .iter()
                    .map(|pokemon| vec![pokemon.internal_id, pokemon.level])
                    .flatten()
                    .collect(),
                vec![0x00],
            ]
            .concat(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PartyPokemon {
    pub level: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl PartyPokemon {
    pub fn new(level: u8, pokedex_id: u8) -> Self {
        PartyPokemon {
            level,
            pokedex_id,
            internal_id: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PartyLevelType {
    SAME(u8),
    DIFFERENT,
}

#[derive(Debug, PartialEq)]
pub enum PokemonEvolution {
    LEVEL(PokemonEvolutionLevel),
    ITEM(PokemonEvolutionItem),
    TRADE(PokemonEvolutionTrade),
}

impl From<&[u8]> for PokemonEvolution {
    fn from(rom: &[u8]) -> Self {
        let mut rom = rom.iter();
        let id = rom.next().unwrap();

        match id {
            0x01 => PokemonEvolution::LEVEL(PokemonEvolutionLevel {
                level: *rom.next().unwrap(),
                internal_id: *rom.next().unwrap() - 1,
                pokedex_id: 0x00,
            }),
            0x02 => PokemonEvolution::ITEM(PokemonEvolutionItem {
                item_id: *rom.next().unwrap(),
                internal_id: *rom.skip(1).next().unwrap() - 1,
                pokedex_id: 0x00,
            }),
            0x03 => PokemonEvolution::TRADE(PokemonEvolutionTrade {
                internal_id: *rom.skip(1).next().unwrap() - 1,
                pokedex_id: 0x00,
            }),
            _ => unreachable!(),
        }
    }
}

impl PokemonEvolution {
    pub fn to_raw(&self) -> Vec<u8> {
        match self {
            PokemonEvolution::LEVEL(evolution) => {
                vec![0x01, evolution.level, evolution.internal_id + 1]
            }
            PokemonEvolution::ITEM(evolution) => {
                vec![0x02, evolution.item_id, 0x01, evolution.internal_id + 1]
            }
            PokemonEvolution::TRADE(evolution) => vec![0x03, 0x01, evolution.internal_id + 1],
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PokemonEvolutionLevel {
    pub level: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl PokemonEvolutionLevel {
    pub fn new(pokedex_id: u8, level: u8) -> PokemonEvolution {
        PokemonEvolution::LEVEL(PokemonEvolutionLevel {
            level,
            pokedex_id,
            internal_id: 0,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct PokemonEvolutionItem {
    pub item_id: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl PokemonEvolutionItem {
    pub fn new(pokedex_id: u8, item_id: u8) -> PokemonEvolution {
        PokemonEvolution::ITEM(PokemonEvolutionItem {
            item_id,
            pokedex_id,
            internal_id: 0,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct PokemonEvolutionTrade {
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl PokemonEvolutionTrade {
    pub fn new(pokedex_id: u8) -> PokemonEvolution {
        PokemonEvolution::TRADE(PokemonEvolutionTrade {
            pokedex_id,
            internal_id: 0,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct MapPokemon {
    pub grass: MapPokemonArea,
    pub water: MapPokemonArea,
}

impl From<&[u8]> for MapPokemon {
    /// Convert &[u8] to MapPokemon
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![
    ///     0x0A, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
    ///     0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00,
    /// ];
    /// let map_pokemon = MapPokemon::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     map_pokemon,
    ///     MapPokemon {
    ///         grass: MapPokemonArea {
    ///             encounter_rate: 10,
    ///             pokemon: vec![MapPokemonInfo::new(10, 0); 10],
    ///         },
    ///         water: MapPokemonArea {
    ///             encounter_rate: 0,
    ///             pokemon: vec![]
    ///         }
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let grass_offset = 0;
        let grass_encounter_rate = rom[grass_offset];
        let grass_pokemon = {
            let count = if grass_encounter_rate != 0x00 { 10 } else { 0 };

            rom[(grass_offset + 1)..]
                .chunks(2)
                .take(count)
                .map(|chunk| {
                    let level = chunk[0];
                    let internal_id = chunk[1] - 1;

                    MapPokemonInfo {
                        level,
                        internal_id,
                        pokedex_id: 0,
                    }
                })
                .collect()
        };
        let water_offset = if grass_encounter_rate == 0x00 {
            grass_offset + 1
        } else {
            grass_offset + 21
        };
        let water_encounter_rate = rom[water_offset];
        let water_pokemon = {
            let count = if water_encounter_rate != 0x00 { 10 } else { 0 };

            rom[(water_offset + 1)..]
                .chunks(2)
                .take(count)
                .map(|chunk| {
                    let level = chunk[0];
                    let internal_id = chunk[1] - 1;

                    MapPokemonInfo {
                        level,
                        internal_id,
                        pokedex_id: 0,
                    }
                })
                .collect()
        };

        MapPokemon {
            grass: MapPokemonArea {
                encounter_rate: grass_encounter_rate,
                pokemon: grass_pokemon,
            },
            water: MapPokemonArea {
                encounter_rate: water_encounter_rate,
                pokemon: water_pokemon,
            },
        }
    }
}

impl MapPokemon {
    pub fn to_raw(&self) -> Vec<u8> {
        let grass_encounter_rate = {
            if self.grass.pokemon.len() == 0 {
                vec![0]
            } else {
                vec![self.grass.encounter_rate]
            }
        };
        let grass_pokemon = self
            .grass
            .pokemon
            .iter()
            .map(|pokemon| vec![pokemon.level, pokemon.internal_id + 1])
            .flatten()
            .collect();
        let water_encounter_rate = {
            if self.water.pokemon.len() == 0 {
                vec![0]
            } else {
                vec![self.water.encounter_rate]
            }
        };
        let water_pokemon = self
            .water
            .pokemon
            .iter()
            .map(|pokemon| vec![pokemon.level, pokemon.internal_id + 1])
            .flatten()
            .collect();

        [
            grass_encounter_rate,
            grass_pokemon,
            water_encounter_rate,
            water_pokemon,
        ]
        .concat()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapPokemonArea {
    pub encounter_rate: u8,
    pub pokemon: Vec<MapPokemonInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapPokemonInfo {
    pub level: u8,
    pub pokedex_id: u8,
    pub(crate) internal_id: u8,
}

impl MapPokemonInfo {
    pub fn new(level: u8, pokedex_id: u8) -> Self {
        MapPokemonInfo {
            level,
            pokedex_id,
            internal_id: 0,
        }
    }
}
