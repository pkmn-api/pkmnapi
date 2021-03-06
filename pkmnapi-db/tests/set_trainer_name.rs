use pkmnapi_db::patch::*;
use pkmnapi_db::string::*;
use pkmnapi_db::*;

mod common;

macro_rules! set_trainer_name_test {
    (
        $test_name:ident,
        $trainer_id:expr,
        $trainer_name:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_trainer_name(
                &$trainer_id,
                &TrainerName {
                    name: ROMString::from($trainer_name),
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for trainer ID: {}",
                    $trainer_id
                ),
                Err(_) => panic!(format!("Could not find trainer ID: {}", $trainer_id)),
            };
        }
    };
}

set_trainer_name_test!(
    set_trainer_name_1,
    1,
    "ABCDEFGHI",
    0x399FF,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
);
set_trainer_name_test!(
    set_trainer_name_2,
    2,
    "ABCDEFGHIJK",
    0x39A09,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]
);
set_trainer_name_test!(
    set_trainer_name_3,
    3,
    "ABCD",
    0x39A15,
    vec![0x80, 0x81, 0x82, 0x83]
);
set_trainer_name_test!(
    set_trainer_name_4,
    4,
    "ABCDEF",
    0x39A1A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_5,
    5,
    "ABCDEFGHIJK",
    0x39A21,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]
);
set_trainer_name_test!(
    set_trainer_name_6,
    6,
    "ABCDEFGHIJK",
    0x39A2D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]
);
set_trainer_name_test!(
    set_trainer_name_7,
    7,
    "ABCDEFGHIJ",
    0x39A39,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]
);
set_trainer_name_test!(
    set_trainer_name_8,
    8,
    "ABCDEFGHIJ",
    0x39A44,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89]
);
set_trainer_name_test!(
    set_trainer_name_9,
    9,
    "ABCDE",
    0x39A4F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_10,
    10,
    "ABCDE",
    0x39A55,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_11,
    11,
    "ABCDEFG",
    0x39A5B,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_12,
    12,
    "ABCDEFGH",
    0x39A63,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]
);
set_trainer_name_test!(
    set_trainer_name_13,
    13,
    "ABCDEFG",
    0x39A6C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_14,
    14,
    "ABCDEFGHI",
    0x39A74,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
);
set_trainer_name_test!(
    set_trainer_name_15,
    15,
    "ABCDEFG",
    0x39A7E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_16,
    16,
    "ABCDEFGH",
    0x39A86,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]
);
set_trainer_name_test!(
    set_trainer_name_17,
    17,
    "ABCDEFG",
    0x39A8F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_18,
    18,
    "ABCDEF",
    0x39A97,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_19,
    19,
    "ABCDEFG",
    0x39A9E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_20,
    20,
    "ABCDEF",
    0x39AA6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_21,
    21,
    "ABCDEFG",
    0x39AAD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_22,
    22,
    "ABCDE",
    0x39AB5,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_23,
    23,
    "ABCDEFGHIJK",
    0x39ABB,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A]
);
set_trainer_name_test!(
    set_trainer_name_24,
    24,
    "ABCDEFGHI",
    0x39AC7,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
);
set_trainer_name_test!(
    set_trainer_name_25,
    25,
    "ABCDEF",
    0x39AD1,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_26,
    26,
    "ABCDEFGH",
    0x39AD8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]
);
set_trainer_name_test!(
    set_trainer_name_27,
    27,
    "ABCDE",
    0x39AE1,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_28,
    28,
    "ABCDEFGHI",
    0x39AE7,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
);
set_trainer_name_test!(
    set_trainer_name_29,
    29,
    "ABCDEFGH",
    0x39AF1,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]
);
set_trainer_name_test!(
    set_trainer_name_30,
    30,
    "ABCDEF",
    0x39AFA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_31,
    31,
    "ABCDEFGHIJKL",
    0x39B01,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]
);
set_trainer_name_test!(
    set_trainer_name_32,
    32,
    "ABCDEFGHIJKL",
    0x39B0E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x8A, 0x8B]
);
set_trainer_name_test!(
    set_trainer_name_33,
    33,
    "ABCDE",
    0x39B1B,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_34,
    34,
    "ABCDE",
    0x39B21,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_35,
    35,
    "ABCDE",
    0x39B27,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_36,
    36,
    "ABCDEFGH",
    0x39B2D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87]
);
set_trainer_name_test!(
    set_trainer_name_37,
    37,
    "ABCDE",
    0x39B36,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
set_trainer_name_test!(
    set_trainer_name_38,
    38,
    "ABCD",
    0x39B3C,
    vec![0x80, 0x81, 0x82, 0x83]
);
set_trainer_name_test!(
    set_trainer_name_39,
    39,
    "ABCDEF",
    0x39B41,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_40,
    40,
    "ABCDEFG",
    0x39B48,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_41,
    41,
    "ABCDEFGHI",
    0x39B50,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
);
set_trainer_name_test!(
    set_trainer_name_42,
    42,
    "ABCDEF",
    0x39B5A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_43,
    43,
    "ABCDEF",
    0x39B61,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_44,
    44,
    "ABCDEFG",
    0x39B68,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86]
);
set_trainer_name_test!(
    set_trainer_name_45,
    45,
    "ABCDEFGHI",
    0x39B70,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88]
);
set_trainer_name_test!(
    set_trainer_name_46,
    46,
    "ABCDEF",
    0x39B7A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85]
);
set_trainer_name_test!(
    set_trainer_name_47,
    47,
    "ABCDE",
    0x39B81,
    vec![0x80, 0x81, 0x82, 0x83, 0x84]
);
