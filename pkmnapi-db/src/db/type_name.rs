use crate::error::{self, Result};
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_type_name_all(&self, type_ids: &Vec<u8>) -> Result<HashMap<u8, TypeName>> {
        self.get_all(type_ids, |id| self.get_type_name(id))
    }

    /// Get type name by type ID
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
    /// let type_name = db.get_type_name(&0).unwrap();
    ///
    /// assert_eq!(
    ///     type_name,
    ///     TypeName {
    ///         name: ROMString::from("NORMAL")
    ///     }
    /// );
    /// ```
    pub fn get_type_name(&self, type_id: &u8) -> Result<TypeName> {
        let _max_id = self.type_id_validate(type_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let pointer_base = offset_base + 0x7DAE;
        let pointer_offset = pointer_base + ((*type_id as usize) * 2);
        let pointer = offset_base + self.get_pointer(pointer_offset);

        let type_name = TypeName::from(&self.rom[pointer..=(pointer + 9)]);

        Ok(type_name)
    }

    /// Set type name by type ID
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
    ///     .set_type_name(
    ///         &0,
    ///         &TypeName {
    ///             name: ROMString::from("BORING"),
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x27DE4,
    ///         length: 0x06,
    ///         data: vec![0x81, 0x8E, 0x91, 0x88, 0x8D, 0x86]
    ///     }
    /// );
    /// ```
    pub fn set_type_name(&self, type_id: &u8, type_name: &TypeName) -> Result<Patch> {
        let old_type_name = self.get_type_name(type_id)?;
        let old_type_name_len = old_type_name.name.value.len();
        let type_name_raw = type_name.to_raw();
        let type_name_len = type_name_raw.len();

        if old_type_name_len < type_name_len {
            return Err(error::Error::TypeNameWrongSize(
                old_type_name_len,
                type_name_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x08;
        let pointer_offset = (offset_base + 0x7DAE) + ((*type_id as usize) * 2);
        let pointer = offset_base + self.get_pointer(pointer_offset);

        let data = [type_name_raw, vec![0x50; old_type_name_len - type_name_len]].concat();

        Ok(Patch::new(&pointer, &data))
    }
}

/// Type name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
