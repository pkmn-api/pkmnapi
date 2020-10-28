use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_tm_move_all(&self, tm_ids: &Vec<u8>) -> Result<HashMap<u8, TMMove>> {
        self.get_all(tm_ids, |id| self.get_tm_move(id))
    }

    /// Get TM move by TM ID
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
    /// let tm_move = db.get_tm_move(&1).unwrap();
    ///
    /// assert_eq!(tm_move, TMMove { move_id: 0x05 });
    /// ```
    pub fn get_tm_move(&self, tm_id: &u8) -> Result<TMMove> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x04;
        let offset = (offset_base + 0x3773) + ((*tm_id as usize) - 1);

        let tm_move = TMMove::from(self.rom[offset]);

        Ok(tm_move)
    }

    /// Set TM move by TM ID
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
    /// let patch = db.set_tm_move(&1, &TMMove { move_id: 0x42 }).unwrap();
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
    pub fn set_tm_move(&self, tm_id: &u8, tm: &TMMove) -> Result<Patch> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x04;
        let offset = (offset_base + 0x3773) + ((*tm_id as usize) - 1);

        Ok(Patch::new(&offset, &tm.to_raw()))
    }
}

/// TM move
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
///
/// let tm_move = TMMove::from(0x01);
///
/// assert_eq!(tm_move, TMMove { move_id: 0x01 });
/// ```
#[derive(Debug, PartialEq)]
pub struct TMMove {
    pub move_id: u8,
}

impl From<u8> for TMMove {
    /// Convert u8 to TM move
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let tm_move = TMMove::from(0x01);
    ///
    /// assert_eq!(tm_move, TMMove { move_id: 0x01 });
    /// ```
    fn from(move_id: u8) -> Self {
        TMMove { move_id }
    }
}

impl TMMove {
    /// TM move to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let tm_move = TMMove::from(0x01);
    ///
    /// let raw = tm_move.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![self.move_id]
    }
}
