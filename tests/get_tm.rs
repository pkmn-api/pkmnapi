use pkmnapi::db::types::*;

mod common;

macro_rules! get_tm_test {
    ($test_name: ident, $tm_id: expr, $move_id: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_tm($tm_id) {
                Ok(tm) => assert_eq!(
                    tm,
                    TM {
                        move_id: MoveID::from($move_id)
                    },
                    "Searched for TM ID: {}",
                    $tm_id
                ),
                Err(_) => panic!(format!("Could not find TM ID: {}", $tm_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_tm_test)]

get_tm_test!(get_tm_1, 1, 5);
get_tm_test!(get_tm_2, 2, 13);
get_tm_test!(get_tm_3, 3, 14);
get_tm_test!(get_tm_4, 4, 18);
get_tm_test!(get_tm_5, 5, 25);
get_tm_test!(get_tm_6, 6, 92);
get_tm_test!(get_tm_7, 7, 32);
get_tm_test!(get_tm_8, 8, 34);
get_tm_test!(get_tm_9, 9, 36);
get_tm_test!(get_tm_10, 10, 38);
get_tm_test!(get_tm_11, 11, 61);
get_tm_test!(get_tm_12, 12, 55);
get_tm_test!(get_tm_13, 13, 58);
get_tm_test!(get_tm_14, 14, 59);
get_tm_test!(get_tm_15, 15, 63);
get_tm_test!(get_tm_16, 16, 6);
get_tm_test!(get_tm_17, 17, 66);
get_tm_test!(get_tm_18, 18, 68);
get_tm_test!(get_tm_19, 19, 69);
get_tm_test!(get_tm_20, 20, 99);
get_tm_test!(get_tm_21, 21, 72);
get_tm_test!(get_tm_22, 22, 76);
get_tm_test!(get_tm_23, 23, 82);
get_tm_test!(get_tm_24, 24, 85);
get_tm_test!(get_tm_25, 25, 87);
get_tm_test!(get_tm_26, 26, 89);
get_tm_test!(get_tm_27, 27, 90);
get_tm_test!(get_tm_28, 28, 91);
get_tm_test!(get_tm_29, 29, 94);
get_tm_test!(get_tm_30, 30, 100);
get_tm_test!(get_tm_31, 31, 102);
get_tm_test!(get_tm_32, 32, 104);
get_tm_test!(get_tm_33, 33, 115);
get_tm_test!(get_tm_34, 34, 117);
get_tm_test!(get_tm_35, 35, 118);
get_tm_test!(get_tm_36, 36, 120);
get_tm_test!(get_tm_37, 37, 121);
get_tm_test!(get_tm_38, 38, 126);
get_tm_test!(get_tm_39, 39, 129);
get_tm_test!(get_tm_40, 40, 130);
get_tm_test!(get_tm_41, 41, 135);
get_tm_test!(get_tm_42, 42, 138);
get_tm_test!(get_tm_43, 43, 143);
get_tm_test!(get_tm_44, 44, 156);
get_tm_test!(get_tm_45, 45, 86);
get_tm_test!(get_tm_46, 46, 149);
get_tm_test!(get_tm_47, 47, 153);
get_tm_test!(get_tm_48, 48, 157);
get_tm_test!(get_tm_49, 49, 161);
get_tm_test!(get_tm_50, 50, 164);
