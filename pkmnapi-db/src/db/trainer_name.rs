use crate::error::{self, Result};
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    /// Get trainer name by trainer ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let trainer_name = db.get_trainer_name(&1).unwrap();
    ///
    /// assert_eq!(
    ///     trainer_name,
    ///     TrainerName {
    ///         name: ROMString::from("YOUNGSTER")
    ///     }
    /// );
    /// ```
    pub fn get_trainer_name(&self, trainer_id: &u8) -> Result<TrainerName> {
        let offset_base = (PkmnapiDB::ROM_PAGE * 0x1C) + 0x19FF;

        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let offset = match {
            if trainer_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*trainer_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id)),
        };

        let trainer_name = TrainerName::from(&self.rom[offset..(offset + 13)]);

        Ok(trainer_name)
    }

    /// Set trainer name by trainer ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::patch::*;
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom, None).unwrap();
    ///
    /// let patch = db
    ///     .set_trainer_name(
    ///         &1,
    ///         &TrainerName {
    ///             name: ROMString::from("ABCDEFGHI"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x399FF,
    ///         length: 0x09,
    ///         data: vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
    ///     }
    /// );
    /// ```
    pub fn set_trainer_name(&self, trainer_id: &u8, trainer_name: &TrainerName) -> Result<Patch> {
        let (min_id, max_id) = self.trainer_id_validate(trainer_id)?;

        let old_trainer_name = self.get_trainer_name(trainer_id)?;
        let old_trainer_name_len = old_trainer_name.name.value.len();
        let trainer_name_raw = trainer_name.to_raw();
        let trainer_name_len = trainer_name_raw.len();

        if old_trainer_name_len != trainer_name_len {
            return Err(error::Error::TrainerNameWrongSize(
                old_trainer_name_len,
                trainer_name_len,
            ));
        }

        let offset_base = (PkmnapiDB::ROM_PAGE * 0x1C) + 0x19FF;

        let offset = match {
            if trainer_id == &1 {
                Some(offset_base)
            } else {
                self.rom[offset_base..]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if *x == 0x50 {
                            return Some(offset_base + i + 1);
                        }

                        None
                    })
                    .take(max_id - 1)
                    .enumerate()
                    .filter_map(|(i, x)| {
                        if (*trainer_id as usize) - 2 == i {
                            return Some(x);
                        }

                        None
                    })
                    .next()
            }
        } {
            Some(offset) => offset,
            None => return Err(error::Error::TrainerIDInvalid(*trainer_id, min_id, max_id)),
        };

        Ok(Patch::new(&offset, &trainer_name_raw))
    }
}

/// Trainer name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
