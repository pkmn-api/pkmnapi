use crate::error::{self, Result};
use crate::patch::*;
use crate::pic::*;
use crate::PkmnapiDB;
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Cursor;

impl PkmnapiDB {
    pub fn get_trainer_pic(&self, trainer_id: &u8) -> Result<Pic> {
        self.trainer_id_validate(trainer_id)?;

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset_base = offset_base + 0x1914;

        let offset = offset_base + (((*trainer_id - 1) as usize) * 0x05);

        let pointer_base = PkmnapiDB::ROM_PAGE * 0x24;
        let pointer = pointer_base + {
            let mut cursor = Cursor::new(&self.rom[offset..(offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        let pic = Pic::new(&self.rom[pointer..])?;

        Ok(pic)
    }

    pub fn set_trainer_pic(
        &self,
        trainer_id: &u8,
        pic: &Pic,
        encoding_method: PicEncodingMethod,
    ) -> Result<Patch> {
        let old_pixels = self.get_trainer_pic(trainer_id)?;
        let pixels = pic.encode(encoding_method);

        if pixels.len() > old_pixels.bytes + 1 {
            return Err(error::Error::PicTooLarge);
        }

        let offset_base = PkmnapiDB::ROM_PAGE * 0x1C;
        let offset_base = offset_base + 0x1914;
        let offset = offset_base + (((*trainer_id - 1) as usize) * 0x05);

        let pointer_base = PkmnapiDB::ROM_PAGE * 0x24;
        let pointer = pointer_base + {
            let mut cursor = Cursor::new(&self.rom[offset..(offset + 2)]);

            cursor.read_u16::<LittleEndian>().unwrap_or(0) as usize
        };

        Ok(Patch::new(&pointer, &pixels))
    }
}
