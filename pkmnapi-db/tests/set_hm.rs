use pkmnapi_db::patch::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! set_hm_test {
    ($test_name:ident, $hm_id:expr, $move_id:expr, $patch_offset:expr, $patch_data:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_hm(&$hm_id, &HM { move_id: $move_id }) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for HM ID: {}",
                    $hm_id
                ),
                Err(_) => panic!(format!("Could not find HM ID: {}", $hm_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(set_hm_test)]

set_hm_test!(set_hm_1, 1, 42, 0x3052, vec![42]);
set_hm_test!(set_hm_2, 2, 42, 0x3053, vec![42]);
set_hm_test!(set_hm_3, 3, 42, 0x3054, vec![42]);
set_hm_test!(set_hm_4, 4, 42, 0x3055, vec![42]);
set_hm_test!(set_hm_5, 5, 42, 0x3056, vec![42]);
