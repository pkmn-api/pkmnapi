pub mod header;
pub mod patch;
pub mod string;

use byteorder::{LittleEndian, ReadBytesExt};
use header::*;
use patch::*;
use std::cmp;
use std::io::{Cursor, Read};
use std::num::Wrapping;
use string::*;

const ROM_PAGE: usize = 0x2000;

#[derive(Debug)]
pub struct PkmnapiDB {
    rom: Vec<u8>,
    pub hash: String,
    pub header: PkmnapiDBHeader,
}

#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTypeName {
    pub name: PkmnapiDBString,
}

#[derive(Debug, PartialEq)]
pub struct PkmnapiDBTypeEffect {
    pub attacker_id: u8,
    pub defender_id: u8,
    pub multiplier: f32,
}

#[derive(Debug, PartialEq)]
pub struct PkmnapiDBStats {
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub type_ids: Vec<u8>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}

impl PkmnapiDB {
    pub fn new(rom: &Vec<u8>) -> Result<PkmnapiDB, String> {
        let hash = format!("{:x}", md5::compute(&rom));
        let header = PkmnapiDBHeader::from(&rom)?;

        Ok(PkmnapiDB {
            rom: rom[..].to_vec(),
            hash,
            header,
        })
    }

    pub fn verify_checksum(&self, global_checksum: u16) -> bool {
        let rom = [&self.rom[..0x014E], &self.rom[0x0150..]].concat();
        let checksum = rom
            .iter()
            .fold(Wrapping(0u16), |acc, x| acc + Wrapping(*x as u16));

        checksum.0 == global_checksum
    }

    pub fn verify_hash<S: Into<String>>(&self, hash: S) -> bool {
        self.hash == hash.into()
    }

    pub fn apply_patch(&mut self, patch: PkmnapiDBPatch) {
        let rom = [
            &self.rom[..patch.offset],
            &patch.data[..],
            &self.rom[(patch.offset + patch.length)..],
        ]
        .concat();

        self.rom = rom;
    }

    pub fn get_type_name_by_id(&self, type_id: u8) -> Result<PkmnapiDBTypeName, String> {
        let offset_base = ROM_PAGE * 0x10;
        let pointer_offset = (offset_base + 0x7DAE) + (type_id as usize * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let name = PkmnapiDBString::new(&self.rom[pointer..=(pointer + 9)]);

        Ok(PkmnapiDBTypeName { name })
    }

    pub fn set_type_name_by_id(
        &self,
        type_id: u8,
        type_name: PkmnapiDBTypeName,
    ) -> Result<PkmnapiDBPatch, String> {
        let old_type_name = self.get_type_name_by_id(type_id)?;
        let old_type_name = old_type_name.name.decode_trimmed();
        let old_type_name_len = old_type_name.len();
        let type_name_raw = type_name.name.value;
        let type_name_len = type_name_raw.len();

        if old_type_name_len < type_name_len {
            return Err(format!(
                "Length mismatch: should be {} characters or fewer, found {}",
                old_type_name_len, type_name_len
            ));
        }

        let offset_base = ROM_PAGE * 0x10;
        let pointer_offset = (offset_base + 0x7DAE) + (type_id as usize * 2);
        let pointer = offset_base + {
            let mut cursor = Cursor::new(&self.rom[pointer_offset..(pointer_offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(PkmnapiDBPatch::new(pointer, type_name_raw))
    }

    pub fn get_type_effect_by_id(&self, type_effect_id: u8) -> Result<PkmnapiDBTypeEffect, String> {
        let offset_base = ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474;

        let max_index = (&self.rom[pointer..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();
        let max_id = ((max_index as f32) / 3.0) as u8;

        if type_effect_id >= max_id {
            return Err(format!("Invalid ID: valid range is 0-{}", max_id));
        }

        let pointer = pointer + (type_effect_id as usize * 3);
        let mut cursor = Cursor::new(&self.rom[pointer..(pointer + 3)]);

        let attacker_id = cursor.read_u8().unwrap_or(0);
        let defender_id = cursor.read_u8().unwrap_or(0);
        let multiplier = (cursor.read_u8().unwrap_or(0) as f32) / 10.0;

        Ok(PkmnapiDBTypeEffect {
            attacker_id,
            defender_id,
            multiplier,
        })
    }

    pub fn set_type_effect_by_id(
        &self,
        type_effect_id: u8,
        type_effect: PkmnapiDBTypeEffect,
    ) -> Result<PkmnapiDBPatch, String> {
        let offset_base = ROM_PAGE * 0x1F;
        let pointer = offset_base + 0x0474;

        let max_index = (&self.rom[pointer..])
            .iter()
            .position(|&r| r == 0xFF)
            .unwrap();
        let max_id = ((max_index as f32) / 3.0) as u8;

        if type_effect_id >= max_id {
            return Err(format!("Invalid ID: valid range is 0-{}", max_id));
        }

        let pointer = pointer + (type_effect_id as usize * 3);
        let type_effect_raw = vec![
            type_effect.attacker_id,
            type_effect.defender_id,
            cmp::min((type_effect.multiplier * 10.0) as u8, 254),
        ];

        Ok(PkmnapiDBPatch::new(pointer, type_effect_raw))
    }

    pub fn get_stats_by_id(&self, pokedex_id: u8) -> Result<PkmnapiDBStats, String> {
        let offset_base = ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + (pokedex_id as usize * 0x1C);
        let mut cursor = Cursor::new(&self.rom[offset..(offset + 0x1C)]);

        cursor.set_position(1); // pokedex id

        let base_hp = cursor.read_u8().unwrap_or(0);
        let base_attack = cursor.read_u8().unwrap_or(0);
        let base_defence = cursor.read_u8().unwrap_or(0);
        let base_speed = cursor.read_u8().unwrap_or(0);
        let base_special = cursor.read_u8().unwrap_or(0);
        let type_ids = {
            let mut type_ids = vec![0x00; 2];

            cursor.read_exact(&mut type_ids).unwrap();

            type_ids
        };
        let catch_rate = cursor.read_u8().unwrap_or(0);
        let base_exp_yield = cursor.read_u8().unwrap_or(0);

        Ok(PkmnapiDBStats {
            base_hp,
            base_attack,
            base_defence,
            base_speed,
            base_special,
            type_ids,
            catch_rate,
            base_exp_yield,
        })
    }

    pub fn set_stats_by_id(
        &self,
        pokedex_id: u8,
        stats: PkmnapiDBStats,
    ) -> Result<PkmnapiDBPatch, String> {
        let offset_base = ROM_PAGE * 0x1C;
        let offset = (offset_base + 0x03DE) + (pokedex_id as usize * 0x1C);

        let stats_raw = [
            &[
                pokedex_id,
                stats.base_hp,
                stats.base_attack,
                stats.base_defence,
                stats.base_speed,
                stats.base_special,
            ],
            &stats.type_ids[..],
            &[stats.catch_rate, stats.base_exp_yield],
        ]
        .concat();

        Ok(PkmnapiDBPatch::new(offset, stats_raw))
    }
}

#[cfg(test)]
mod tests {
    use crate::db::*;

    #[test]
    fn new_success() {
        let rom = vec![0u8; 1024];

        PkmnapiDB::new(&rom).unwrap();
    }

    #[test]
    fn new_failure() {
        let rom = vec![];

        match PkmnapiDB::new(&rom) {
            Err(e) => assert_eq!(e, "Header too small"),
            _ => {}
        };
    }

    #[test]
    fn header_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();
        let header = PkmnapiDBHeader::from(&rom).unwrap();

        assert_eq!(db.header, header);
    }

    #[test]
    fn header_verify_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.header.verify(), true);
    }

    #[test]
    fn header_verify_fail() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x42],                                                  // (wrong) header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.header.verify(), false);
    }

    #[test]
    fn verify_checksum_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.verify_checksum(0x1A41), true);
    }

    #[test]
    fn verify_checksum_fail() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.verify_checksum(0x1234), false);
    }

    #[test]
    fn verify_hash_success() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.verify_hash("b933af3d953bedd6ed3911ef6724cfa2"), true);
    }

    #[test]
    fn verify_hash_fail() {
        let rom = [
            vec![0u8; 0x100],             // padding
            vec![0x00, 0xC3, 0x50, 0x01], // entry_point
            vec![
                // logo
                0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C,
                0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6,
                0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC,
                0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
            ],
            "GAMEBOYGAME".chars().map(|c| c as u8).collect::<Vec<u8>>(), // title
            vec![0u8; 5],                                                // title padding
            vec![0x30u8, 0x31u8],                                        // new_licensee_code
            vec![0u8],                                                   // sgb_flag
            vec![0u8],                                                   // cartridge_type
            vec![0x05u8],                                                // rom_size
            vec![0u8],                                                   // ram_size
            vec![0x01u8],                                                // destination_code
            vec![0x01u8],                                                // old_licensee_code
            vec![0x01u8],                                                // mask_rom_version_number
            vec![0x60],                                                  // header_checksum
            vec![0x1A, 0x41],                                            // global_checksum
        ]
        .concat();
        let db = PkmnapiDB::new(&rom).unwrap();

        assert_eq!(db.verify_hash("0123456789abcdef0123456789abcdef"), false);
    }
}
