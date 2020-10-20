use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use byteorder::ReadBytesExt;
use std::cmp;
use std::collections::HashMap;
use std::io::Cursor;

impl PkmnapiDB {
    pub fn get_type_effect_all(
        &self,
        type_effect_ids: &Vec<u8>,
    ) -> Result<HashMap<u8, TypeEffect>> {
        let type_effect_all: HashMap<u8, TypeEffect> = type_effect_ids
            .iter()
            .map(|type_effect_id| {
                let type_name = self.get_type_effect(type_effect_id)?;

                Ok((*type_effect_id, type_name))
            })
            .collect::<Result<HashMap<u8, TypeEffect>>>()?;

        Ok(type_effect_all)
    }

    /// Get type effect by type effect ID
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
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let type_effect = db.get_type_effect(&0).unwrap();
    ///
    /// assert_eq!(
    ///     type_effect,
    ///     TypeEffect {
    ///         attacking_type_id: 0x15,
    ///         defending_type_id: 0x14,
    ///         multiplier: 2.0
    ///     }
    /// );
    /// ```
    pub fn get_type_effect(&self, type_effect_id: &u8) -> Result<TypeEffect> {
        let _max_id = self.type_effect_id_validate(type_effect_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0F;
        let pointer = (offset_base + 0x2474) + ((*type_effect_id as usize) * 0x03);

        let type_effect = TypeEffect::from(&self.rom[pointer..(pointer + 3)]);

        Ok(type_effect)
    }

    /// Set type effect by type effect ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_type_effect(
    ///         &0,
    ///         &TypeEffect {
    ///             attacking_type_id: 0x13,
    ///             defending_type_id: 0x37,
    ///             multiplier: 0.5,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3E474,
    ///         length: 0x03,
    ///         data: vec![0x13, 0x37, 0x05]
    ///     }
    /// );
    /// ```
    pub fn set_type_effect(&self, type_effect_id: &u8, type_effect: &TypeEffect) -> Result<Patch> {
        let _max_id = self.type_effect_id_validate(type_effect_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0F;
        let pointer = offset_base + 0x2474 + ((*type_effect_id as usize) * 3);

        let type_effect_raw = type_effect.to_raw();

        Ok(Patch::new(&pointer, &type_effect_raw))
    }
}

/// Type effect (attacker, defender, multiplier)
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
