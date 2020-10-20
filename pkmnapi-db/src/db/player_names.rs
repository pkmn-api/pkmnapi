use crate::error::{self, Result};
use crate::patch::*;
use crate::string::*;
use crate::PkmnapiDB;

impl PkmnapiDB {
    pub fn get_player_names(&self) -> Result<PlayerNames> {
        let offset_base = PkmnapiDB::ROM_PAGE;
        let offset = offset_base + 0x2AA8;

        let player_names = PlayerNames::from(&self.rom[offset..]);

        Ok(player_names)
    }

    pub fn set_player_names(&self, player_names: &PlayerNames) -> Result<Patch> {
        let old_player_names = self.get_player_names()?;
        let old_player_names_data = old_player_names.to_raw();
        let old_player_names_data_len = old_player_names_data.len();
        let player_names_data_a = player_names.to_raw();
        let player_names_data_len = player_names_data_a.len();

        if old_player_names_data_len != player_names_data_len {
            return Err(error::Error::PlayerNamesWrongSize(
                old_player_names_data_len,
                player_names_data_len,
            ));
        }

        let offset_base = PkmnapiDB::ROM_PAGE;
        let offset_a = offset_base + 0x2AA8;
        let offset_b = offset_base + 0x2AF2;
        let offset_raw_start = offset_a + player_names_data_len;
        let offset_raw_len = offset_b - offset_a - player_names_data_len;

        let player_names_data_b: Vec<u8> = player_names_data_a
            .iter()
            .map(|&x| {
                if x == 0x4E {
                    return 0x50;
                }

                return x;
            })
            .collect();

        let player_names_data = [
            player_names_data_a,
            self.rom[offset_raw_start..(offset_raw_start + offset_raw_len)].to_vec(),
            player_names_data_b,
        ]
        .concat();

        Ok(Patch::new(&offset_a, &player_names_data))
    }
}

#[derive(Debug, PartialEq)]
pub struct PlayerNames {
    pub player: Vec<ROMString>,
    pub rival: Vec<ROMString>,
}

impl From<&[u8]> for PlayerNames {
    fn from(rom: &[u8]) -> Self {
        let strings: Vec<ROMString> = rom
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if i == 0x00 {
                    return Some(i);
                }

                if *x == 0x50 || *x == 0x4E {
                    return Some(i + 1);
                }

                None
            })
            .take(8)
            .enumerate()
            .filter_map(|(i, index)| {
                if i % 4 != 0 {
                    return Some(index);
                }

                None
            })
            .map(|i| {
                let string: Vec<u8> = rom[i..]
                    .iter()
                    .take_while(|&x| *x != 0x50 && *x != 0x4E)
                    .map(|x| *x)
                    .collect();

                ROMString::new(&string[..])
            })
            .collect();

        PlayerNames {
            player: strings[0..3].to_vec(),
            rival: strings[3..].to_vec(),
        }
    }
}

impl PlayerNames {
    pub fn to_raw(&self) -> Vec<u8> {
        let new_name = ROMString::from("NEW NAME");
        let player_data = self
            .player
            .iter()
            .map(|name| [vec![0x4E], name.value.to_vec()].concat())
            .flatten()
            .collect();
        let rival_data = self
            .rival
            .iter()
            .map(|name| [vec![0x4E], name.value.to_vec()].concat())
            .flatten()
            .collect();

        [
            new_name.value.to_vec(),
            player_data,
            vec![0x50],
            new_name.value.to_vec(),
            rival_data,
            vec![0x50],
        ]
        .concat()
    }
}
