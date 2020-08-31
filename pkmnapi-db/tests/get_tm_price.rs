use pkmnapi_db::types::*;

mod common;

macro_rules! get_tm_price_test {
    ($test_name: ident, $tm_id: expr, $tm_price: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_tm_price($tm_id) {
                Ok(tm_price) => assert_eq!(
                    tm_price,
                    TMPrice { value: $tm_price },
                    "Searched for TM ID: {}",
                    $tm_id
                ),
                Err(_) => panic!(format!("Could not find TM ID: {}", $tm_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_tm_price_test)]

get_tm_price_test!(get_tm_price_1, 1, 3000);
get_tm_price_test!(get_tm_price_2, 2, 2000);
get_tm_price_test!(get_tm_price_3, 3, 2000);
get_tm_price_test!(get_tm_price_4, 4, 1000);
get_tm_price_test!(get_tm_price_5, 5, 3000);
get_tm_price_test!(get_tm_price_6, 6, 4000);
get_tm_price_test!(get_tm_price_7, 7, 2000);
get_tm_price_test!(get_tm_price_8, 8, 4000);
get_tm_price_test!(get_tm_price_9, 9, 3000);
get_tm_price_test!(get_tm_price_10, 10, 4000);
get_tm_price_test!(get_tm_price_11, 11, 2000);
get_tm_price_test!(get_tm_price_12, 12, 1000);
get_tm_price_test!(get_tm_price_13, 13, 4000);
get_tm_price_test!(get_tm_price_14, 14, 5000);
get_tm_price_test!(get_tm_price_15, 15, 5000);
get_tm_price_test!(get_tm_price_16, 16, 5000);
get_tm_price_test!(get_tm_price_17, 17, 3000);
get_tm_price_test!(get_tm_price_18, 18, 2000);
get_tm_price_test!(get_tm_price_19, 19, 3000);
get_tm_price_test!(get_tm_price_20, 20, 2000);
get_tm_price_test!(get_tm_price_21, 21, 5000);
get_tm_price_test!(get_tm_price_22, 22, 5000);
get_tm_price_test!(get_tm_price_23, 23, 5000);
get_tm_price_test!(get_tm_price_24, 24, 2000);
get_tm_price_test!(get_tm_price_25, 25, 5000);
get_tm_price_test!(get_tm_price_26, 26, 4000);
get_tm_price_test!(get_tm_price_27, 27, 5000);
get_tm_price_test!(get_tm_price_28, 28, 2000);
get_tm_price_test!(get_tm_price_29, 29, 4000);
get_tm_price_test!(get_tm_price_30, 30, 1000);
get_tm_price_test!(get_tm_price_31, 31, 2000);
get_tm_price_test!(get_tm_price_32, 32, 1000);
get_tm_price_test!(get_tm_price_33, 33, 1000);
get_tm_price_test!(get_tm_price_34, 34, 2000);
get_tm_price_test!(get_tm_price_35, 35, 4000);
get_tm_price_test!(get_tm_price_36, 36, 2000);
get_tm_price_test!(get_tm_price_37, 37, 2000);
get_tm_price_test!(get_tm_price_38, 38, 5000);
get_tm_price_test!(get_tm_price_39, 39, 2000);
get_tm_price_test!(get_tm_price_40, 40, 4000);
get_tm_price_test!(get_tm_price_41, 41, 2000);
get_tm_price_test!(get_tm_price_42, 42, 2000);
get_tm_price_test!(get_tm_price_43, 43, 5000);
get_tm_price_test!(get_tm_price_44, 44, 2000);
get_tm_price_test!(get_tm_price_45, 45, 2000);
get_tm_price_test!(get_tm_price_46, 46, 4000);
get_tm_price_test!(get_tm_price_47, 47, 3000);
get_tm_price_test!(get_tm_price_48, 48, 4000);
get_tm_price_test!(get_tm_price_49, 49, 4000);
get_tm_price_test!(get_tm_price_50, 50, 2000);
