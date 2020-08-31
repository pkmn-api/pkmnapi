//! Pkmnapi types module
//!
//! # Example
//!
//! ```
//! use pkmnapi_db::types::*;
//!
//! let pokedex_id = PokedexID::from(151);
//!
//! assert_eq!(pokedex_id, 151);
//! ```

use crate::string::*;
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
/// use pkmnapi_db::types::*;
///
/// let pokedex_id = PokedexID::from(151);
///
/// assert_eq!(pokedex_id, 151);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct PokedexID(u8);

impl PokedexID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for PokedexID {
    fn from(pokedex_id: u8) -> Self {
        PokedexID(pokedex_id)
    }
}

impl PartialEq<u8> for PokedexID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for PokedexID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for PokedexID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for PokedexID {
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
/// use pkmnapi_db::types::*;
///
/// let internal_id = InternalID::from(0x14);
///
/// assert_eq!(internal_id, 0x14);
/// ```
#[derive(Debug)]
pub struct InternalID(u8);

impl InternalID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for InternalID {
    fn from(internal_id: u8) -> Self {
        InternalID(internal_id)
    }
}

impl fmt::Display for InternalID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u8> for InternalID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for InternalID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl Add<usize> for InternalID {
    type Output = usize;

    fn add(self, other: usize) -> usize {
        self.0 as usize + other
    }
}

impl Mul<usize> for InternalID {
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
/// use pkmnapi_db::types::*;
///
/// let type_id = TypeID::from(0x12);
///
/// assert_eq!(type_id, 0x12);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TypeID(u8);

impl TypeID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for TypeID {
    fn from(type_id: u8) -> Self {
        TypeID(type_id)
    }
}

impl fmt::Display for TypeID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<u8> for TypeID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl Mul<usize> for TypeID {
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
/// use pkmnapi_db::types::*;
///
/// let type_effect_id = TypeEffectID::from(0x12);
///
/// assert_eq!(type_effect_id, 0x12);
/// ```
#[derive(Debug)]
pub struct TypeEffectID(u8);

impl TypeEffectID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for TypeEffectID {
    fn from(type_effect_id: u8) -> Self {
        TypeEffectID(type_effect_id)
    }
}

impl PartialEq<u8> for TypeEffectID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for TypeEffectID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl Mul<usize> for TypeEffectID {
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
///         attacking_type_id: TypeID::from(0x01),
///         defending_type_id: TypeID::from(0x02),
///         multiplier: 2.0
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct TypeEffect {
    pub attacking_type_id: TypeID,
    pub defending_type_id: TypeID,
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
    ///         attacking_type_id: TypeID::from(0x01),
    ///         defending_type_id: TypeID::from(0x02),
    ///         multiplier: 2.0
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let attacking_type_id = TypeID::from(cursor.read_u8().unwrap_or(0));
        let defending_type_id = TypeID::from(cursor.read_u8().unwrap_or(0));
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
    ///     attacking_type_id: TypeID::from(0x01),
    ///     defending_type_id: TypeID::from(0x02),
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
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
/// let stats = Stats::from(&rom[..]);
///
/// assert_eq!(
///     stats,
///     Stats {
///         pokedex_id: PokedexID::from(0x01),
///         base_hp: 0x02,
///         base_attack: 0x03,
///         base_defence: 0x04,
///         base_speed: 0x05,
///         base_special: 0x06,
///         type_ids: vec![TypeID::from(0x07), TypeID::from(0x08)],
///         catch_rate: 0x09,
///         base_exp_yield: 0x0A
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct Stats {
    pub pokedex_id: PokedexID,
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub type_ids: Vec<TypeID>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}

impl From<&[u8]> for Stats {
    /// Convert &[u8] to Stats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A];
    /// let stats = Stats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     stats,
    ///     Stats {
    ///         pokedex_id: PokedexID::from(0x01),
    ///         base_hp: 0x02,
    ///         base_attack: 0x03,
    ///         base_defence: 0x04,
    ///         base_speed: 0x05,
    ///         base_special: 0x06,
    ///         type_ids: vec![TypeID::from(0x07), TypeID::from(0x08)],
    ///         catch_rate: 0x09,
    ///         base_exp_yield: 0x0A
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let pokedex_id = PokedexID::from(cursor.read_u8().unwrap_or(0));
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
                .map(|type_id| TypeID::from(*type_id))
                .collect()
        };
        let catch_rate = cursor.read_u8().unwrap_or(0);
        let base_exp_yield = cursor.read_u8().unwrap_or(0);

        Stats {
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

impl Stats {
    /// Convert stats to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let stats = Stats {
    ///     pokedex_id: PokedexID::from(0x01),
    ///     base_hp: 0x02,
    ///     base_attack: 0x03,
    ///     base_defence: 0x04,
    ///     base_speed: 0x05,
    ///     base_special: 0x06,
    ///     type_ids: vec![TypeID::from(0x07), TypeID::from(0x08)],
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

/// Move ID
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let move_id = MoveID::from(0x12);
///
/// assert_eq!(move_id, 0x12);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct MoveID(u8);

impl MoveID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for MoveID {
    fn from(move_id: u8) -> Self {
        MoveID(move_id)
    }
}

impl PartialEq<u8> for MoveID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for MoveID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for MoveID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for MoveID {
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
/// use pkmnapi_db::types::*;
///
/// let rom = vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23];
/// let type_name = MoveStats::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     MoveStats {
///         move_id: MoveID::from(0x01),
///         effect: 0x00,
///         power: 0x28,
///         type_id: TypeID::from(0x00),
///         accuracy: 1.0,
///         pp: 0x23
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct MoveStats {
    pub move_id: MoveID,
    pub effect: u8,
    pub power: u8,
    pub type_id: TypeID,
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
    ///         move_id: MoveID::from(0x01),
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: TypeID::from(0x00),
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let move_id = MoveID::from(cursor.read_u8().unwrap_or(0));
        let effect = cursor.read_u8().unwrap_or(0);
        let power = cursor.read_u8().unwrap_or(0);
        let type_id = TypeID::from(cursor.read_u8().unwrap_or(0));
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
    ///     move_id: MoveID::from(0x01),
    ///     effect: 0x00,
    ///     power: 0x28,
    ///     type_id: TypeID::from(0x00),
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

/// HM ID
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let hm_id = HMID::from(0x12);
///
/// assert_eq!(hm_id, 0x12);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct HMID(u8);

impl From<u8> for HMID {
    fn from(move_id: u8) -> Self {
        HMID(move_id)
    }
}

impl PartialEq<u8> for HMID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for HMID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for HMID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for HMID {
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
/// use pkmnapi_db::types::*;
///
/// let hm = HM::from(0x01);
///
/// assert_eq!(
///     hm,
///     HM {
///         move_id: MoveID::from(0x01)
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct HM {
    pub move_id: MoveID,
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
    /// assert_eq!(
    ///     hm,
    ///     HM {
    ///         move_id: MoveID::from(0x01)
    ///     }
    /// );
    /// ```
    fn from(move_id: u8) -> Self {
        HM {
            move_id: MoveID::from(move_id),
        }
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
        vec![self.move_id.value()]
    }
}

/// TM ID
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let tm_id = TMID::from(0x12);
///
/// assert_eq!(tm_id, 0x12);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TMID(u8);

impl From<u8> for TMID {
    fn from(move_id: u8) -> Self {
        TMID(move_id)
    }
}

impl PartialEq<u8> for TMID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for TMID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for TMID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for TMID {
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
/// use pkmnapi_db::types::*;
///
/// let tm = TM::from(0x01);
///
/// assert_eq!(
///     tm,
///     TM {
///         move_id: MoveID::from(0x01)
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct TM {
    pub move_id: MoveID,
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
    /// assert_eq!(
    ///     tm,
    ///     TM {
    ///         move_id: MoveID::from(0x01)
    ///     }
    /// );
    /// ```
    fn from(move_id: u8) -> Self {
        TM {
            move_id: MoveID::from(move_id),
        }
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
        vec![self.move_id.value()]
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
/// let pokedex_entry_text = PokedexEntryText::from(&rom[..]);
///
/// assert_eq!(
///     pokedex_entry_text,
///     PokedexEntryText {
///         text: ROMString::from("DRILL"),
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct PokedexEntryText {
    pub text: ROMString,
}

impl From<&[u8]> for PokedexEntryText {
    /// Convert &[u8] to PokedexEntryText
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let rom = vec![0x00, 0x83, 0x91, 0x88, 0x8B, 0x8B, 0x5F];
    /// let pokedex_entry = PokedexEntryText::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     pokedex_entry,
    ///     PokedexEntryText {
    ///         text: ROMString::from("DRILL"),
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let text_end_index = rom.iter().position(|&r| r == 0x5F).unwrap_or(rom.len());

        let text = ROMString::new(&rom[1..text_end_index]);

        PokedexEntryText { text }
    }
}

impl PokedexEntryText {
    /// Pokédex entry text to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::types::*;
    ///
    /// let pokedex_entry_text = PokedexEntryText {
    ///     text: ROMString::from("DRILL"),
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

#[derive(Debug, PartialEq)]
pub enum PokemonPicFace {
    FRONT,
    BACK,
}

impl From<bool> for PokemonPicFace {
    /// Convert bool to PokemonPicFace
    ///
    /// ```
    /// use pkmnapi_db::types::*;
    ///
    /// let pokemon_pic_face = PokemonPicFace::from(true);
    ///
    /// assert_eq!(pokemon_pic_face, PokemonPicFace::FRONT);
    ///
    /// let pokemon_pic_face = PokemonPicFace::from(false);
    ///
    /// assert_eq!(pokemon_pic_face, PokemonPicFace::BACK);
    /// ```
    fn from(pokemon_pic_face: bool) -> PokemonPicFace {
        match pokemon_pic_face {
            true => PokemonPicFace::FRONT,
            false => PokemonPicFace::BACK,
        }
    }
}

/// Trainer ID
///
/// # Example
///
/// ```
/// use pkmnapi_db::types::*;
///
/// let trainer_id = TrainerID::from(0x12);
///
/// assert_eq!(trainer_id, 0x12);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TrainerID(u8);

impl TrainerID {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl From<u8> for TrainerID {
    fn from(trainer_id: u8) -> Self {
        TrainerID(trainer_id)
    }
}

impl PartialEq<u8> for TrainerID {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for TrainerID {
    fn partial_cmp(&self, other: &u8) -> Option<Ordering> {
        self.0.partial_cmp(&other)
    }
}

impl fmt::Display for TrainerID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Sub<usize> for TrainerID {
    type Output = usize;

    fn sub(self, other: usize) -> usize {
        self.0 as usize - other
    }
}

impl Mul<usize> for TrainerID {
    type Output = usize;

    fn mul(self, other: usize) -> usize {
        self.0 as usize * other
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
