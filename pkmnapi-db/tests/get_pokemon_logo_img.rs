use std::fs;

mod common;

#[test]
#[ignore]
fn get_pokemon_logo_img() {
    let db = common::load_rom();
    let img = db.get_pokemon_logo_img().unwrap();
    let img_data = fs::read("../secrets/data/pokemon_logo.png").unwrap();

    assert_eq!(img.to_png().unwrap(), img_data);
}
