use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_hm_move_all(&self, hm_ids: &Vec<u8>) -> Result<HashMap<u8, HMMove>> {
        self.get_all(hm_ids, |id| self.get_hm_move(id))
    }

    /// Get HM move by HM ID
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
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let hm_move = db.get_hm_move(&1).unwrap();
    ///
    /// assert_eq!(hm_move, HMMove { move_id: 0x0F });
    /// ```
    pub fn get_hm_move(&self, hm_id: &u8) -> Result<HMMove> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = 0x3052;
        let offset = offset_base + ((*hm_id as usize) - 1);

        let hm_move = HMMove::from(self.rom[offset]);

        Ok(hm_move)
    }

    /// Set HM move by HM ID
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
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let patch = db.set_hm_move(&1, &HMMove { move_id: 0x42 }).unwrap();
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
    pub fn set_hm_move(&self, hm_id: &u8, hm: &HMMove) -> Result<Patch> {
        let _max_id = self.hm_id_validate(hm_id)?;

        let offset_base = 0x3052;
        let offset = offset_base + ((*hm_id as usize) - 1);

        Ok(Patch::new(&offset, &hm.to_raw()))
    }
}

/// HM move
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
///
/// let hm_move = HMMove::from(0x01);
///
/// assert_eq!(hm_move, HMMove { move_id: 0x01 });
/// ```
#[derive(Debug, PartialEq)]
pub struct HMMove {
    pub move_id: u8,
}

impl From<u8> for HMMove {
    /// Convert u8 to HM move
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let hm_move = HMMove::from(0x01);
    ///
    /// assert_eq!(hm_move, HMMove { move_id: 0x01 });
    /// ```
    fn from(move_id: u8) -> Self {
        HMMove { move_id }
    }
}

impl HMMove {
    /// HM move to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let hm_move = HMMove::from(0x01);
    ///
    /// let raw = hm_move.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.move_id]
    }
}
