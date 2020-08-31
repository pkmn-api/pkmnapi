use pkmnapi::db::patch::*;
use pkmnapi::db::string::*;
use pkmnapi::db::types::*;

mod common;

macro_rules! set_pokedex_entry_test {
    ($test_name: ident, $pokedex_id: expr, $species: expr, $height: expr, $weight: expr, $patch_offset: expr, $patch_data: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_pokedex_entry(
                $pokedex_id,
                PokedexEntry {
                    species: ROMString::from($species),
                    height: $height,
                    weight: $weight,
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

#[rustfmt::skip::macros(set_pokedex_entry_test)]

set_pokedex_entry_test!(set_pokedex_entry_1, 1, "ABCD", 13, 37, 0x40E33, vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]);
set_pokedex_entry_test!(
    set_pokedex_entry_2,
    2,
    "ABCD",
    13,
    37,
    0x40680,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_3,
    3,
    "ABCD",
    13,
    37,
    0x40E41,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_4,
    4,
    "ABCDEF",
    13,
    37,
    0x40F2F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_5,
    5,
    "ABCDE",
    13,
    37,
    0x40F53,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_6,
    6,
    "ABCDE",
    13,
    37,
    0x40F72,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_7,
    7,
    "ABCDEFGHIJ",
    13,
    37,
    0x40F3F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_8,
    8,
    "ABCDEF",
    13,
    37,
    0x40F62,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_9,
    9,
    "ABCDEFGHI",
    13,
    37,
    0x407C1,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_10,
    10,
    "ABCD",
    13,
    37,
    0x40CAF,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_11,
    11,
    "ABCDEF",
    13,
    37,
    0x40CBD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_12,
    12,
    "ABCDEFGHI",
    13,
    37,
    0x40CCD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_13,
    13,
    "ABCDEFGHI",
    13,
    37,
    0x40C1C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_14,
    14,
    "ABCDEF",
    13,
    37,
    0x40C2F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_15,
    15,
    "ABCDEFGHIJ",
    13,
    37,
    0x40C3F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_16,
    16,
    "ABCDEFGHI",
    13,
    37,
    0x40827,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_17,
    17,
    "ABCD",
    13,
    37,
    0x40E03,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_18,
    18,
    "ABCD",
    13,
    37,
    0x40E11,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_19,
    19,
    "ABC",
    13,
    37,
    0x40EAE,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_20,
    20,
    "ABC",
    13,
    37,
    0x40EBB,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_21,
    21,
    "ABCDEFGHI",
    13,
    37,
    0x4063C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_22,
    22,
    "ABCD",
    13,
    37,
    0x40819,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_23,
    23,
    "ABCDE",
    13,
    37,
    0x40BD9,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_24,
    24,
    "ABCDE",
    13,
    37,
    0x408B9,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_25,
    25,
    "ABCDE",
    13,
    37,
    0x40A8B,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_26,
    26,
    "ABCDE",
    13,
    37,
    0x40A9A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_27,
    27,
    "ABCDE",
    13,
    37,
    0x40B0F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_28,
    28,
    "ABCDE",
    13,
    37,
    0x40B1E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_29,
    29,
    "ABCDEFGHIJ",
    13,
    37,
    0x406DD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_30,
    30,
    "ABCDEFGHIJ",
    13,
    37,
    0x40EDC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_31,
    31,
    "ABCDE",
    13,
    37,
    0x406F1,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_32,
    32,
    "ABCDEFGHIJ",
    13,
    37,
    0x40619,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_33,
    33,
    "ABCDEFGHIJ",
    13,
    37,
    0x40EC8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_34,
    34,
    "ABCDE",
    13,
    37,
    0x4065D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_35,
    35,
    "ABCDE",
    13,
    37,
    0x4062D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_36,
    36,
    "ABCDE",
    13,
    37,
    0x40D91,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_37,
    37,
    "ABC",
    13,
    37,
    0x40A71,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_38,
    38,
    "ABC",
    13,
    37,
    0x40A7E,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_39,
    39,
    "ABCDEFG",
    13,
    37,
    0x40B4D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_40,
    40,
    "ABCDEFG",
    13,
    37,
    0x40B5E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_41,
    41,
    "ABC",
    13,
    37,
    0x40BCC,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_42,
    42,
    "ABC",
    13,
    37,
    0x40D14,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_43,
    43,
    "ABCD",
    13,
    37,
    0x40F81,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_44,
    44,
    "ABCD",
    13,
    37,
    0x40F8F,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_45,
    45,
    "ABCDEF",
    13,
    37,
    0x40F9D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_46,
    46,
    "ABCDEFGH",
    13,
    37,
    0x40BE8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_47,
    47,
    "ABCDEFGH",
    13,
    37,
    0x408C8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_48,
    48,
    "ABCDEF",
    13,
    37,
    0x409AD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_49,
    49,
    "ABCDEFGHIJ",
    13,
    37,
    0x40C89,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_50,
    50,
    "ABCD",
    13,
    37,
    0x40979,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_51,
    51,
    "ABCD",
    13,
    37,
    0x40C7B,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_52,
    52,
    "ABCDEFGHIJ",
    13,
    37,
    0x40A49,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_53,
    53,
    "ABCDEFGHIJ",
    13,
    37,
    0x40DB4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_54,
    54,
    "ABCD",
    13,
    37,
    0x408DA,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_55,
    55,
    "ABCD",
    13,
    37,
    0x40CF4,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_56,
    56,
    "ABCDEFGHIJ",
    13,
    37,
    0x40953,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_57,
    57,
    "ABCDEFGHIJ",
    13,
    37,
    0x40C67,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_58,
    58,
    "ABCDE",
    13,
    37,
    0x407F6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_59,
    59,
    "ABCDEFGHI",
    13,
    37,
    0x40733,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_60,
    60,
    "ABCDEFG",
    13,
    37,
    0x409E0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_61,
    61,
    "ABCDEFG",
    13,
    37,
    0x40BFA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_62,
    62,
    "ABCDEFG",
    13,
    37,
    0x40C0B,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_63,
    63,
    "ABC",
    13,
    37,
    0x40DE9,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_64,
    64,
    "ABC",
    13,
    37,
    0x40849,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_65,
    65,
    "ABC",
    13,
    37,
    0x40DF6,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_66,
    66,
    "ABCDEFGHIJ",
    13,
    37,
    0x40BB8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_67,
    67,
    "ABCDEFGHIJ",
    13,
    37,
    0x40871,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_68,
    68,
    "ABCDEFGHIJ",
    13,
    37,
    0x40CE0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_69,
    69,
    "ABCDEF",
    13,
    37,
    0x40FAD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_70,
    70,
    "ABCDEFGHIJ",
    13,
    37,
    0x40FBD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_71,
    71,
    "ABCDEFGHIJ",
    13,
    37,
    0x40FD1,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_72,
    72,
    "ABCDEFGHI",
    13,
    37,
    0x4077E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_73,
    73,
    "ABCDEFGHI",
    13,
    37,
    0x40E4F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_74,
    74,
    "ABCD",
    13,
    37,
    0x40EF0,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_75,
    75,
    "ABCD",
    13,
    37,
    0x40856,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_76,
    76,
    "ABCDEFG",
    13,
    37,
    0x408FA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_77,
    77,
    "ABCDEFGHIJ",
    13,
    37,
    0x40E86,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_78,
    78,
    "ABCDEFGHIJ",
    13,
    37,
    0x40E9A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_79,
    79,
    "ABCDE",
    13,
    37,
    0x4083A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_80,
    80,
    "ABCDEFGHIJ",
    13,
    37,
    0x4066C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_81,
    81,
    "ABCDEF",
    13,
    37,
    0x40F1F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_82,
    82,
    "ABCDEF",
    13,
    37,
    0x4092F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_83,
    83,
    "ABCDEFGHI",
    13,
    37,
    0x4099A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_84,
    84,
    "ABCDEFGHI",
    13,
    37,
    0x409CD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_85,
    85,
    "ABCDEFGHIJ",
    13,
    37,
    0x40C53,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_86,
    86,
    "ABCDEFGH",
    13,
    37,
    0x40967,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_87,
    87,
    "ABCDEFGH",
    13,
    37,
    0x40C9D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_88,
    88,
    "ABCDEF",
    13,
    37,
    0x406BD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_89,
    89,
    "ABCDEF",
    13,
    37,
    0x40D52,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_90,
    90,
    "ABCDEFG",
    13,
    37,
    0x4076D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_91,
    91,
    "ABCDEFG",
    13,
    37,
    0x40D72,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_92,
    92,
    "ABC",
    13,
    37,
    0x40791,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_93,
    93,
    "ABC",
    13,
    37,
    0x40DDC,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_94,
    94,
    "ABCDEF",
    13,
    37,
    0x406CD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_95,
    95,
    "ABCDEFGHIJ",
    13,
    37,
    0x40805,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_96,
    96,
    "ABCDEFGH",
    13,
    37,
    0x408E8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_97,
    97,
    "ABCDEFGH",
    13,
    37,
    0x40D02,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_98,
    98,
    "ABCDEFGHIJ",
    13,
    37,
    0x40A5D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_99,
    99,
    "ABCDEF",
    13,
    37,
    0x40D62,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_100,
    100,
    "ABCD",
    13,
    37,
    0x4064F,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_101,
    101,
    "ABCD",
    13,
    37,
    0x40D83,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_102,
    102,
    "ABC",
    13,
    37,
    0x406B0,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_103,
    103,
    "ABCDEFG",
    13,
    37,
    0x4068E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_104,
    104,
    "ABCDEF",
    13,
    37,
    0x40700,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_105,
    105,
    "ABCDEFGHIJ",
    13,
    37,
    0x40DC8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_106,
    106,
    "ABCDEFG",
    13,
    37,
    0x40896,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_107,
    107,
    "ABCDEFGH",
    13,
    37,
    0x408A7,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_108,
    108,
    "ABCDEFG",
    13,
    37,
    0x4069F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_109,
    109,
    "ABCDEFGHIJ",
    13,
    37,
    0x4093F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_110,
    110,
    "ABCDEFGHIJ",
    13,
    37,
    0x40DA0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_111,
    111,
    "ABCDEF",
    13,
    37,
    0x40710,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_112,
    112,
    "ABCDE",
    13,
    37,
    0x405FA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_113,
    113,
    "ABC",
    13,
    37,
    0x40864,
    vec![0x80, 0x81, 0x82, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_114,
    114,
    "ABCD",
    13,
    37,
    0x407E8,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_115,
    115,
    "ABCDEF",
    13,
    37,
    0x40609,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_116,
    116,
    "ABCDEF",
    13,
    37,
    0x40AEF,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_117,
    117,
    "ABCDEF",
    13,
    37,
    0x40AFF,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_118,
    118,
    "ABCDEFGH",
    13,
    37,
    0x40E62,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_119,
    119,
    "ABCDEFGH",
    13,
    37,
    0x40E74,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_120,
    120,
    "ABCDEFGHI",
    13,
    37,
    0x407AE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_121,
    121,
    "ABCDEFGHIJ",
    13,
    37,
    0x40E1F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_122,
    122,
    "ABCDEFG",
    13,
    37,
    0x40885,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_123,
    123,
    "ABCDEF",
    13,
    37,
    0x4079E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_124,
    124,
    "ABCDEFGHIJ",
    13,
    37,
    0x409F1,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_125,
    125,
    "ABCDEFGH",
    13,
    37,
    0x4091D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_126,
    126,
    "ABCDEFGH",
    13,
    37,
    0x4090B,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_127,
    127,
    "ABCDEFGHIJ",
    13,
    37,
    0x407D4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_128,
    128,
    "ABCDEFGHI",
    13,
    37,
    0x40987,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_129,
    129,
    "ABCD",
    13,
    37,
    0x40D44,
    vec![0x80, 0x81, 0x82, 0x83, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_130,
    130,
    "ABCDEFGHI",
    13,
    37,
    0x4075A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_131,
    131,
    "ABCDEFGHI",
    13,
    37,
    0x40720,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_132,
    132,
    "ABCDEFGHI",
    13,
    37,
    0x40A36,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_133,
    133,
    "ABCDEFGHI",
    13,
    37,
    0x40B6F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_134,
    134,
    "ABCDEFGHIJ",
    13,
    37,
    0x40BA4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_135,
    135,
    "ABCDEFGHI",
    13,
    37,
    0x40B91,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_136,
    136,
    "ABCDE",
    13,
    37,
    0x40B82,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_137,
    137,
    "ABCDEFG",
    13,
    37,
    0x40EFE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_138,
    138,
    "ABCDEF",
    13,
    37,
    0x40B2D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_139,
    139,
    "ABCDEF",
    13,
    37,
    0x40B3D,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_140,
    140,
    "ABCDEFGHI",
    13,
    37,
    0x40AC9,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_141,
    141,
    "ABCDEFGHI",
    13,
    37,
    0x40ADC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_142,
    142,
    "ABCDEF",
    13,
    37,
    0x40F0F,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_143,
    143,
    "ABCDEFGH",
    13,
    37,
    0x40D32,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_144,
    144,
    "ABCDEF",
    13,
    37,
    0x40A14,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_145,
    145,
    "ABCDEFGH",
    13,
    37,
    0x40A24,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_146,
    146,
    "ABCDE",
    13,
    37,
    0x40A05,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_147,
    147,
    "ABCDEF",
    13,
    37,
    0x40AA9,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_148,
    148,
    "ABCDEF",
    13,
    37,
    0x40AB9,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_149,
    149,
    "ABCDEF",
    13,
    37,
    0x409BD,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_150,
    150,
    "ABCDEFG",
    13,
    37,
    0x40D21,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x50, 1, 1, 37, 0]
);
set_pokedex_entry_test!(
    set_pokedex_entry_151,
    151,
    "ABCDEFGHIJ",
    13,
    37,
    0x40746,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87, 0x88, 0x89, 0x50, 1, 1, 37, 0]
);
