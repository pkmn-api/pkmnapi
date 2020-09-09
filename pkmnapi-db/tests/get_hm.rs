use pkmnapi_db::types::*;

mod common;

macro_rules! get_hm_test {
    ($test_name: ident, $hm_id: expr, $move_id: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_hm(&$hm_id) {
                Ok(hm) => assert_eq!(
                    hm,
                    HM { move_id: $move_id },
                    "Searched for HM ID: {}",
                    $hm_id
                ),
                Err(_) => panic!(format!("Could not find HM ID: {}", $hm_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_hm_test)]

get_hm_test!(get_hm_1, 1, 15);
get_hm_test!(get_hm_2, 2, 19);
get_hm_test!(get_hm_3, 3, 57);
get_hm_test!(get_hm_4, 4, 70);
get_hm_test!(get_hm_5, 5, 148);
