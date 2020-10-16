use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_tm_all(&self, tm_ids: &Vec<u8>) -> Result<HashMap<u8, TM>> {
        let tm_all: HashMap<u8, TM> = tm_ids
            .iter()
            .map(|tm_id| {
                let tm = self.get_tm(tm_id)?;

                Ok((*tm_id, tm))
            })
            .collect::<Result<HashMap<u8, TM>>>()?;

        Ok(tm_all)
    }

    /// Get TM by TM ID
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
    /// let tm = db.get_tm(&1).unwrap();
    ///
    /// assert_eq!(tm, TM { move_id: 0x05 });
    /// ```
    pub fn get_tm(&self, tm_id: &u8) -> Result<TM> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x09;
        let offset = (offset_base + 0x1773) + ((*tm_id as usize) - 1);

        let tm = TM::from(self.rom[offset]);

        Ok(tm)
    }

    /// Set TM by TM ID
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
    /// let patch = db.set_tm(&1, &TM { move_id: 0x42 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x13773,
    ///         length: 0x01,
    ///         data: vec![0x42]
    ///     }
    /// );
    /// ```
    pub fn set_tm(&self, tm_id: &u8, tm: &TM) -> Result<Patch> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x09;
        let offset = (offset_base + 0x1773) + ((*tm_id as usize) - 1);

        Ok(Patch::new(&offset, &tm.to_raw()))
    }
}

/// TM
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
///
/// let tm = TM::from(0x01);
///
/// assert_eq!(tm, TM { move_id: 0x01 });
/// ```
#[derive(Debug, PartialEq)]
pub struct TM {
    pub move_id: u8,
}

impl From<u8> for TM {
    /// Convert u8 to TM
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let tm = TM::from(0x01);
    ///
    /// assert_eq!(tm, TM { move_id: 0x01 });
    /// ```
    fn from(move_id: u8) -> Self {
        TM { move_id }
    }
}

impl TM {
    /// TM to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let tm = TM::from(0x01);
    ///
    /// let raw = tm.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.move_id]
    }
}
