use crate::db::string::*;
use byteorder::ReadBytesExt;
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

/// Type ID
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
///         name: PkmnapiDBString::from_string("ABC@")
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
    ///         name: PkmnapiDBString::from_string("ABC@")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name = PkmnapiDBString::new(rom);

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
    ///     name: PkmnapiDBString::from_string("ABC@"),
    /// };
    ///
    /// let raw = type_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82, 0x50]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }

    /// Type name to string (trimmed)
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let type_name = PkmnapiDBTypeName {
    ///     name: PkmnapiDBString::from_string("ABC@"),
    /// };
    ///
    /// let string = type_name.to_string();
    ///
    /// assert_eq!(string, "ABC");
    /// ```
    pub fn to_string(&self) -> String {
        self.name.decode_trimmed()
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
/// let type_name = PkmnapiDBPokemonName::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     PkmnapiDBPokemonName {
///         name: PkmnapiDBString::from_string("ABC@")
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
    /// let type_name = PkmnapiDBPokemonName::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     type_name,
    ///     PkmnapiDBPokemonName {
    ///         name: PkmnapiDBString::from_string("ABC@")
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let name = PkmnapiDBString::new(rom);

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
    ///     name: PkmnapiDBString::from_string("ABC@"),
    /// };
    ///
    /// let raw = pokemon_name.to_raw();
    ///
    /// assert_eq!(raw, vec![0x80, 0x81, 0x82, 0x50]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        self.name.value[..].to_vec()
    }

    /// Pokémon name to string (trimmed)
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi::db::string::*;
    /// use pkmnapi::db::types::*;
    ///
    /// let pokemon_name = PkmnapiDBPokemonName {
    ///     name: PkmnapiDBString::from_string("ABC@"),
    /// };
    ///
    /// let string = pokemon_name.to_string();
    ///
    /// assert_eq!(string, "ABC");
    /// ```
    pub fn to_string(&self) -> String {
        self.name.decode_trimmed()
    }
}
