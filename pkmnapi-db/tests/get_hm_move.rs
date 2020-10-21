use pkmnapi_db::*;

mod common;

macro_rules! get_hm_move_test {
    ($test_name:ident, $hm_id:expr, $move_id:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_hm_move(&$hm_id) {
                Ok(hm_move) => assert_eq!(
                    hm_move,
                    HMMove { move_id: $move_id },
                    "Searched for HM ID: {}",
                    $hm_id
                ),
                Err(_) => panic!(format!("Could not find HM ID: {}", $hm_id)),
            };
        }
    };
}

get_hm_move_test!(get_hm_move_1, 1, 15);
get_hm_move_test!(get_hm_move_2, 2, 19);
get_hm_move_test!(get_hm_move_3, 3, 57);
get_hm_move_test!(get_hm_move_4, 4, 70);
get_hm_move_test!(get_hm_move_5, 5, 148);
