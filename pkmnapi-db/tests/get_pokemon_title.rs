mod common;

#[test]
#[ignore]
#[allow(non_snake_case)]
fn get_pokemon_title() {
    let db = common::load_rom();

    let pokemon_title = db.get_pokemon_title().unwrap();

    assert_eq!(
        pokemon_title,
        vec![
            0xB0, 0xB1, 0x99, 0x70, 0x03, 0x1A, 0x54, 0x04, 0x01, 0x94, 0x19, 0x4C, 0x96, 0x22,
            0xA3, 0x85,
        ]
    );
}
