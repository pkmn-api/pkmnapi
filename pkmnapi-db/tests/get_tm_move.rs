use pkmnapi_db::*;

mod common;

macro_rules! get_tm_move_test {
    ($test_name:ident, $tm_id:expr, $move_id:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_tm_move(&$tm_id) {
                Ok(tm_move) => assert_eq!(
                    tm_move,
                    TMMove { move_id: $move_id },
                    "Searched for TM ID: {}",
                    $tm_id
                ),
                Err(_) => panic!(format!("Could not find TM ID: {}", $tm_id)),
            };
        }
    };
}

get_tm_move_test!(get_tm_move_1, 1, 5);
get_tm_move_test!(get_tm_move_2, 2, 13);
get_tm_move_test!(get_tm_move_3, 3, 14);
get_tm_move_test!(get_tm_move_4, 4, 18);
get_tm_move_test!(get_tm_move_5, 5, 25);
get_tm_move_test!(get_tm_move_6, 6, 92);
get_tm_move_test!(get_tm_move_7, 7, 32);
get_tm_move_test!(get_tm_move_8, 8, 34);
get_tm_move_test!(get_tm_move_9, 9, 36);
get_tm_move_test!(get_tm_move_10, 10, 38);
get_tm_move_test!(get_tm_move_11, 11, 61);
get_tm_move_test!(get_tm_move_12, 12, 55);
get_tm_move_test!(get_tm_move_13, 13, 58);
get_tm_move_test!(get_tm_move_14, 14, 59);
get_tm_move_test!(get_tm_move_15, 15, 63);
get_tm_move_test!(get_tm_move_16, 16, 6);
get_tm_move_test!(get_tm_move_17, 17, 66);
get_tm_move_test!(get_tm_move_18, 18, 68);
get_tm_move_test!(get_tm_move_19, 19, 69);
get_tm_move_test!(get_tm_move_20, 20, 99);
get_tm_move_test!(get_tm_move_21, 21, 72);
get_tm_move_test!(get_tm_move_22, 22, 76);
get_tm_move_test!(get_tm_move_23, 23, 82);
get_tm_move_test!(get_tm_move_24, 24, 85);
get_tm_move_test!(get_tm_move_25, 25, 87);
get_tm_move_test!(get_tm_move_26, 26, 89);
get_tm_move_test!(get_tm_move_27, 27, 90);
get_tm_move_test!(get_tm_move_28, 28, 91);
get_tm_move_test!(get_tm_move_29, 29, 94);
get_tm_move_test!(get_tm_move_30, 30, 100);
get_tm_move_test!(get_tm_move_31, 31, 102);
get_tm_move_test!(get_tm_move_32, 32, 104);
get_tm_move_test!(get_tm_move_33, 33, 115);
get_tm_move_test!(get_tm_move_34, 34, 117);
get_tm_move_test!(get_tm_move_35, 35, 118);
get_tm_move_test!(get_tm_move_36, 36, 120);
get_tm_move_test!(get_tm_move_37, 37, 121);
get_tm_move_test!(get_tm_move_38, 38, 126);
get_tm_move_test!(get_tm_move_39, 39, 129);
get_tm_move_test!(get_tm_move_40, 40, 130);
get_tm_move_test!(get_tm_move_41, 41, 135);
get_tm_move_test!(get_tm_move_42, 42, 138);
get_tm_move_test!(get_tm_move_43, 43, 143);
get_tm_move_test!(get_tm_move_44, 44, 156);
get_tm_move_test!(get_tm_move_45, 45, 86);
get_tm_move_test!(get_tm_move_46, 46, 149);
get_tm_move_test!(get_tm_move_47, 47, 153);
get_tm_move_test!(get_tm_move_48, 48, 157);
get_tm_move_test!(get_tm_move_49, 49, 161);
get_tm_move_test!(get_tm_move_50, 50, 164);
