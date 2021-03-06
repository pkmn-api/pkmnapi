use pkmnapi_db::patch::*;
use pkmnapi_db::string::*;
use pkmnapi_db::*;

mod common;

macro_rules! set_pokemon_name_test {
    (
        $test_name:ident,
        $pokedex_id:expr,
        $pokemon_name:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_pokemon_name(
                &$pokedex_id,
                &PokemonName {
                    name: ROMString::from($pokemon_name),
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

set_pokemon_name_test!(
    set_pokemon_name_1,
    1,
    "ABCDE",
    0x1C80E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_2,
    2,
    "ABCDE",
    0x1C26E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_3,
    3,
    "ABCDE",
    0x1C818,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_4,
    4,
    "ABCDE",
    0x1C8F4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_5,
    5,
    "ABCDE",
    0x1C908,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_6,
    6,
    "ABCDE",
    0x1C91C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_7,
    7,
    "ABCDE",
    0x1C8FE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_8,
    8,
    "ABCDE",
    0x1C912,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_9,
    9,
    "ABCDE",
    0x1C32C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_10,
    10,
    "ABCDE",
    0x1C6E2,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_11,
    11,
    "ABCDE",
    0x1C6EC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_12,
    12,
    "ABCDE",
    0x1C6F6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_13,
    13,
    "ABCDE",
    0x1C674,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_14,
    14,
    "ABCDE",
    0x1C67E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_15,
    15,
    "ABCDE",
    0x1C688,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_16,
    16,
    "ABCDE",
    0x1C37C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_17,
    17,
    "ABCDE",
    0x1C7F0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_18,
    18,
    "ABCDE",
    0x1C7FA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_19,
    19,
    "ABCDE",
    0x1C886,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_20,
    20,
    "ABCDE",
    0x1C890,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_21,
    21,
    "ABCDE",
    0x1C246,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_22,
    22,
    "ABCDE",
    0x1C372,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_23,
    23,
    "ABCDE",
    0x1C64C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_24,
    24,
    "ABCDE",
    0x1C3D6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_25,
    25,
    "ABCDE",
    0x1C55C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_26,
    26,
    "ABCDE",
    0x1C566,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_27,
    27,
    "ABCDE",
    0x1C5D4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_28,
    28,
    "ABCDE",
    0x1C5DE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_29,
    29,
    "ABCDE",
    0x1C2AA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_30,
    30,
    "ABCDE",
    0x1C8A4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_31,
    31,
    "ABCDE",
    0x1C2B4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_32,
    32,
    "ABCDE",
    0x1C232,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_33,
    33,
    "ABCDE",
    0x1C89A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_34,
    34,
    "ABCDE",
    0x1C25A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_35,
    35,
    "ABCDE",
    0x1C23C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_36,
    36,
    "ABCDE",
    0x1C7A0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_37,
    37,
    "ABCDE",
    0x1C548,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_38,
    38,
    "ABCDE",
    0x1C552,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_39,
    39,
    "ABCDE",
    0x1C5FC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_40,
    40,
    "ABCDE",
    0x1C606,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_41,
    41,
    "ABCDE",
    0x1C642,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_42,
    42,
    "ABCDE",
    0x1C728,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_43,
    43,
    "ABCDE",
    0x1C94E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_44,
    44,
    "ABCDE",
    0x1C958,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_45,
    45,
    "ABCDE",
    0x1C962,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_46,
    46,
    "ABCDE",
    0x1C656,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_47,
    47,
    "ABCDE",
    0x1C3E0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_48,
    48,
    "ABCDE",
    0x1C49E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_49,
    49,
    "ABCDE",
    0x1C6BA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_50,
    50,
    "ABCDE",
    0x1C462,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_51,
    51,
    "ABCDE",
    0x1C6B0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_52,
    52,
    "ABCDE",
    0x1C516,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_53,
    53,
    "ABCDE",
    0x1C7B4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_54,
    54,
    "ABCDE",
    0x1C3EA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_55,
    55,
    "ABCDE",
    0x1C714,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_56,
    56,
    "ABCDE",
    0x1C44E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_57,
    57,
    "ABCDE",
    0x1C6A6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_58,
    58,
    "ABCDE",
    0x1C35E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_59,
    59,
    "ABCDE",
    0x1C2DC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_60,
    60,
    "ABCDE",
    0x1C4DA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_61,
    61,
    "ABCDE",
    0x1C660,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_62,
    62,
    "ABCDE",
    0x1C66A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_63,
    63,
    "ABCDE",
    0x1C7DC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_64,
    64,
    "ABCDE",
    0x1C390,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_65,
    65,
    "ABCDE",
    0x1C7E6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_66,
    66,
    "ABCDE",
    0x1C638,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_67,
    67,
    "ABCDE",
    0x1C3AE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_68,
    68,
    "ABCDE",
    0x1C700,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_69,
    69,
    "ABCDE",
    0x1C96C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_70,
    70,
    "ABCDE",
    0x1C976,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_71,
    71,
    "ABCDE",
    0x1C980,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_72,
    72,
    "ABCDE",
    0x1C304,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_73,
    73,
    "ABCDE",
    0x1C822,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_74,
    74,
    "ABCDE",
    0x1C8AE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_75,
    75,
    "ABCDE",
    0x1C39A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_76,
    76,
    "ABCDE",
    0x1C3FE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_77,
    77,
    "ABCDE",
    0x1C872,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_78,
    78,
    "ABCDE",
    0x1C87C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_79,
    79,
    "ABCDE",
    0x1C386,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_80,
    80,
    "ABCDE",
    0x1C264,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_81,
    81,
    "ABCDE",
    0x1C8D6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_82,
    82,
    "ABCDE",
    0x1C430,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_83,
    83,
    "ABCDE",
    0x1C494,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_84,
    84,
    "ABCDE",
    0x1C4D0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_85,
    85,
    "ABCDE",
    0x1C69C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_86,
    86,
    "ABCDE",
    0x1C458,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_87,
    87,
    "ABCDE",
    0x1C6C4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_88,
    88,
    "ABCDE",
    0x1C296,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_89,
    89,
    "ABCDE",
    0x1C764,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_90,
    90,
    "ABCDE",
    0x1C2FA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_91,
    91,
    "ABCDE",
    0x1C782,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_92,
    92,
    "ABCDE",
    0x1C30E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_93,
    93,
    "ABCDE",
    0x1C7D2,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_94,
    94,
    "ABCDE",
    0x1C2A0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_95,
    95,
    "ABCDE",
    0x1C368,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_96,
    96,
    "ABCDE",
    0x1C3F4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_97,
    97,
    "ABCDE",
    0x1C71E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_98,
    98,
    "ABCDE",
    0x1C520,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_99,
    99,
    "ABCDE",
    0x1C778,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_100,
    100,
    "ABCDE",
    0x1C250,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_101,
    101,
    "ABCDE",
    0x1C796,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_102,
    102,
    "ABCDE",
    0x1C28C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_103,
    103,
    "ABCDE",
    0x1C278,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_104,
    104,
    "ABCDE",
    0x1C2BE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_105,
    105,
    "ABCDE",
    0x1C7BE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_106,
    106,
    "ABCDE",
    0x1C3C2,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_107,
    107,
    "ABCDE",
    0x1C3CC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_108,
    108,
    "ABCDE",
    0x1C282,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_109,
    109,
    "ABCDE",
    0x1C43A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_110,
    110,
    "ABCDE",
    0x1C7AA,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_111,
    111,
    "ABCDE",
    0x1C2C8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_112,
    112,
    "ABCDE",
    0x1C21E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_113,
    113,
    "ABCDE",
    0x1C3A4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_114,
    114,
    "ABCDE",
    0x1C340,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_115,
    115,
    "ABCDE",
    0x1C228,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_116,
    116,
    "ABCDE",
    0x1C5AC,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_117,
    117,
    "ABCDE",
    0x1C5B6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_118,
    118,
    "ABCDE",
    0x1C836,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_119,
    119,
    "ABCDE",
    0x1C840,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_120,
    120,
    "ABCDE",
    0x1C322,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_121,
    121,
    "ABCDE",
    0x1C804,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_122,
    122,
    "ABCDE",
    0x1C3B8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_123,
    123,
    "ABCDE",
    0x1C318,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_124,
    124,
    "ABCDE",
    0x1C4E4,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_125,
    125,
    "ABCDE",
    0x1C426,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_126,
    126,
    "ABCDE",
    0x1C412,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_127,
    127,
    "ABCDE",
    0x1C336,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_128,
    128,
    "ABCDE",
    0x1C46C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_129,
    129,
    "ABCDE",
    0x1C746,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_130,
    130,
    "ABCDE",
    0x1C2F0,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_131,
    131,
    "ABCDE",
    0x1C2D2,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_132,
    132,
    "ABCDE",
    0x1C50C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_133,
    133,
    "ABCDE",
    0x1C610,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_134,
    134,
    "ABCDE",
    0x1C62E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_135,
    135,
    "ABCDE",
    0x1C624,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_136,
    136,
    "ABCDE",
    0x1C61A,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_137,
    137,
    "ABCDE",
    0x1C8B8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_138,
    138,
    "ABCDE",
    0x1C5E8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_139,
    139,
    "ABCDE",
    0x1C5F2,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_140,
    140,
    "ABCDE",
    0x1C598,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_141,
    141,
    "ABCDE",
    0x1C5A2,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_142,
    142,
    "ABCDE",
    0x1C8C2,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_143,
    143,
    "ABCDE",
    0x1C73C,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_144,
    144,
    "ABCDE",
    0x1C4F8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_145,
    145,
    "ABCDE",
    0x1C502,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_146,
    146,
    "ABCDE",
    0x1C4EE,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_147,
    147,
    "ABCDE",
    0x1C584,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_148,
    148,
    "ABCDE",
    0x1C58E,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_149,
    149,
    "ABCDE",
    0x1C4A8,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_150,
    150,
    "ABCDE",
    0x1C732,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
set_pokemon_name_test!(
    set_pokemon_name_151,
    151,
    "ABCDE",
    0x1C2E6,
    vec![0x80, 0x81, 0x82, 0x83, 0x84, 0x50, 0x50, 0x50, 0x50, 0x50]
);
