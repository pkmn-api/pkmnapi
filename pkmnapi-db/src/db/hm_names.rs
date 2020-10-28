use crate::error::Result;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_hm_name_all(&self, hm_ids: &Vec<u8>) -> Result<HashMap<u8, HMName>> {
        self.get_all(hm_ids, |id| self.get_hm_name(id))
    }

    /// Get HM name by HM ID
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
    /// let hm_name = db.get_hm_name(&1).unwrap();
    ///
    /// assert_eq!(hm_name, HMName { name: ROMString::from("HM01") });
    /// ```
    pub fn get_hm_name(&self, hm_id: &u8) -> Result<HMName> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset = 0x303E;
        let hm_prefix = ROMString::new(&self.rom[offset..(offset + 2)]);

        let hm_name = HMName {
            name: ROMString::from(format!("{}{:02}", hm_prefix, hm_id)),
        };

        Ok(hm_name)
    }
}

/// HM name
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let hm_name = HMName {
///     name: ROMString::from("HM01"),
/// };
///
/// assert_eq!(hm_name.name.to_string(), "HM01");
/// ```
#[derive(Debug, PartialEq)]
pub struct HMName {
    pub name: ROMString,
}
