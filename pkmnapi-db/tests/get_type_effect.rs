use pkmnapi_db::types::*;

mod common;

macro_rules! get_type_effect_test {
    ($test_name: ident, $type_effect_id: expr, $attacking_type_id: expr, $defending_type_id: expr, $multiplier: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_type_effect(&$type_effect_id) {
                Ok(type_effect) => assert_eq!(
                    type_effect,
                    TypeEffect {
                        attacking_type_id: $attacking_type_id,
                        defending_type_id: $defending_type_id,
                        multiplier: $multiplier
                    },
                    "Searched for type effect ID: {}",
                    $type_effect_id
                ),
                Err(_) => panic!(format!(
                    "Could not find type effect ID: {}",
                    $type_effect_id
                )),
            };
        }
    };
}

#[rustfmt::skip::macros(get_type_effect_test)]

get_type_effect_test!(get_type_effect_0, 0, 21, 20, 2.0);
get_type_effect_test!(get_type_effect_1, 1, 20, 22, 2.0);
get_type_effect_test!(get_type_effect_2, 2, 20, 25, 2.0);
get_type_effect_test!(get_type_effect_3, 3, 22, 21, 2.0);
get_type_effect_test!(get_type_effect_4, 4, 23, 21, 2.0);
get_type_effect_test!(get_type_effect_5, 5, 21, 5, 2.0);
get_type_effect_test!(get_type_effect_6, 6, 4, 2, 0.0);
get_type_effect_test!(get_type_effect_7, 7, 21, 21, 0.5);
get_type_effect_test!(get_type_effect_8, 8, 20, 20, 0.5);
get_type_effect_test!(get_type_effect_9, 9, 23, 23, 0.5);
get_type_effect_test!(get_type_effect_10, 10, 25, 25, 0.5);
get_type_effect_test!(get_type_effect_11, 11, 22, 22, 0.5);
get_type_effect_test!(get_type_effect_12, 12, 24, 24, 0.5);
get_type_effect_test!(get_type_effect_13, 13, 20, 21, 0.5);
get_type_effect_test!(get_type_effect_14, 14, 22, 20, 0.5);
get_type_effect_test!(get_type_effect_15, 15, 21, 22, 0.5);
get_type_effect_test!(get_type_effect_16, 16, 23, 22, 0.5);
get_type_effect_test!(get_type_effect_17, 17, 0, 5, 0.5);
get_type_effect_test!(get_type_effect_18, 18, 0, 8, 0.0);
get_type_effect_test!(get_type_effect_19, 19, 8, 8, 2.0);
get_type_effect_test!(get_type_effect_20, 20, 20, 7, 2.0);
get_type_effect_test!(get_type_effect_21, 21, 20, 5, 0.5);
get_type_effect_test!(get_type_effect_22, 22, 21, 4, 2.0);
get_type_effect_test!(get_type_effect_23, 23, 23, 4, 0.0);
get_type_effect_test!(get_type_effect_24, 24, 23, 2, 2.0);
get_type_effect_test!(get_type_effect_25, 25, 22, 4, 2.0);
get_type_effect_test!(get_type_effect_26, 26, 22, 7, 0.5);
get_type_effect_test!(get_type_effect_27, 27, 22, 3, 0.5);
get_type_effect_test!(get_type_effect_28, 28, 22, 5, 2.0);
get_type_effect_test!(get_type_effect_29, 29, 22, 2, 0.5);
get_type_effect_test!(get_type_effect_30, 30, 25, 21, 0.5);
get_type_effect_test!(get_type_effect_31, 31, 25, 22, 2.0);
get_type_effect_test!(get_type_effect_32, 32, 25, 4, 2.0);
get_type_effect_test!(get_type_effect_33, 33, 25, 2, 2.0);
get_type_effect_test!(get_type_effect_34, 34, 1, 0, 2.0);
get_type_effect_test!(get_type_effect_35, 35, 1, 3, 0.5);
get_type_effect_test!(get_type_effect_36, 36, 1, 2, 0.5);
get_type_effect_test!(get_type_effect_37, 37, 1, 24, 0.5);
get_type_effect_test!(get_type_effect_38, 38, 1, 7, 0.5);
get_type_effect_test!(get_type_effect_39, 39, 1, 5, 2.0);
get_type_effect_test!(get_type_effect_40, 40, 1, 25, 2.0);
get_type_effect_test!(get_type_effect_41, 41, 1, 8, 0.0);
get_type_effect_test!(get_type_effect_42, 42, 3, 22, 2.0);
get_type_effect_test!(get_type_effect_43, 43, 3, 3, 0.5);
get_type_effect_test!(get_type_effect_44, 44, 3, 4, 0.5);
get_type_effect_test!(get_type_effect_45, 45, 3, 7, 2.0);
get_type_effect_test!(get_type_effect_46, 46, 3, 5, 0.5);
get_type_effect_test!(get_type_effect_47, 47, 3, 8, 0.5);
get_type_effect_test!(get_type_effect_48, 48, 4, 20, 2.0);
get_type_effect_test!(get_type_effect_49, 49, 4, 23, 2.0);
get_type_effect_test!(get_type_effect_50, 50, 4, 22, 0.5);
get_type_effect_test!(get_type_effect_51, 51, 4, 7, 0.5);
get_type_effect_test!(get_type_effect_52, 52, 4, 5, 2.0);
get_type_effect_test!(get_type_effect_53, 53, 4, 3, 2.0);
get_type_effect_test!(get_type_effect_54, 54, 2, 23, 0.5);
get_type_effect_test!(get_type_effect_55, 55, 2, 1, 2.0);
get_type_effect_test!(get_type_effect_56, 56, 2, 7, 2.0);
get_type_effect_test!(get_type_effect_57, 57, 2, 22, 2.0);
get_type_effect_test!(get_type_effect_58, 58, 2, 5, 0.5);
get_type_effect_test!(get_type_effect_59, 59, 24, 1, 2.0);
get_type_effect_test!(get_type_effect_60, 60, 24, 3, 2.0);
get_type_effect_test!(get_type_effect_61, 61, 7, 20, 0.5);
get_type_effect_test!(get_type_effect_62, 62, 7, 22, 2.0);
get_type_effect_test!(get_type_effect_63, 63, 7, 1, 0.5);
get_type_effect_test!(get_type_effect_64, 64, 7, 2, 0.5);
get_type_effect_test!(get_type_effect_65, 65, 7, 24, 2.0);
get_type_effect_test!(get_type_effect_66, 66, 7, 8, 0.5);
get_type_effect_test!(get_type_effect_67, 67, 7, 3, 2.0);
get_type_effect_test!(get_type_effect_68, 68, 5, 20, 2.0);
get_type_effect_test!(get_type_effect_69, 69, 5, 1, 0.5);
get_type_effect_test!(get_type_effect_70, 70, 5, 4, 0.5);
get_type_effect_test!(get_type_effect_71, 71, 5, 2, 2.0);
get_type_effect_test!(get_type_effect_72, 72, 5, 7, 2.0);
get_type_effect_test!(get_type_effect_73, 73, 5, 25, 2.0);
get_type_effect_test!(get_type_effect_74, 74, 8, 0, 0.0);
get_type_effect_test!(get_type_effect_75, 75, 8, 24, 0.0);
get_type_effect_test!(get_type_effect_76, 76, 20, 26, 0.5);
get_type_effect_test!(get_type_effect_77, 77, 21, 26, 0.5);
get_type_effect_test!(get_type_effect_78, 78, 23, 26, 0.5);
get_type_effect_test!(get_type_effect_79, 79, 22, 26, 0.5);
get_type_effect_test!(get_type_effect_80, 80, 25, 26, 2.0);
get_type_effect_test!(get_type_effect_81, 81, 26, 26, 2.0);
