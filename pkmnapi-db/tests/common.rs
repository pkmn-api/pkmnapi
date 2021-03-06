use pkmnapi_db::*;
use std::env;
use std::fs;

#[allow(non_snake_case)]
pub fn load_rom() -> PkmnapiDB {
    let PKMN_ROM = env::var("PKMN_ROM")
        .expect("Set the PKMN_ROM environment variable to point to the ROM location");

    let rom = fs::read(PKMN_ROM).unwrap();

    PkmnapiDB::new(&rom).build().unwrap()
}
