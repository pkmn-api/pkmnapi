use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

#[test]
#[ignore]
#[allow(non_snake_case)]
fn get_player_names() {
    let db = common::load_rom();

    let player_names = db.get_player_names().unwrap();

    #[cfg(feature = "PKMN_RED")]
    assert_eq!(
        player_names,
        PlayerNames {
            player: vec![
                ROMString::from("RED"),
                ROMString::from("ASH"),
                ROMString::from("JACK"),
            ],
            rival: vec![
                ROMString::from("BLUE"),
                ROMString::from("GARY"),
                ROMString::from("JOHN"),
            ]
        }
    );

    #[cfg(not(feature = "PKMN_RED"))]
    assert_eq!(
        player_names,
        PlayerNames {
            player: vec![
                ROMString::from("BLUE"),
                ROMString::from("GARY"),
                ROMString::from("JOHN"),
            ],
            rival: vec![
                ROMString::from("RED"),
                ROMString::from("ASH"),
                ROMString::from("JACK"),
            ]
        }
    );
}
