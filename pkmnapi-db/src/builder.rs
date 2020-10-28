use crate::error::Result;
use crate::header::Header;
use crate::sav::Sav;
use crate::PkmnapiDB;

#[derive(Debug)]
pub struct PkmnapiDBBuilder {
    pub rom: Vec<u8>,
    pub sav: Option<Vec<u8>>,
}

impl PkmnapiDBBuilder {
    pub fn sav(&mut self, sav: Vec<u8>) -> &Self {
        self.sav = Some(sav);

        self
    }

    pub fn build(self) -> Result<PkmnapiDB> {
        let hash = format!("{:x}", md5::compute(&self.rom));
        let header = Header::from(&self.rom)?;
        let rom = self.rom[..].to_vec();
        let sav = match self.sav {
            Some(sav) => Some(Sav::new(&sav)?),
            None => None,
        };

        Ok(PkmnapiDB {
            rom,
            sav,
            hash,
            header,
        })
    }
}
