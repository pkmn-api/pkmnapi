use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    /// Get TM price by TM ID
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
    /// let tm_price = db.get_tm_price(&1).unwrap();
    ///
    /// assert_eq!(tm_price, TMPrice { value: 3000 });
    /// ```
    pub fn get_tm_price(&self, tm_id: &u8) -> Result<TMPrice> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x3D;
        let offset = (offset_base + 0x1FA7) + (((*tm_id as usize - 1) as f32 / 2.0) as usize);
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0xF0) >> 4
            } else {
                self.rom[offset] & 0x0F
            }
        };

        let tm_price = TMPrice::from(value);

        Ok(tm_price)
    }

    /// Set TM price by TM ID
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
    /// let patch = db.set_tm_price(&1, &TMPrice { value: 9000 }).unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x7BFA7,
    ///         length: 0x01,
    ///         data: vec![0x92]
    ///     }
    /// );
    /// ```
    pub fn set_tm_price(&self, tm_id: &u8, tm_price: &TMPrice) -> Result<Patch> {
        let _max_id = self.tm_id_validate(tm_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x3D;
        let offset = (offset_base + 0x1FA7) + ((((*tm_id as usize) - 1) as f32 / 2.0) as usize);
        let value = {
            if ((tm_id - 1) % 2) == 0 {
                (self.rom[offset] & 0x0F) | (tm_price.to_raw()[0] << 0x04)
            } else {
                (self.rom[offset] & 0xF0) | (tm_price.to_raw()[0])
            }
        };

        Ok(Patch::new(&offset, &vec![value]))
    }
}

/// TM price
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
///
/// let tm = TMPrice::from(0x01);
///
/// assert_eq!(tm, TMPrice { value: 1000 });
/// ```
#[derive(Debug, PartialEq)]
pub struct TMPrice {
    pub value: u32,
}

impl From<u8> for TMPrice {
    /// Convert u8 to TMPrice
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let tm = TMPrice::from(0x01);
    ///
    /// assert_eq!(tm, TMPrice { value: 1000 });
    /// ```
    fn from(tm_price: u8) -> Self {
        TMPrice {
            value: (tm_price as u32) * 1000,
        }
    }
}

impl TMPrice {
    /// TM price to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let tm_price = TMPrice::from(0x01);
    ///
    /// let raw = tm_price.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![(self.value as f32 / 1000.0) as u8]
    }
}
