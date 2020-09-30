use pkmnapi_db::patch::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! set_pokemon_evolutions_test {
    ($test_name: ident, $pokedex_id: expr, $pokemon_evolutions: expr, $patch_offset: expr, $patch_data: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_pokemon_evolutions(&$pokedex_id, &$pokemon_evolutions) {
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

#[rustfmt::skip::macros(set_pokemon_evolutions_test)]

set_pokemon_evolutions_test!(set_pokemon_evolutions_1, 1, vec![PokemonEvolutionLevel::new(112, 42)], 0x3B844, vec![0x01, 0x2A, 0x01]);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_2,
    2,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B24B,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_3,
    3,
    vec![],
    0x3B857,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_4,
    4,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B938,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_5,
    5,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B95A,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_6,
    6,
    vec![],
    0x3B97C,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_7,
    7,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B949,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_8,
    8,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B96B,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_9,
    9,
    vec![],
    0x3B346,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_10,
    10,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B742,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_11,
    11,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B747,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_12,
    12,
    vec![],
    0x3B74C,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_13,
    13,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B6E4,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_14,
    14,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B6E9,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_15,
    15,
    vec![],
    0x3B6EE,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_16,
    16,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B39C,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_17,
    17,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B823,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_18,
    18,
    vec![],
    0x3B834,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_19,
    19,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B8C1,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_20,
    20,
    vec![],
    0x3B8CE,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_21,
    21,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B215,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_22,
    22,
    vec![],
    0x3B390,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_23,
    23,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B6AE,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_24,
    24,
    vec![],
    0x3B421,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_25,
    25,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B592,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_26,
    26,
    vec![],
    0x3B5A2,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_27,
    27,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B5FC,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_28,
    28,
    vec![],
    0x3B60B,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_29,
    29,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B297,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_30,
    30,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B8EA,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_31,
    31,
    vec![],
    0x3B2A8,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_32,
    32,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B1F2,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_33,
    33,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B8D8,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_34,
    34,
    vec![],
    0x3B233,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_35,
    35,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B203,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_36,
    36,
    vec![],
    0x3B7DF,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_37,
    37,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B580,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_38,
    38,
    vec![],
    0x3B590,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_39,
    39,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B62E,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_40,
    40,
    vec![],
    0x3B642,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_41,
    41,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B69F,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_42,
    42,
    vec![],
    0x3B784,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_43,
    43,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B992,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_44,
    44,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B9A3,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_45,
    45,
    vec![],
    0x3B9B5,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_46,
    46,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B6BD,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_47,
    47,
    vec![],
    0x3B42D,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_48,
    48,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B4EF,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_49,
    49,
    vec![],
    0x3B724,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_50,
    50,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B4C2,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_51,
    51,
    vec![],
    0x3B718,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_52,
    52,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B55C,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_53,
    53,
    vec![],
    0x3B7ED,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_54,
    54,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B439,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_55,
    55,
    vec![],
    0x3B76A,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_56,
    56,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B4A4,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_57,
    57,
    vec![],
    0x3B70C,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_58,
    58,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B374,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_59,
    59,
    vec![],
    0x3B2DE,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_60,
    60,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B523,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_61,
    61,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B6CC,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_62,
    62,
    vec![],
    0x3B6DE,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_63,
    63,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B810,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_64,
    64,
    vec![PokemonEvolutionTrade::new(112)],
    0x3B3BE,
    vec![0x03, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_65,
    65,
    vec![],
    0x3B815,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_66,
    66,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B690,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_67,
    67,
    vec![PokemonEvolutionTrade::new(112)],
    0x3B3EE,
    vec![0x03, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_68,
    68,
    vec![],
    0x3B75C,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_69,
    69,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B9BD,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_70,
    70,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B9D0,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_71,
    71,
    vec![],
    0x3B9E4,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_72,
    72,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B306,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_73,
    73,
    vec![],
    0x3B867,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_74,
    74,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B8FC,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_75,
    75,
    vec![PokemonEvolutionTrade::new(112)],
    0x3B3CF,
    vec![0x03, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_76,
    76,
    vec![],
    0x3B459,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_77,
    77,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B8A2,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_78,
    78,
    vec![],
    0x3B8B3,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_79,
    79,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B3AD,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_80,
    80,
    vec![],
    0x3B23B,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_81,
    81,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B923,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_82,
    82,
    vec![],
    0x3B485,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_83,
    83,
    vec![],
    0x3B4E3,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_84,
    84,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B512,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_85,
    85,
    vec![],
    0x3B6FE,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_86,
    86,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B4B3,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_87,
    87,
    vec![],
    0x3B732,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_88,
    88,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B280,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_89,
    89,
    vec![],
    0x3B7B1,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_90,
    90,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B2F6,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_91,
    91,
    vec![],
    0x3B7CD,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_92,
    92,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B31B,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_93,
    93,
    vec![PokemonEvolutionTrade::new(112)],
    0x3B807,
    vec![0x03, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_94,
    94,
    vec![],
    0x3B291,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_95,
    95,
    vec![],
    0x3B384,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_96,
    96,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B448,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_97,
    97,
    vec![],
    0x3B776,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_98,
    98,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B56B,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_99,
    99,
    vec![],
    0x3B7C1,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_100,
    100,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B224,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_101,
    101,
    vec![],
    0x3B7D3,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_102,
    102,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B26E,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_103,
    103,
    vec![],
    0x3B25E,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_104,
    104,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B2B0,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_105,
    105,
    vec![],
    0x3B7F9,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_106,
    106,
    vec![],
    0x3B409,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_107,
    107,
    vec![],
    0x3B415,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_108,
    108,
    vec![],
    0x3B262,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_109,
    109,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B493,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_110,
    110,
    vec![],
    0x3B7E1,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_111,
    111,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B2BF,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_112,
    112,
    vec![],
    0x3B1D8,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_113,
    113,
    vec![],
    0x3B3E0,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_114,
    114,
    vec![],
    0x3B362,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_115,
    115,
    vec![],
    0x3B1E6,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_116,
    116,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B5DD,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_117,
    117,
    vec![],
    0x3B5EC,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_118,
    118,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B87B,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_119,
    119,
    vec![],
    0x3B88C,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_120,
    120,
    vec![PokemonEvolutionItem::new(112, 10)],
    0x3B332,
    vec![0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_121,
    121,
    vec![],
    0x3B842,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_122,
    122,
    vec![],
    0x3B3FD,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_123,
    123,
    vec![],
    0x3B324,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_124,
    124,
    vec![],
    0x3B534,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_125,
    125,
    vec![],
    0x3B479,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_126,
    126,
    vec![],
    0x3B469,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_127,
    127,
    vec![],
    0x3B354,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_128,
    128,
    vec![],
    0x3B4D1,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_129,
    129,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B7A6,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_130,
    130,
    vec![],
    0x3B2EA,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_131,
    131,
    vec![],
    0x3B2D0,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_132,
    132,
    vec![],
    0x3B55A,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_133,
    133,
    vec![
        PokemonEvolutionItem::new(112, 10),
        PokemonEvolutionItem::new(112, 10),
        PokemonEvolutionItem::new(112, 10),
    ],
    0x3B644,
    vec![0x02, 0x0A, 0x01, 0x01, 0x02, 0x0A, 0x01, 0x01, 0x02, 0x0A, 0x01, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_134,
    134,
    vec![],
    0x3B67E,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_135,
    135,
    vec![],
    0x3B66C,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_136,
    136,
    vec![],
    0x3B65A,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_137,
    137,
    vec![],
    0x3B90D,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_138,
    138,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B617,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_139,
    139,
    vec![],
    0x3B624,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_140,
    140,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B5C6,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_141,
    141,
    vec![],
    0x3B5D3,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_142,
    142,
    vec![],
    0x3B917,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_143,
    143,
    vec![],
    0x3B79C,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_144,
    144,
    vec![],
    0x3B54A,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_145,
    145,
    vec![],
    0x3B552,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_146,
    146,
    vec![],
    0x3B542,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_147,
    147,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B5A8,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_148,
    148,
    vec![PokemonEvolutionLevel::new(112, 42)],
    0x3B5B7,
    vec![0x01, 0x2A, 0x01]
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_149,
    149,
    vec![],
    0x3B500,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_150,
    150,
    vec![],
    0x3B790,
    vec![] as Vec<u8>
);
set_pokemon_evolutions_test!(
    set_pokemon_evolutions_151,
    151,
    vec![],
    0x3B2E0,
    vec![] as Vec<u8>
);
