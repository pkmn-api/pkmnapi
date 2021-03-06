use pkmnapi_db::patch::*;
use pkmnapi_db::string::*;
use pkmnapi_db::*;

mod common;

macro_rules! set_pokedex_text_test {
    (
        $test_name:ident,
        $pokedex_id:expr,
        $pokedex_text:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_pokedex_text(
                &$pokedex_id,
                &PokedexText {
                    text: ROMString::from($pokedex_text),
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

set_pokedex_text_test!(
    set_pokedex_text_1,
    1,
    "ABCDE",
    0xAEE81,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_2,
    2,
    "ABCDE",
    0xAC2DE,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_3,
    3,
    "ABCDE",
    0xAEEDF,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_4,
    4,
    "ABCDE",
    0xAF416,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_5,
    5,
    "ABCDE",
    0xAF4D4,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_6,
    6,
    "ABCDE",
    0xAF593,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_7,
    7,
    "ABCDE",
    0xAF475,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_8,
    8,
    "ABCDE",
    0xAF52D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_9,
    9,
    "ABCDE",
    0xAC9E4,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_10,
    10,
    "ABCDE",
    0xAE565,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_11,
    11,
    "ABCDE",
    0xAE5C7,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_12,
    12,
    "ABCDE",
    0xAE626,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_13,
    13,
    "ABCDE",
    0xAE275,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_14,
    14,
    "ABCDE",
    0xAE2CA,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_15,
    15,
    "ABCDE",
    0xAE32D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_16,
    16,
    "ABCDE",
    0xACC26,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_17,
    17,
    "ABCDE",
    0xAED57,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_18,
    18,
    "ABCDE",
    0xAEDB9,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_19,
    19,
    "ABCDE",
    0xAF120,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_20,
    20,
    "ABCDE",
    0xAF17C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_21,
    21,
    "ABCDE",
    0xAC170,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_22,
    22,
    "ABCDE",
    0xACBC8,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_23,
    23,
    "ABCDE",
    0xAE0F1,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_24,
    24,
    "ABCDE",
    0xACF81,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_25,
    25,
    "ABCDE",
    0xAD98C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_26,
    26,
    "ABCDE",
    0xAD9EA,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_27,
    27,
    "ABCDE",
    0xADC7B,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_28,
    28,
    "ABCDE",
    0xADCD9,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_29,
    29,
    "ABCDE",
    0xAC51F,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_30,
    30,
    "ABCDE",
    0xAF238,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_31,
    31,
    "ABCDE",
    0xAC57D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_32,
    32,
    "ABCDE",
    0xAC0B4,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_33,
    33,
    "ABCDE",
    0xAF1DB,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_34,
    34,
    "ABCDE",
    0xAC227,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_35,
    35,
    "ABCDE",
    0xAC114,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_36,
    36,
    "ABCDE",
    0xAEAC0,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_37,
    37,
    "ABCDE",
    0xAD8CE,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_38,
    38,
    "ABCDE",
    0xAD92B,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_39,
    39,
    "ABCDE",
    0xADDE7,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_40,
    40,
    "ABCDE",
    0xADE4E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_41,
    41,
    "ABCDE",
    0xAE08D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_42,
    42,
    "ABCDE",
    0xAE7AD,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_43,
    43,
    "ABCDE",
    0xAF5F1,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_44,
    44,
    "ABCDE",
    0xAF658,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_45,
    45,
    "ABCDE",
    0xAF6B6,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_46,
    46,
    "ABCDE",
    0xAE14C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_47,
    47,
    "ABCDE",
    0xACFDB,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_48,
    48,
    "ABCDE",
    0xAD4AE,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_49,
    49,
    "ABCDE",
    0xAE4A7,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_50,
    50,
    "ABCDE",
    0xAD396,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_51,
    51,
    "ABCDE",
    0xAE44A,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_52,
    52,
    "ABCDE",
    0xAD80E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_53,
    53,
    "ABCDE",
    0xAEB76,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_54,
    54,
    "ABCDE",
    0xAD042,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_55,
    55,
    "ABCDE",
    0xAE6E4,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_56,
    56,
    "ABCDE",
    0xAD2DE,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_57,
    57,
    "ABCDE",
    0xAE3E8,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_58,
    58,
    "ABCDE",
    0xACB05,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_59,
    59,
    "ABCDE",
    0xAC6F2,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_60,
    60,
    "ABCDE",
    0xAD5C2,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_61,
    61,
    "ABCDE",
    0xAE1B2,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_62,
    62,
    "ABCDE",
    0xAE20E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_63,
    63,
    "ABCDE",
    0xAEC9D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_64,
    64,
    "ABCDE",
    0xACCDF,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_65,
    65,
    "ABCDE",
    0xAECF9,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_66,
    66,
    "ABCDE",
    0xAE02F,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_67,
    67,
    "ABCDE",
    0xACDF3,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_68,
    68,
    "ABCDE",
    0xAE67E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_69,
    69,
    "ABCDE",
    0xAF71C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_70,
    70,
    "ABCDE",
    0xAF77C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_71,
    71,
    "ABCDE",
    0xAF7DA,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_72,
    72,
    "ABCDE",
    0xAC872,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_73,
    73,
    "ABCDE",
    0xAEF3B,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_74,
    74,
    "ABCDE",
    0xAF290,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_75,
    75,
    "ABCDE",
    0xACD39,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_76,
    76,
    "ABCDE",
    0xAD0FF,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_77,
    77,
    "ABCDE",
    0xAF05D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_78,
    78,
    "ABCDE",
    0xAF0C4,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_79,
    79,
    "ABCDE",
    0xACC89,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_80,
    80,
    "ABCDE",
    0xAC27D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_81,
    81,
    "ABCDE",
    0xAF3AF,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_82,
    82,
    "ABCDE",
    0xAD219,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_83,
    83,
    "ABCDE",
    0xAD457,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_84,
    84,
    "ABCDE",
    0xAD565,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_85,
    85,
    "ABCDE",
    0xAE38A,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_86,
    86,
    "ABCDE",
    0xAD33D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_87,
    87,
    "ABCDE",
    0xAE508,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_88,
    88,
    "ABCDE",
    0xAC460,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_89,
    89,
    "ABCDE",
    0xAE93C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_90,
    90,
    "ABCDE",
    0xAC819,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_91,
    91,
    "ABCDE",
    0xAEA00,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_92,
    92,
    "ABCDE",
    0xAC8D6,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_93,
    93,
    "ABCDE",
    0xAEC3D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_94,
    94,
    "ABCDE",
    0xAC4C1,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_95,
    95,
    "ABCDE",
    0xACB61,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_96,
    96,
    "ABCDE",
    0xAD0A1,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_97,
    97,
    "ABCDE",
    0xAE74B,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_98,
    98,
    "ABCDE",
    0xAD871,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_99,
    99,
    "ABCDE",
    0xAE99D,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_100,
    100,
    "ABCDE",
    0xAC1CB,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_101,
    101,
    "ABCDE",
    0xAEA5A,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_102,
    102,
    "ABCDE",
    0xAC40C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_103,
    103,
    "ABCDE",
    0xAC340,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_104,
    104,
    "ABCDE",
    0xAC5DB,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_105,
    105,
    "ABCDE",
    0xAEBD9,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_106,
    106,
    "ABCDE",
    0xACEB4,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_107,
    107,
    "ABCDE",
    0xACF19,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_108,
    108,
    "ABCDE",
    0xAC3A7,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_109,
    109,
    "ABCDE",
    0xAD277,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_110,
    110,
    "ABCDE",
    0xAEB1B,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_111,
    111,
    "ABCDE",
    0xAC632,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_112,
    112,
    "ABCDE",
    0xAC000,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_113,
    113,
    "ABCDE",
    0xACD9B,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_114,
    114,
    "ABCDE",
    0xACA9E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_115,
    115,
    "ABCDE",
    0xAC05B,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_116,
    116,
    "ABCDE",
    0xADBBD,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_117,
    117,
    "ABCDE",
    0xADC1A,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_118,
    118,
    "ABCDE",
    0xAEF9F,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_119,
    119,
    "ABCDE",
    0xAF000,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_120,
    120,
    "ABCDE",
    0xAC98E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_121,
    121,
    "ABCDE",
    0xAEE1E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_122,
    122,
    "ABCDE",
    0xACE59,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_123,
    123,
    "ABCDE",
    0xAC932,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_124,
    124,
    "ABCDE",
    0xAD625,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_125,
    125,
    "ABCDE",
    0xAD1BC,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_126,
    126,
    "ABCDE",
    0xAD161,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_127,
    127,
    "ABCDE",
    0xACA43,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_128,
    128,
    "ABCDE",
    0xAD3FA,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_129,
    129,
    "ABCDE",
    0xAE8D8,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_130,
    130,
    "ABCDE",
    0xAC7B8,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_131,
    131,
    "ABCDE",
    0xAC693,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_132,
    132,
    "ABCDE",
    0xAD7A6,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_133,
    133,
    "ABCDE",
    0xADEB7,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_134,
    134,
    "ABCDE",
    0xADFCD,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_135,
    135,
    "ABCDE",
    0xADF73,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_136,
    136,
    "ABCDE",
    0xADF18,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_137,
    137,
    "ABCDE",
    0xAF2F0,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_138,
    138,
    "ABCDE",
    0xADD37,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_139,
    139,
    "ABCDE",
    0xADD8E,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_140,
    140,
    "ABCDE",
    0xADAFA,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_141,
    141,
    "ABCDE",
    0xADB56,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_142,
    142,
    "ABCDE",
    0xAF34C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_143,
    143,
    "ABCDE",
    0xAE878,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_144,
    144,
    "ABCDE",
    0xAD6E5,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_145,
    145,
    "ABCDE",
    0xAD743,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_146,
    146,
    "ABCDE",
    0xAD683,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_147,
    147,
    "ABCDE",
    0xADA3F,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_148,
    148,
    "ABCDE",
    0xADAA0,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_149,
    149,
    "ABCDE",
    0xAD50C,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_150,
    150,
    "ABCDE",
    0xAE812,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
set_pokedex_text_test!(
    set_pokedex_text_151,
    151,
    "ABCDE",
    0xAC74F,
    vec![0x00, 0x80, 0x81, 0x82, 0x83, 0x84, 0x5F]
);
