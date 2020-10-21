use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_trainer_reward_all(&self, trainer_ids: &Vec<u8>) -> Result<HashMap<u8, u32>> {
        self.get_all(trainer_ids, |id| self.get_trainer_reward(id))
    }

    /// Get trainer reward by trainer ID
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
    /// let trainer_reward = db.get_trainer_reward(&1).unwrap();
    ///
    /// assert_eq!(
    ///     trainer_reward,
    ///     1500
    /// );
    /// ```
    pub fn get_trainer_reward(&self, trainer_id: &u8) -> Result<u32> {
        self.trainer_id_validate(trainer_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = (offset_base + 0x1914) + (((*trainer_id - 1) as usize) * 0x05);

        let trainer_reward = vec![
            (self.rom[offset + 2] & 0xF0) >> 0x04,
            self.rom[offset + 2] & 0x0F,
            (self.rom[offset + 3] & 0xF0) >> 0x04,
            self.rom[offset + 3] & 0x0F,
            (self.rom[offset + 4] & 0xF0) >> 0x04,
            self.rom[offset + 4] & 0x0F,
        ]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, val)| {
            acc + ((*val as u32) * (10u32.pow(5 - (i as u32))))
        });

        Ok(trainer_reward)
    }

    /// Set trainer reward by trainer ID
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
    ///     .set_trainer_reward(
    ///         &1,
    ///         &1337,
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x39916,
    ///         length: 0x03,
    ///         data: vec![0x00, 0x13, 0x37]
    ///     }
    /// );
    /// ```
    pub fn set_trainer_reward(&self, trainer_id: &u8, trainer_reward: &u32) -> Result<Patch> {
        self.trainer_id_validate(trainer_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x0E;
        let offset = ((offset_base + 0x1914) + (((*trainer_id - 1) as usize) * 0x05)) + 0x02;

        let data = (0..=5)
            .map(|i| ((*trainer_reward / 10u32.pow(5 - i)) % 10) as u8)
            .collect::<Vec<u8>>()
            .as_slice()
            .chunks(2)
            .map(|chunk| (chunk[0] << 0x04) | chunk[1])
            .collect();

        Ok(Patch::new(&offset, &data))
    }
}
