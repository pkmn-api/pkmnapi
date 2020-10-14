use crate::error::Result;
use crate::patch::*;
use crate::PkmnapiDB;
use byteorder::ReadBytesExt;
use std::io::Cursor;

impl PkmnapiDB {
    /// Get move stats by move ID
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
    /// let move_stats = db.get_move_stats(&1).unwrap();
    ///
    /// assert_eq!(
    ///     move_stats,
    ///     MoveStats {
    ///         move_id: 0x01,
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: 0x00,
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// ```
    pub fn get_move_stats(&self, move_id: &u8) -> Result<MoveStats> {
        let (_min_id, _max_id) = self.move_id_validate(move_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + (((*move_id as usize) - 1) * 0x06);

        let move_stats = MoveStats::from(&self.rom[offset..(offset + 6)]);

        Ok(move_stats)
    }

    /// Set move stats by move ID
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
    /// let patch = db
    ///     .set_move_stats(
    ///         &1,
    ///         &MoveStats {
    ///             move_id: 0x01,
    ///             effect: 0x00,
    ///             power: 0xFF,
    ///             type_id: 0x01,
    ///             accuracy: 0.0,
    ///             pp: 0xFF,
    ///         },
    ///     )
    ///     .unwrap();
    ///
    /// assert_eq!(
    ///     patch,
    ///     Patch {
    ///         offset: 0x38000,
    ///         length: 0x06,
    ///         data: vec![0x01, 0x00, 0xFF, 0x01, 0x00, 0xFF]
    ///     }
    /// );
    /// ```
    pub fn set_move_stats(&self, move_id: &u8, move_stats: &MoveStats) -> Result<Patch> {
        let (_min_id, _max_id) = self.move_id_validate(move_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset = offset_base + (((*move_id as usize) - 1) * 0x06);

        let move_stats_raw = move_stats.to_raw();

        Ok(Patch::new(&offset, &move_stats_raw))
    }
}

/// Move stats
///
/// # Example
///
/// ```
/// use pkmnapi_db::*;
///
/// let rom = vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23];
/// let type_name = MoveStats::from(&rom[..]);
///
/// assert_eq!(
///     type_name,
///     MoveStats {
///         move_id: 0x01,
///         effect: 0x00,
///         power: 0x28,
///         type_id: 0x00,
///         accuracy: 1.0,
///         pp: 0x23
///     }
/// );
/// ```
#[derive(Debug, PartialEq)]
pub struct MoveStats {
    pub move_id: u8,
    pub effect: u8,
    pub power: u8,
    pub type_id: u8,
    pub accuracy: f32,
    pub pp: u8,
}

impl From<&[u8]> for MoveStats {
    /// Convert &[u8] to MoveStats
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let rom = vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23];
    /// let move_stats = MoveStats::from(&rom[..]);
    ///
    /// assert_eq!(
    ///     move_stats,
    ///     MoveStats {
    ///         move_id: 0x01,
    ///         effect: 0x00,
    ///         power: 0x28,
    ///         type_id: 0x00,
    ///         accuracy: 1.0,
    ///         pp: 0x23
    ///     }
    /// );
    /// ```
    fn from(rom: &[u8]) -> Self {
        let mut cursor = Cursor::new(rom);

        let move_id = cursor.read_u8().unwrap_or(0);
        let effect = cursor.read_u8().unwrap_or(0);
        let power = cursor.read_u8().unwrap_or(0);
        let type_id = cursor.read_u8().unwrap_or(0);
        let accuracy = (cursor.read_u8().unwrap_or(0) as f32) / 255.0;
        let pp = cursor.read_u8().unwrap_or(0);

        MoveStats {
            move_id,
            effect,
            power,
            type_id,
            accuracy,
            pp,
        }
    }
}

impl MoveStats {
    /// Pokémon name to raw bytes
    ///
    /// # Example
    ///
    /// ```
    /// use pkmnapi_db::*;
    ///
    /// let move_stats = MoveStats {
    ///     move_id: 0x01,
    ///     effect: 0x00,
    ///     power: 0x28,
    ///     type_id: 0x00,
    ///     accuracy: 1.0,
    ///     pp: 0x23,
    /// };
    ///
    /// let raw = move_stats.to_raw();
    ///
    /// assert_eq!(raw, vec![0x01, 0x00, 0x28, 0x00, 0xFF, 0x23]);
    /// ```
    pub fn to_raw(&self) -> Vec<u8> {
        vec![
            self.move_id,
            self.effect,
            self.power,
            self.type_id,
            (self.accuracy * 255.0) as u8,
            self.pp,
        ]
    }
}
