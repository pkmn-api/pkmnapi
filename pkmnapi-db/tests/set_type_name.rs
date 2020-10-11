use pkmnapi_db::patch::*;
use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! set_type_name_test {
    ($test_name:ident, $type_id:expr, $type_name:expr, $patch_offset:expr, $patch_data:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_type_name(
                &$type_id,
                &TypeName {
                    name: ROMString::from($type_name),
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for type ID: {}",
                    $type_id
                ),
                Err(_) => panic!(format!("Could not find type ID: {}", $type_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(set_type_name_test)]

set_type_name_test!(set_type_name_0, 0, "ABC", 0x27DE4, vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]);
set_type_name_test!(
    set_type_name_1,
    1,
    "ABC",
    0x27DEB,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_2,
    2,
    "ABC",
    0x27DF4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_3,
    3,
    "ABC",
    0x27DFB,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_4,
    4,
    "ABC",
    0x27E28,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_5,
    5,
    "ABC",
    0x27E2F,
    vec![0x80, 0x81, 0x82, 0x50]
);
set_type_name_test!(
    set_type_name_6,
    6,
    "ABC",
    0x27E34,
    vec![0x80, 0x81, 0x82, 0x50]
);
set_type_name_test!(set_type_name_7, 7, "ABC", 0x27E39, vec![0x80, 0x81, 0x82]);
set_type_name_test!(
    set_type_name_8,
    8,
    "ABC",
    0x27E3D,
    vec![0x80, 0x81, 0x82, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_9,
    9,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_10,
    10,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_11,
    11,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_12,
    12,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_13,
    13,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_14,
    14,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_15,
    15,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_16,
    16,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_17,
    17,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_18,
    18,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_19,
    19,
    "ABC",
    0x27DE4,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_20,
    20,
    "ABC",
    0x27E02,
    vec![0x80, 0x81, 0x82, 0x50]
);
set_type_name_test!(
    set_type_name_21,
    21,
    "ABC",
    0x27E07,
    vec![0x80, 0x81, 0x82, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_22,
    22,
    "ABC",
    0x27E0D,
    vec![0x80, 0x81, 0x82, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_23,
    23,
    "ABC",
    0x27E13,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_type_name_test!(
    set_type_name_24,
    24,
    "ABC",
    0x27E1C,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50, 0x50]
);
set_type_name_test!(set_type_name_25, 25, "ABC", 0x27E24, vec![0x80, 0x81, 0x82]);
set_type_name_test!(
    set_type_name_26,
    26,
    "ABC",
    0x27E43,
    vec![0x80, 0x81, 0x82, 0x50, 0x50, 0x50]
);
