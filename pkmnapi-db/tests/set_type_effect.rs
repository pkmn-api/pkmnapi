use pkmnapi_db::patch::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! set_type_effect_test {
    (
        $test_name:ident,
        $type_effect_id:expr,
        $attacking_type_id:expr,
        $defending_type_id:expr,
        $multiplier:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_type_effect(
                &$type_effect_id,
                &TypeEffect {
                    attacking_type_id: $attacking_type_id,
                    defending_type_id: $defending_type_id,
                    multiplier: $multiplier,
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
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

#[rustfmt::skip::macros(set_type_effect_test)]

set_type_effect_test!(set_type_effect_0, 0, 0x01, 0x03, 0.5, 0x3E474, vec![0x01, 0x03, 0x05]);
set_type_effect_test!(
    set_type_effect_1,
    1,
    0x01,
    0x03,
    0.5,
    0x3E477,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_2,
    2,
    0x01,
    0x03,
    0.5,
    0x3E47A,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_3,
    3,
    0x01,
    0x03,
    0.5,
    0x3E47D,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_4,
    4,
    0x01,
    0x03,
    0.5,
    0x3E480,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_5,
    5,
    0x01,
    0x03,
    0.5,
    0x3E483,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_6,
    6,
    0x01,
    0x03,
    0.5,
    0x3E486,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_7,
    7,
    0x01,
    0x03,
    0.5,
    0x3E489,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_8,
    8,
    0x01,
    0x03,
    0.5,
    0x3E48C,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_9,
    9,
    0x01,
    0x03,
    0.5,
    0x3E48F,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_10,
    10,
    0x01,
    0x03,
    0.5,
    0x3E492,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_11,
    11,
    0x01,
    0x03,
    0.5,
    0x3E495,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_12,
    12,
    0x01,
    0x03,
    0.5,
    0x3E498,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_13,
    13,
    0x01,
    0x03,
    0.5,
    0x3E49B,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_14,
    14,
    0x01,
    0x03,
    0.5,
    0x3E49E,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_15,
    15,
    0x01,
    0x03,
    0.5,
    0x3E4A1,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_16,
    16,
    0x01,
    0x03,
    0.5,
    0x3E4A4,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_17,
    17,
    0x01,
    0x03,
    0.5,
    0x3E4A7,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_18,
    18,
    0x01,
    0x03,
    0.5,
    0x3E4AA,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_19,
    19,
    0x01,
    0x03,
    0.5,
    0x3E4AD,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_20,
    20,
    0x01,
    0x03,
    0.5,
    0x3E4B0,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_21,
    21,
    0x01,
    0x03,
    0.5,
    0x3E4B3,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_22,
    22,
    0x01,
    0x03,
    0.5,
    0x3E4B6,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_23,
    23,
    0x01,
    0x03,
    0.5,
    0x3E4B9,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_24,
    24,
    0x01,
    0x03,
    0.5,
    0x3E4BC,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_25,
    25,
    0x01,
    0x03,
    0.5,
    0x3E4BF,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_26,
    26,
    0x01,
    0x03,
    0.5,
    0x3E4C2,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_27,
    27,
    0x01,
    0x03,
    0.5,
    0x3E4C5,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_28,
    28,
    0x01,
    0x03,
    0.5,
    0x3E4C8,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_29,
    29,
    0x01,
    0x03,
    0.5,
    0x3E4CB,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_30,
    30,
    0x01,
    0x03,
    0.5,
    0x3E4CE,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_31,
    31,
    0x01,
    0x03,
    0.5,
    0x3E4D1,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_32,
    32,
    0x01,
    0x03,
    0.5,
    0x3E4D4,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_33,
    33,
    0x01,
    0x03,
    0.5,
    0x3E4D7,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_34,
    34,
    0x01,
    0x03,
    0.5,
    0x3E4DA,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_35,
    35,
    0x01,
    0x03,
    0.5,
    0x3E4DD,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_36,
    36,
    0x01,
    0x03,
    0.5,
    0x3E4E0,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_37,
    37,
    0x01,
    0x03,
    0.5,
    0x3E4E3,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_38,
    38,
    0x01,
    0x03,
    0.5,
    0x3E4E6,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_39,
    39,
    0x01,
    0x03,
    0.5,
    0x3E4E9,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_40,
    40,
    0x01,
    0x03,
    0.5,
    0x3E4EC,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_41,
    41,
    0x01,
    0x03,
    0.5,
    0x3E4EF,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_42,
    42,
    0x01,
    0x03,
    0.5,
    0x3E4F2,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_43,
    43,
    0x01,
    0x03,
    0.5,
    0x3E4F5,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_44,
    44,
    0x01,
    0x03,
    0.5,
    0x3E4F8,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_45,
    45,
    0x01,
    0x03,
    0.5,
    0x3E4FB,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_46,
    46,
    0x01,
    0x03,
    0.5,
    0x3E4FE,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_47,
    47,
    0x01,
    0x03,
    0.5,
    0x3E501,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_48,
    48,
    0x01,
    0x03,
    0.5,
    0x3E504,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_49,
    49,
    0x01,
    0x03,
    0.5,
    0x3E507,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_50,
    50,
    0x01,
    0x03,
    0.5,
    0x3E50A,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_51,
    51,
    0x01,
    0x03,
    0.5,
    0x3E50D,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_52,
    52,
    0x01,
    0x03,
    0.5,
    0x3E510,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_53,
    53,
    0x01,
    0x03,
    0.5,
    0x3E513,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_54,
    54,
    0x01,
    0x03,
    0.5,
    0x3E516,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_55,
    55,
    0x01,
    0x03,
    0.5,
    0x3E519,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_56,
    56,
    0x01,
    0x03,
    0.5,
    0x3E51C,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_57,
    57,
    0x01,
    0x03,
    0.5,
    0x3E51F,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_58,
    58,
    0x01,
    0x03,
    0.5,
    0x3E522,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_59,
    59,
    0x01,
    0x03,
    0.5,
    0x3E525,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_60,
    60,
    0x01,
    0x03,
    0.5,
    0x3E528,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_61,
    61,
    0x01,
    0x03,
    0.5,
    0x3E52B,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_62,
    62,
    0x01,
    0x03,
    0.5,
    0x3E52E,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_63,
    63,
    0x01,
    0x03,
    0.5,
    0x3E531,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_64,
    64,
    0x01,
    0x03,
    0.5,
    0x3E534,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_65,
    65,
    0x01,
    0x03,
    0.5,
    0x3E537,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_66,
    66,
    0x01,
    0x03,
    0.5,
    0x3E53A,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_67,
    67,
    0x01,
    0x03,
    0.5,
    0x3E53D,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_68,
    68,
    0x01,
    0x03,
    0.5,
    0x3E540,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_69,
    69,
    0x01,
    0x03,
    0.5,
    0x3E543,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_70,
    70,
    0x01,
    0x03,
    0.5,
    0x3E546,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_71,
    71,
    0x01,
    0x03,
    0.5,
    0x3E549,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_72,
    72,
    0x01,
    0x03,
    0.5,
    0x3E54C,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_73,
    73,
    0x01,
    0x03,
    0.5,
    0x3E54F,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_74,
    74,
    0x01,
    0x03,
    0.5,
    0x3E552,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_75,
    75,
    0x01,
    0x03,
    0.5,
    0x3E555,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_76,
    76,
    0x01,
    0x03,
    0.5,
    0x3E558,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_77,
    77,
    0x01,
    0x03,
    0.5,
    0x3E55B,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_78,
    78,
    0x01,
    0x03,
    0.5,
    0x3E55E,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_79,
    79,
    0x01,
    0x03,
    0.5,
    0x3E561,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_80,
    80,
    0x01,
    0x03,
    0.5,
    0x3E564,
    vec![0x01, 0x03, 0x05]
);
set_type_effect_test!(
    set_type_effect_81,
    81,
    0x01,
    0x03,
    0.5,
    0x3E567,
    vec![0x01, 0x03, 0x05]
);
