use pkmnapi_db::patch::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! set_tm_price_test {
    ($test_name:ident, $tm_id:expr, $price:expr, $patch_offset:expr, $patch_data:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_tm_price(&$tm_id, &TMPrice { value: $price }) {
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

#[rustfmt::skip::macros(set_tm_price_test)]

set_tm_price_test!(set_tm_1, 1, 4000, 0x7BFA7, vec![0x42]);
set_tm_price_test!(set_tm_2, 2, 4000, 0x7BFA7, vec![0x34]);
set_tm_price_test!(set_tm_3, 3, 4000, 0x7BFA8, vec![0x41]);
set_tm_price_test!(set_tm_4, 4, 4000, 0x7BFA8, vec![0x24]);
set_tm_price_test!(set_tm_5, 5, 4000, 0x7BFA9, vec![0x44]);
set_tm_price_test!(set_tm_6, 6, 4000, 0x7BFA9, vec![0x34]);
set_tm_price_test!(set_tm_7, 7, 4000, 0x7BFAA, vec![0x44]);
set_tm_price_test!(set_tm_8, 8, 4000, 0x7BFAA, vec![0x24]);
set_tm_price_test!(set_tm_9, 9, 4000, 0x7BFAB, vec![0x44]);
set_tm_price_test!(set_tm_10, 10, 4000, 0x7BFAB, vec![0x34]);
set_tm_price_test!(set_tm_11, 11, 4000, 0x7BFAC, vec![0x41]);
set_tm_price_test!(set_tm_12, 12, 4000, 0x7BFAC, vec![0x24]);
set_tm_price_test!(set_tm_13, 13, 4000, 0x7BFAD, vec![0x45]);
set_tm_price_test!(set_tm_14, 14, 4000, 0x7BFAD, vec![0x44]);
set_tm_price_test!(set_tm_15, 15, 4000, 0x7BFAE, vec![0x45]);
set_tm_price_test!(set_tm_16, 16, 4000, 0x7BFAE, vec![0x54]);
set_tm_price_test!(set_tm_17, 17, 4000, 0x7BFAF, vec![0x42]);
set_tm_price_test!(set_tm_18, 18, 4000, 0x7BFAF, vec![0x34]);
set_tm_price_test!(set_tm_19, 19, 4000, 0x7BFB0, vec![0x42]);
set_tm_price_test!(set_tm_20, 20, 4000, 0x7BFB0, vec![0x34]);
set_tm_price_test!(set_tm_21, 21, 4000, 0x7BFB1, vec![0x45]);
set_tm_price_test!(set_tm_22, 22, 4000, 0x7BFB1, vec![0x54]);
set_tm_price_test!(set_tm_23, 23, 4000, 0x7BFB2, vec![0x42]);
set_tm_price_test!(set_tm_24, 24, 4000, 0x7BFB2, vec![0x54]);
set_tm_price_test!(set_tm_25, 25, 4000, 0x7BFB3, vec![0x44]);
set_tm_price_test!(set_tm_26, 26, 4000, 0x7BFB3, vec![0x54]);
set_tm_price_test!(set_tm_27, 27, 4000, 0x7BFB4, vec![0x42]);
set_tm_price_test!(set_tm_28, 28, 4000, 0x7BFB4, vec![0x54]);
set_tm_price_test!(set_tm_29, 29, 4000, 0x7BFB5, vec![0x41]);
set_tm_price_test!(set_tm_30, 30, 4000, 0x7BFB5, vec![0x44]);
set_tm_price_test!(set_tm_31, 31, 4000, 0x7BFB6, vec![0x41]);
set_tm_price_test!(set_tm_32, 32, 4000, 0x7BFB6, vec![0x24]);
set_tm_price_test!(set_tm_33, 33, 4000, 0x7BFB7, vec![0x42]);
set_tm_price_test!(set_tm_34, 34, 4000, 0x7BFB7, vec![0x14]);
set_tm_price_test!(set_tm_35, 35, 4000, 0x7BFB8, vec![0x42]);
set_tm_price_test!(set_tm_36, 36, 4000, 0x7BFB8, vec![0x44]);
set_tm_price_test!(set_tm_37, 37, 4000, 0x7BFB9, vec![0x45]);
set_tm_price_test!(set_tm_38, 38, 4000, 0x7BFB9, vec![0x24]);
set_tm_price_test!(set_tm_39, 39, 4000, 0x7BFBA, vec![0x44]);
set_tm_price_test!(set_tm_40, 40, 4000, 0x7BFBA, vec![0x24]);
set_tm_price_test!(set_tm_41, 41, 4000, 0x7BFBB, vec![0x42]);
set_tm_price_test!(set_tm_42, 42, 4000, 0x7BFBB, vec![0x24]);
set_tm_price_test!(set_tm_43, 43, 4000, 0x7BFBC, vec![0x42]);
set_tm_price_test!(set_tm_44, 44, 4000, 0x7BFBC, vec![0x54]);
set_tm_price_test!(set_tm_45, 45, 4000, 0x7BFBD, vec![0x44]);
set_tm_price_test!(set_tm_46, 46, 4000, 0x7BFBD, vec![0x24]);
set_tm_price_test!(set_tm_47, 47, 4000, 0x7BFBE, vec![0x44]);
set_tm_price_test!(set_tm_48, 48, 4000, 0x7BFBE, vec![0x34]);
set_tm_price_test!(set_tm_49, 49, 4000, 0x7BFBF, vec![0x42]);
set_tm_price_test!(set_tm_50, 50, 4000, 0x7BFBF, vec![0x44]);
