use pkmnapi_db::*;
use std::fs;

fn main() {
    let rom = fs::read("../secrets/pkmn-ff0000.gb").unwrap();
    let db = PkmnapiDB::new(&rom, None).unwrap();

    let (min_id, max_id) = db.tm_id_bounds();

    for id in min_id..=max_id {
        let tm_name = db.get_tm_name(&(id as u8)).unwrap();

        println!("{}", tm_name.name);
    }
}
