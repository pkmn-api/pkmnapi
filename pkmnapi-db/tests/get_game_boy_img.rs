use std::fs;

mod common;

#[test]
#[ignore]
fn get_game_boy_img() {
    let db = common::load_rom();
    let img = db.get_game_boy_img().unwrap();
    let img_data = fs::read("../secrets/data/game_boy.png").unwrap();

    assert_eq!(img.to_png().unwrap(), img_data);
}
