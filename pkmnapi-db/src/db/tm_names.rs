use crate::error::Result;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_tm_name_all(&self, tm_ids: &Vec<u8>) -> Result<HashMap<u8, TMName>> {
        self.get_all(tm_ids, |id| self.get_tm_name(id))
    }

    /// Get TM name by TM ID
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    /// use pkmnapi_db::string::*;
    /// use std::fs;
    /// # use std::env;
    /// # let rom_path = env::var("PKMN_ROM").expect("Set the PKMN_ROM environment variable to point to the ROM location");
    ///
    /// let rom = fs::read(rom_path).unwrap();
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let tm_name = db.get_tm_name(&1).unwrap();
    ///
    /// assert_eq!(tm_name, TMName { name: ROMString::from("TM01") });
    /// ```
    pub fn get_tm_name(&self, tm_id: &u8) -> Result<TMName> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset = 0x303C;
        let tm_prefix = ROMString::new(&self.rom[offset..(offset + 2)]);

        let tm_name = TMName {
            name: ROMString::from(format!("{}{:02}", tm_prefix, tm_id)),
        };

        Ok(tm_name)
    }
}

/// TM name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let tm_name = TMName {
///     name: ROMString::from("TM01"),
/// };
///
/// assert_eq!(tm_name.name.to_string(), "TM01");
/// ```
#[derive(Debug, PartialEq)]
pub struct TMName {
    pub name: ROMString,
}
