//! Pkmnapi types module
//!
//! # Example
//!
//! ```
//! use pkmnapi::db::types::*;
//!
//! let pokedex_id = PkmnapiDBPokedexID::from(151);
//!
//! assert_eq!(pokedex_id, 151);
//! ```

use crate::db::string::*;
use byteorder::{LittleEndian, ReadBytesExt};
use std::cmp::{self, Ordering};
use std::fmt;
use std::io::{Cursor, Read};
use std::ops::{Add, Mul, Sub};

/// Pokédex ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let pokedex_id = PkmnapiDBPokedexID::from(151);
///
/// assert_eq!(pokedex_id, 151);
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBPokedexID(u8);

impl PkmnapiDBPokedexID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PkmnapiDBPokedexID {
    fn from(pokedex_id: u8) -> Self {
        PkmnapiDBPokedexID(pokedex_id)
    }
}

impl PartialEq<u8> for PkmnapiDBPokedexID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBPokedexID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for PkmnapiDBPokedexID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for PkmnapiDBPokedexID {
    type Output = usize;

    fn sub(self, other: usize) -> usize {
        self.0 as usize - other
    }
}

/// Internal ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let internal_id = PkmnapiDBInternalID::from(0x14);
///
/// assert_eq!(internal_id, 0x14);
/// ```
#[derive(Debug)]
pub struct PkmnapiDBInternalID(u8);

impl PkmnapiDBInternalID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PkmnapiDBInternalID {
    fn from(internal_id: u8) -> Self {
        PkmnapiDBInternalID(internal_id)
    }
}

impl fmt::Display for PkmnapiDBInternalID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u8> for PkmnapiDBInternalID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBInternalID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl Add<usize> for PkmnapiDBInternalID {
    type Output = usize;

    fn add(self, other: usize) -> usize {
        self.0 as usize + other
    }
}

impl Mul<usize> for PkmnapiDBInternalID {
    type Output = usize;

    fn mul(self, other: usize) -> usize {
        self.0 as usize * other
    }
}

/// Type ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let type_id = PkmnapiDBTypeID::from(0x12);
///
/// assert_eq!(type_id, 0x12);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct PkmnapiDBTypeID(u8);

impl PkmnapiDBTypeID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PkmnapiDBTypeID {
    fn from(type_id: u8) -> Self {
        PkmnapiDBTypeID(type_id)
    }
}

impl fmt::Display for PkmnapiDBTypeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u8> for PkmnapiDBTypeID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl Mul<usize> for PkmnapiDBTypeID {
    type Output = usize;

    fn mul(self, other: usize) -> usize {
        self.0 as usize * other
    }
}

/// Type effect ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let type_effect_id = PkmnapiDBTypeEffectID::from(0x12);
///
/// assert_eq!(type_effect_id, 0x12);
/// ```
#[derive(Debug)]
pub struct PkmnapiDBTypeEffectID(u8);

impl PkmnapiDBTypeEffectID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PkmnapiDBTypeEffectID {
    fn from(type_effect_id: u8) -> Self {
        PkmnapiDBTypeEffectID(type_effect_id)
    }
}

impl PartialEq<u8> for PkmnapiDBTypeEffectID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBTypeEffectID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl Mul<usize> for PkmnapiDBTypeEffectID {
    type Output = usize;

    fn mul(self, other: usize) -> usize {
        self.0 as usize * other
    }
}

/// Type name
///
/// # Example
///
/// ```
/// use pkmnapi::db::string::*;
/// use pkmnapi::db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let type_name = PkmnapiDBTypeName::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     PkmnapiDBTypeName {
///         name: PkmnapiDBString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTypeName {
    pub name: PkmnapiDBString,
}

impl From<&[u8]> for PkmnapiDBTypeName {
    /// Convert &[u8] to PkmnapiDBTypeName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let type_name = PkmnapiDBTypeName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     type_name,
    ///     PkmnapiDBTypeName {
    ///         name: PkmnapiDBString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = PkmnapiDBString::new(&rom[..name_end_index]);

        PkmnapiDBTypeName { name }
    }
}

impl PkmnapiDBTypeName {
    /// Type name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let type_name = PkmnapiDBTypeName {
    ///     name: PkmnapiDBString::from("ABC"),
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
/// use pkmnapi::db::types::*;
///
/// let rom = vec![0x01, 0x02, 0x14];
/// let type_effect = PkmnapiDBTypeEffect::from(&rom[..]);
///
/// assert_eq!(
///     type_effect,
///     PkmnapiDBTypeEffect {
///         attacking_type_id: PkmnapiDBTypeID::from(0x01),
///         defending_type_id: PkmnapiDBTypeID::from(0x02),
///         multiplier: 2.0
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTypeEffect {
    pub attacking_type_id: PkmnapiDBTypeID,
    pub defending_type_id: PkmnapiDBTypeID,
    pub multiplier: f32,
}

impl From<&[u8]> for PkmnapiDBTypeEffect {
    /// Convert &[u8] to PkmnapiDBTypeEffect
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x14];
    /// let type_effect = PkmnapiDBTypeEffect::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     PkmnapiDBTypeEffect {
    ///         attacking_type_id: PkmnapiDBTypeID::from(0x01),
    ///         defending_type_id: PkmnapiDBTypeID::from(0x02),
    ///         multiplier: 2.0
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let attacking_type_id = PkmnapiDBTypeID::from(cursor.read_u8().unwrap_or(0));
        let defending_type_id = PkmnapiDBTypeID::from(cursor.read_u8().unwrap_or(0));
        let multiplier = (cursor.read_u8().unwrap_or(0) as f32) / 10.0;

        PkmnapiDBTypeEffect {
            attacking_type_id,
            defending_type_id,
            multiplier,
        }
    }
}

impl PkmnapiDBTypeEffect {
    /// Convert type effect to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let type_effect = PkmnapiDBTypeEffect {
    ///     attacking_type_id: PkmnapiDBTypeID::from(0x01),
    ///     defending_type_id: PkmnapiDBTypeID::from(0x02),
    ///     multiplier: 2.0,
    /// };
    ///
    /// let raw = type_effect.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x02, 0x14]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![
            self.attacking_type_id.value(),
            self.defending_type_id.value(),
            cmp::min((self.multiplier * 10.0) as u8, 254),
        ]
    }
}

/// Pokemon stats (HP, attack, defence, etc)
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
/// let stats = PkmnapiDBStats::from(&rom[..]);
///
/// assert_eq!(
///     stats,
///     PkmnapiDBStats {
///         pokedex_id: PkmnapiDBPokedexID::from(0x01),
///         base_hp: 0x02,
///         base_attack: 0x03,
///         base_defence: 0x04,
///         base_speed: 0x05,
///         base_special: 0x06,
///         type_ids: vec![PkmnapiDBTypeID::from(0x07), PkmnapiDBTypeID::from(0x08)],
///         catch_rate: 0x09,
///         base_exp_yield: 0x0A
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBStats {
    pub pokedex_id: PkmnapiDBPokedexID,
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub type_ids: Vec<PkmnapiDBTypeID>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}

impl From<&[u8]> for PkmnapiDBStats {
    /// Convert &[u8] to PkmnapiDBStats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
    /// let stats = PkmnapiDBStats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     stats,
    ///     PkmnapiDBStats {
    ///         pokedex_id: PkmnapiDBPokedexID::from(0x01),
    ///         base_hp: 0x02,
    ///         base_attack: 0x03,
    ///         base_defence: 0x04,
    ///         base_speed: 0x05,
    ///         base_special: 0x06,
    ///         type_ids: vec![PkmnapiDBTypeID::from(0x07), PkmnapiDBTypeID::from(0x08)],
    ///         catch_rate: 0x09,
    ///         base_exp_yield: 0x0A
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let pokedex_id = PkmnapiDBPokedexID::from(cursor.read_u8().unwrap_or(0));
        let base_hp = cursor.read_u8().unwrap_or(0);
        let base_attack = cursor.read_u8().unwrap_or(0);
        let base_defence = cursor.read_u8().unwrap_or(0);
        let base_speed = cursor.read_u8().unwrap_or(0);
        let base_special = cursor.read_u8().unwrap_or(0);
        let type_ids = {
            let mut type_ids = vec![0x00; 2];

            cursor.read_exact(&mut type_ids).unwrap();

            type_ids
                .iter()
                .map(|type_id| PkmnapiDBTypeID::from(*type_id))
                .collect()
        };
        let catch_rate = cursor.read_u8().unwrap_or(0);
        let base_exp_yield = cursor.read_u8().unwrap_or(0);

        PkmnapiDBStats {
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

impl PkmnapiDBStats {
    /// Convert stats to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let stats = PkmnapiDBStats {
    ///     pokedex_id: PkmnapiDBPokedexID::from(0x01),
    ///     base_hp: 0x02,
    ///     base_attack: 0x03,
    ///     base_defence: 0x04,
    ///     base_speed: 0x05,
    ///     base_special: 0x06,
    ///     type_ids: vec![PkmnapiDBTypeID::from(0x07), PkmnapiDBTypeID::from(0x08)],
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
                self.pokedex_id.value(),
                self.base_hp,
                self.base_attack,
                self.base_defence,
                self.base_speed,
                self.base_special,
            ],
            self.type_ids
                .iter()
                .map(|type_id| type_id.value())
                .collect::<Vec<u8>>(),
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
/// use pkmnapi::db::string::*;
/// use pkmnapi::db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let pokemon_name = PkmnapiDBPokemonName::from(&rom[..]);
///
/// assert_eq!(
///     pokemon_name,
///     PkmnapiDBPokemonName {
///         name: PkmnapiDBString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBPokemonName {
    pub name: PkmnapiDBString,
}

impl From<&[u8]> for PkmnapiDBPokemonName {
    /// Convert &[u8] to PkmnapiDBPokemonName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let pokemon_name = PkmnapiDBPokemonName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokemon_name,
    ///     PkmnapiDBPokemonName {
    ///         name: PkmnapiDBString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = PkmnapiDBString::new(&rom[..name_end_index]);

        PkmnapiDBPokemonName { name }
    }
}

impl PkmnapiDBPokemonName {
    /// Pokémon name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let pokemon_name = PkmnapiDBPokemonName {
    ///     name: PkmnapiDBString::from("ABC"),
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

/// Move ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let move_id = PkmnapiDBMoveID::from(0x12);
///
/// assert_eq!(move_id, 0x12);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct PkmnapiDBMoveID(u8);

impl PkmnapiDBMoveID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PkmnapiDBMoveID {
    fn from(move_id: u8) -> Self {
        PkmnapiDBMoveID(move_id)
    }
}

impl PartialEq<u8> for PkmnapiDBMoveID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBMoveID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for PkmnapiDBMoveID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for PkmnapiDBMoveID {
    type Output = usize;

    fn sub(self, other: usize) -> usize {
        self.0 as usize - other
    }
}

/// Move stats
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let rom = vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23];
/// let type_name = PkmnapiDBMoveStats::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     PkmnapiDBMoveStats {
///         move_id: PkmnapiDBMoveID::from(0x01),
///         effect: 0x00,
///         power: 0x28,
///         type_id: PkmnapiDBTypeID::from(0x00),
///         accuracy: 1.0,
///         pp: 0x23
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBMoveStats {
    pub move_id: PkmnapiDBMoveID,
    pub effect: u8,
    pub power: u8,
    pub type_id: PkmnapiDBTypeID,
    pub accuracy: f32,
    pub pp: u8,
}

impl From<&[u8]> for PkmnapiDBMoveStats {
    /// Convert &[u8] to PkmnapiDBMoveStats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23];
    /// let move_stats = PkmnapiDBMoveStats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     move_stats,
    ///     PkmnapiDBMoveStats {
    ///         move_id: PkmnapiDBMoveID::from(0x01),
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: PkmnapiDBTypeID::from(0x00),
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let move_id = PkmnapiDBMoveID::from(cursor.read_u8().unwrap_or(0));
        let effect = cursor.read_u8().unwrap_or(0);
        let power = cursor.read_u8().unwrap_or(0);
        let type_id = PkmnapiDBTypeID::from(cursor.read_u8().unwrap_or(0));
        let accuracy = (cursor.read_u8().unwrap_or(0) as f32) / 255.0;
        let pp = cursor.read_u8().unwrap_or(0);

        PkmnapiDBMoveStats {
            move_id,
            effect,
            power,
            type_id,
            accuracy,
            pp,
        }
    }
}

impl PkmnapiDBMoveStats {
    /// Pokémon name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let move_stats = PkmnapiDBMoveStats {
    ///     move_id: PkmnapiDBMoveID::from(0x01),
    ///     effect: 0x00,
    ///     power: 0x28,
    ///     type_id: PkmnapiDBTypeID::from(0x00),
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
            self.move_id.value(),
            self.effect,
            self.power,
            self.type_id.value(),
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
/// use pkmnapi::db::string::*;
/// use pkmnapi::db::types::*;
///
/// let rom = vec![0x80, 0x81, 0x82, 0x50];
/// let type_name = PkmnapiDBMoveName::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     PkmnapiDBMoveName {
///         name: PkmnapiDBString::from("ABC")
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBMoveName {
    pub name: PkmnapiDBString,
}

impl From<&[u8]> for PkmnapiDBMoveName {
    /// Convert &[u8] to PkmnapiDBMoveName
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![0x80, 0x81, 0x82, 0x50];
    /// let move_name = PkmnapiDBMoveName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     move_name,
    ///     PkmnapiDBMoveName {
    ///         name: PkmnapiDBString::from("ABC")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let name = PkmnapiDBString::new(&rom[..name_end_index]);

        PkmnapiDBMoveName { name }
    }
}

impl PkmnapiDBMoveName {
    /// Move name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let move_name = PkmnapiDBMoveName {
    ///     name: PkmnapiDBString::from("ABC"),
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

/// HM ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let hm_id = PkmnapiDBHMID::from(0x12);
///
/// assert_eq!(hm_id, 0x12);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct PkmnapiDBHMID(u8);

impl From<u8> for PkmnapiDBHMID {
    fn from(move_id: u8) -> Self {
        PkmnapiDBHMID(move_id)
    }
}

impl PartialEq<u8> for PkmnapiDBHMID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBHMID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for PkmnapiDBHMID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for PkmnapiDBHMID {
    type Output = usize;

    fn sub(self, other: usize) -> usize {
        self.0 as usize - other
    }
}

/// HM
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let hm = PkmnapiDBHM::from(0x01);
///
/// assert_eq!(
///     hm,
///     PkmnapiDBHM {
///         move_id: PkmnapiDBMoveID::from(0x01)
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBHM {
    pub move_id: PkmnapiDBMoveID,
}

impl From<u8> for PkmnapiDBHM {
    /// Convert u8 to PkmnapiDBHM
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let hm = PkmnapiDBHM::from(0x01);
    ///
    /// assert_eq!(
    ///     hm,
    ///     PkmnapiDBHM {
    ///         move_id: PkmnapiDBMoveID::from(0x01)
    ///     }
    /// );
    /// ```
    fn from(move_id: u8) -> Self {
        PkmnapiDBHM {
            move_id: PkmnapiDBMoveID::from(move_id),
        }
    }
}

impl PkmnapiDBHM {
    /// HM to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let hm = PkmnapiDBHM::from(0x01);
    ///
    /// let raw = hm.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.move_id.value()]
    }
}

/// TM ID
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let tm_id = PkmnapiDBTMID::from(0x12);
///
/// assert_eq!(tm_id, 0x12);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PkmnapiDBTMID(u8);

impl From<u8> for PkmnapiDBTMID {
    fn from(move_id: u8) -> Self {
        PkmnapiDBTMID(move_id)
    }
}

impl PartialEq<u8> for PkmnapiDBTMID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PkmnapiDBTMID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for PkmnapiDBTMID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for PkmnapiDBTMID {
    type Output = usize;

    fn sub(self, other: usize) -> usize {
        self.0 as usize - other
    }
}

/// TM
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let tm = PkmnapiDBTM::from(0x01);
///
/// assert_eq!(
///     tm,
///     PkmnapiDBTM {
///         move_id: PkmnapiDBMoveID::from(0x01)
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTM {
    pub move_id: PkmnapiDBMoveID,
}

impl From<u8> for PkmnapiDBTM {
    /// Convert u8 to PkmnapiDBTM
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let tm = PkmnapiDBTM::from(0x01);
    ///
    /// assert_eq!(
    ///     tm,
    ///     PkmnapiDBTM {
    ///         move_id: PkmnapiDBMoveID::from(0x01)
    ///     }
    /// );
    /// ```
    fn from(move_id: u8) -> Self {
        PkmnapiDBTM {
            move_id: PkmnapiDBMoveID::from(move_id),
        }
    }
}

impl PkmnapiDBTM {
    /// TM to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let tm = PkmnapiDBTM::from(0x01);
    ///
    /// let raw = tm.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.move_id.value()]
    }
}

/// TM price
///
/// # Example
///
/// ```
/// use pkmnapi::db::types::*;
///
/// let tm = PkmnapiDBTMPrice::from(0x01);
///
/// assert_eq!(tm, PkmnapiDBTMPrice { value: 1000 });
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTMPrice {
    pub value: u32,
}

impl From<u8> for PkmnapiDBTMPrice {
    /// Convert u8 to PkmnapiDBTMPrice
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let tm = PkmnapiDBTMPrice::from(0x01);
    ///
    /// assert_eq!(tm, PkmnapiDBTMPrice { value: 1000 });
    /// ```
    fn from(tm_price: u8) -> Self {
        PkmnapiDBTMPrice {
            value: (tm_price as u32) * 1000,
        }
    }
}

impl PkmnapiDBTMPrice {
    /// TM price to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::types::*;
    ///
    /// let tm_price = PkmnapiDBTMPrice::from(0x01);
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
/// use pkmnapi::db::string::*;
/// use pkmnapi::db::types::*;
///
/// let rom = vec![
///     0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
/// ];
/// let pokedex_entry = PkmnapiDBPokedexEntry::from(&rom[..]);
///
/// assert_eq!(
///     pokedex_entry,
///     PkmnapiDBPokedexEntry {
///         species: PkmnapiDBString::from("DRILL"),
///         height: 75,
///         weight: 2650,
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBPokedexEntry {
    pub species: PkmnapiDBString,
    pub height: u32,
    pub weight: u32,
}

impl From<&[u8]> for PkmnapiDBPokedexEntry {
    /// Convert &[u8] to PkmnapiDBPokedexEntry
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![
    ///     0x83, 0x91, 0x88, 0x8B, 0x8B, 0x50, 0x06, 0x03, 0x5A, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00,
    /// ];
    /// let pokedex_entry = PkmnapiDBPokedexEntry::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokedex_entry,
    ///     PkmnapiDBPokedexEntry {
    ///         species: PkmnapiDBString::from("DRILL"),
    ///         height: 75,
    ///         weight: 2650,
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let species_end_index = rom.iter().position(|&r| r == 0x50).unwrap_or(rom.len());

        let species = PkmnapiDBString::new(&rom[..species_end_index]);

        let mut cursor = Cursor::new(&rom[(species_end_index + 1)..]);

        let height_ft = cursor.read_u8().unwrap_or(0) as u32;
        let height_in = cursor.read_u8().unwrap_or(0) as u32;
        let height = (height_ft * 12) + height_in;
        let weight = cursor.read_u16::<LittleEndian>().unwrap_or(0) as u32;

        PkmnapiDBPokedexEntry {
            species,
            height,
            weight,
        }
    }
}

impl PkmnapiDBPokedexEntry {
    /// Pokédex entry to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let pokedex_entry = PkmnapiDBPokedexEntry {
    ///     species: PkmnapiDBString::from("DRILL"),
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
        let species_string = PkmnapiDBString::from(self.species.to_string());
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
/// use pkmnapi::db::string::*;
/// use pkmnapi::db::types::*;
///
/// let rom = vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F];
/// let pokedex_entry_text = PkmnapiDBPokedexEntryText::from(&rom[..]);
///
/// assert_eq!(
///     pokedex_entry_text,
///     PkmnapiDBPokedexEntryText {
///         text: PkmnapiDBString::from("DRILL"),
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PkmnapiDBPokedexEntryText {
    pub text: PkmnapiDBString,
}

impl From<&[u8]> for PkmnapiDBPokedexEntryText {
    /// Convert &[u8] to PkmnapiDBPokedexEntryText
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let rom = vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F];
    /// let pokedex_entry = PkmnapiDBPokedexEntryText::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokedex_entry,
    ///     PkmnapiDBPokedexEntryText {
    ///         text: PkmnapiDBString::from("DRILL"),
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let text_end_index = rom.iter().position(|&r| r == 0x5F).unwrap_or(rom.len());

        let text = PkmnapiDBString::new(&rom[1..text_end_index]);

        PkmnapiDBPokedexEntryText { text }
    }
}

impl PkmnapiDBPokedexEntryText {
    /// Pokédex entry text to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let pokedex_entry_text = PkmnapiDBPokedexEntryText {
    ///     text: PkmnapiDBString::from("DRILL"),
    /// };
    ///
    /// let raw = pokedex_entry_text.to_raw();
    ///
    /// assert_eq!(raw, vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        [vec![0x00], self.text.value.to_vec(), vec![0x5F]].concat()
    }
}
