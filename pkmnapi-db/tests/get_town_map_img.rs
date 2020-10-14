use std::fs;

mod common;

#[test]
#[ignore]
fn get_town_map_img() {
    let db = common::load_rom();
    let img = db.get_town_map_img().unwrap();
    let img_data = fs::read("../secrets/data/town_map.png").unwrap();

    assert_eq!(img.to_png().unwrap(), img_data);
}
