use crate::error::Result;
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;
use std::collections::HashMap;

impl PkmnapiDB {
    pub fn get_trade_all(&self, trade_ids: &Vec<u8>) -> Result<HashMap<u8, Trade>> {
        self.get_all(trade_ids, |id| self.get_trade(id))
    }

    /// Get trade by trade ID
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
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let trade = db.get_trade(&0).unwrap();
    ///
    /// assert_eq!(
    ///     trade,
    ///     Trade::new(33, 30, "TERRY")
    /// );
    /// ```
    pub fn get_trade(&self, trade_id: &u8) -> Result<Trade> {
        self.trade_id_validate(trade_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x1B7B) + ((*trade_id * 0x0E) as usize);

        let trade = Trade::from(&self.rom[offset..]);

        let give_pokedex_id = self.internal_id_to_pokedex_id(&trade.give_internal_id)?;
        let get_pokedex_id = self.internal_id_to_pokedex_id(&trade.get_internal_id)?;

        let trade = Trade {
            give_pokedex_id,
            give_internal_id: 0,
            get_pokedex_id,
            get_internal_id: 0,
            nickname: trade.nickname,
        };

        Ok(trade)
    }

    /// Set trade by trade ID
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
    /// let db = PkmnapiDB::new(&rom).build().unwrap();
    ///
    /// let patch = db
    ///     .set_trade(
    ///         &0,
    ///         &Trade::new(4, 6, "CHARCHAR")
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x71B7B,
    ///         length: 0x0E,
    ///         data: vec![0xB0, 0xB4, 0x00, 0x82, 0x87, 0x80, 0x91, 0x82, 0x87, 0x80, 0x91, 0x50, 0x50, 0x50]
    ///     }
    /// );
    /// ```
    pub fn set_trade(&self, trade_id: &u8, trade: &Trade) -> Result<Patch> {
        self.trade_id_validate(trade_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x1B7B) + ((*trade_id * 0x0E) as usize);

        let give_internal_id = self.pokedex_id_to_internal_id(&trade.give_pokedex_id)?;
        let get_internal_id = self.pokedex_id_to_internal_id(&trade.get_pokedex_id)?;

        let trade = Trade {
            give_pokedex_id: 0,
            give_internal_id,
            get_pokedex_id: 0,
            get_internal_id,
            nickname: trade.nickname.clone(),
        };

        let data = trade.to_raw();

        Ok(Patch::new(&offset, &data))
    }
}

/// Trade
///
/// # Example
///
/// ```
/// use pkmnapi_db::string::*;
/// use pkmnapi_db::*;
///
/// let rom = vec![
///     0x01, 0x01, 0x00, 0x82, 0x87, 0x80, 0x91, 0x82, 0x87, 0x80, 0x91, 0x50, 0x50, 0x50,
/// ];
/// let trade = Trade::from(&rom[..]);
///
/// assert_eq!(trade.nickname, ROMString::from("CHARCHAR"));
/// ```
#[derive(Debug, PartialEq)]
pub struct Trade {
    pub give_pokedex_id: u8,
    pub(crate) give_internal_id: u8,
    pub get_pokedex_id: u8,
    pub(crate) get_internal_id: u8,
    pub nickname: ROMString,
}

impl From<&[u8]> for Trade {
    /// Convert &[u8] to Trade
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::string::*;
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![
    ///     0x01, 0x01, 0x00, 0x82, 0x87, 0x80, 0x91, 0x82, 0x87, 0x80, 0x91, 0x50, 0x50, 0x50,
    /// ];
    /// let trade = Trade::from(&rom[..]);
    ///
    /// assert_eq!(trade.nickname, ROMString::from("CHARCHAR"));
    /// ```
    fn from(rom: &[u8]) -> Self {
        let give_internal_id = rom[0] - 1;
        let get_internal_id = rom[1] - 1;
        let nickname: Vec<u8> = rom[3..]
            .iter()
            .take_while(|&x| *x != 0x50)
            .map(|x| *x)
            .collect();
        let nickname = ROMString::new(&nickname.as_slice());

        Trade {
            give_pokedex_id: 0,
            give_internal_id,
            get_pokedex_id: 0,
            get_internal_id,
            nickname,
        }
    }
}

impl Trade {
    pub fn new<S: Into<String>>(give_pokedex_id: u8, get_pokedex_id: u8, nickname: S) -> Self {
        Trade {
            give_pokedex_id,
            give_internal_id: 0,
            get_pokedex_id,
            get_internal_id: 0,
            nickname: ROMString::from(nickname),
        }
    }

    /// Trade to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let trade = Trade::new(4, 6, "CHARCHAR");
    ///
    /// let raw = trade.to_raw();
    ///
    /// assert_eq!(
    ///     raw,
    ///     vec![0x01, 0x01, 0x00, 0x82, 0x87, 0x80, 0x91, 0x82, 0x87, 0x80, 0x91, 0x50, 0x50, 0x50]
    /// );
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        let nickname = self.nickname.value.to_vec();
        let padding_len = 11 - nickname.len();

        [
            vec![self.give_internal_id + 1, self.get_internal_id + 1, 0x00],
            nickname,
            vec![0x50; padding_len],
        ]
        .concat()
    }
}
