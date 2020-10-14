use pkmnapi_db::patch::*;
use pkmnapi_db::*;

mod common;

macro_rules! set_tm_test {
    ($test_name:ident, $tm_id:expr, $move_id:expr, $patch_offset:expr, $patch_data:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_tm(&$tm_id, &TM { move_id: $move_id }) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for TM ID: {}",
                    $tm_id
                ),
                Err(_) => panic!(format!("Could not find TM ID: {}", $tm_id)),
            };
        }
    };
}

set_tm_test!(set_tm_1, 1, 0x42, 0x13773, vec![0x42]);
set_tm_test!(set_tm_2, 2, 0x42, 0x13774, vec![0x42]);
set_tm_test!(set_tm_3, 3, 0x42, 0x13775, vec![0x42]);
set_tm_test!(set_tm_4, 4, 0x42, 0x13776, vec![0x42]);
set_tm_test!(set_tm_5, 5, 0x42, 0x13777, vec![0x42]);
set_tm_test!(set_tm_6, 6, 0x42, 0x13778, vec![0x42]);
set_tm_test!(set_tm_7, 7, 0x42, 0x13779, vec![0x42]);
set_tm_test!(set_tm_8, 8, 0x42, 0x1377A, vec![0x42]);
set_tm_test!(set_tm_9, 9, 0x42, 0x1377B, vec![0x42]);
set_tm_test!(set_tm_10, 10, 0x42, 0x1377C, vec![0x42]);
set_tm_test!(set_tm_11, 11, 0x42, 0x1377D, vec![0x42]);
set_tm_test!(set_tm_12, 12, 0x42, 0x1377E, vec![0x42]);
set_tm_test!(set_tm_13, 13, 0x42, 0x1377F, vec![0x42]);
set_tm_test!(set_tm_14, 14, 0x42, 0x13780, vec![0x42]);
set_tm_test!(set_tm_15, 15, 0x42, 0x13781, vec![0x42]);
set_tm_test!(set_tm_16, 16, 0x42, 0x13782, vec![0x42]);
set_tm_test!(set_tm_17, 17, 0x42, 0x13783, vec![0x42]);
set_tm_test!(set_tm_18, 18, 0x42, 0x13784, vec![0x42]);
set_tm_test!(set_tm_19, 19, 0x42, 0x13785, vec![0x42]);
set_tm_test!(set_tm_20, 20, 0x42, 0x13786, vec![0x42]);
set_tm_test!(set_tm_21, 21, 0x42, 0x13787, vec![0x42]);
set_tm_test!(set_tm_22, 22, 0x42, 0x13788, vec![0x42]);
set_tm_test!(set_tm_23, 23, 0x42, 0x13789, vec![0x42]);
set_tm_test!(set_tm_24, 24, 0x42, 0x1378A, vec![0x42]);
set_tm_test!(set_tm_25, 25, 0x42, 0x1378B, vec![0x42]);
set_tm_test!(set_tm_26, 26, 0x42, 0x1378C, vec![0x42]);
set_tm_test!(set_tm_27, 27, 0x42, 0x1378D, vec![0x42]);
set_tm_test!(set_tm_28, 28, 0x42, 0x1378E, vec![0x42]);
set_tm_test!(set_tm_29, 29, 0x42, 0x1378F, vec![0x42]);
set_tm_test!(set_tm_30, 30, 0x42, 0x13790, vec![0x42]);
set_tm_test!(set_tm_31, 31, 0x42, 0x13791, vec![0x42]);
set_tm_test!(set_tm_32, 32, 0x42, 0x13792, vec![0x42]);
set_tm_test!(set_tm_33, 33, 0x42, 0x13793, vec![0x42]);
set_tm_test!(set_tm_34, 34, 0x42, 0x13794, vec![0x42]);
set_tm_test!(set_tm_35, 35, 0x42, 0x13795, vec![0x42]);
set_tm_test!(set_tm_36, 36, 0x42, 0x13796, vec![0x42]);
set_tm_test!(set_tm_37, 37, 0x42, 0x13797, vec![0x42]);
set_tm_test!(set_tm_38, 38, 0x42, 0x13798, vec![0x42]);
set_tm_test!(set_tm_39, 39, 0x42, 0x13799, vec![0x42]);
set_tm_test!(set_tm_40, 40, 0x42, 0x1379A, vec![0x42]);
set_tm_test!(set_tm_41, 41, 0x42, 0x1379B, vec![0x42]);
set_tm_test!(set_tm_42, 42, 0x42, 0x1379C, vec![0x42]);
set_tm_test!(set_tm_43, 43, 0x42, 0x1379D, vec![0x42]);
set_tm_test!(set_tm_44, 44, 0x42, 0x1379E, vec![0x42]);
set_tm_test!(set_tm_45, 45, 0x42, 0x1379F, vec![0x42]);
set_tm_test!(set_tm_46, 46, 0x42, 0x137A0, vec![0x42]);
set_tm_test!(set_tm_47, 47, 0x42, 0x137A1, vec![0x42]);
set_tm_test!(set_tm_48, 48, 0x42, 0x137A2, vec![0x42]);
set_tm_test!(set_tm_49, 49, 0x42, 0x137A3, vec![0x42]);
set_tm_test!(set_tm_50, 50, 0x42, 0x137A4, vec![0x42]);
