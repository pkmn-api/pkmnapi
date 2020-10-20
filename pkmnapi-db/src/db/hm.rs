use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_hm_all(&self, hm_ids: &Vec<u8>) -> Result<HashMap<u8, HM>> {
        let hm_all: HashMap<u8, HM> = hm_ids
            .iter()
            .map(|hm_id| {
                let hm = self.get_hm(hm_id)?;

                Ok((*hm_id, hm))
            })
            .collect::<Result<HashMap<u8, HM>>>()?;

        Ok(hm_all)
    }

    /// Get HM by HM ID
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
    /// let hm = db.get_hm(&1).unwrap();
    ///
    /// assert_eq!(hm, HM { move_id: 0x0F });
    /// ```
    pub fn get_hm(&self, hm_id: &u8) -> Result<HM> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = 0x3052;
        let offset = offset_base + ((*hm_id as usize) - 1);

        let hm = HM::from(self.rom[offset]);

        Ok(hm)
    }

    /// Set HM by HM ID
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
    /// let patch = db.set_hm(&1, &HM { move_id: 0x42 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x3052,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// ```
    pub fn set_hm(&self, hm_id: &u8, hm: &HM) -> Result<Patch> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = 0x3052;
        let offset = offset_base + ((*hm_id as usize) - 1);

        Ok(Patch::new(&offset, &hm.to_raw()))
    }
}

/// HM
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
    /// use pkmnapi_db::*;
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
