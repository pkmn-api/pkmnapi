use pkmnapi_db::patch::*;
use pkmnapi_db::*;

mod common;

macro_rules! set_pokemon_learnset_test {
    (
        $test_name:ident,
        $pokedex_id:expr,
        $pokemon_learnset:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_pokemon_learnset(&$pokedex_id, &$pokemon_learnset) {
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

set_pokemon_learnset_test!(
    set_pokemon_learnset_1,
    1,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B848,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_2,
    2,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B24F,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_3,
    3,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B858,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_4,
    4,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B93C,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_5,
    5,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B95E,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_6,
    6,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B97D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_7,
    7,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B94D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_8,
    8,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B96F,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_9,
    9,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B347,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_10,
    10,
    vec![] as Vec<PokemonLearnset>,
    0x3B746,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_11,
    11,
    vec![] as Vec<PokemonLearnset>,
    0x3B74B,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_12,
    12,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B74D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_13,
    13,
    vec![] as Vec<PokemonLearnset>,
    0x3B6E8,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_14,
    14,
    vec![] as Vec<PokemonLearnset>,
    0x3B6ED,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_15,
    15,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B6EF,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_16,
    16,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B3A0,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_17,
    17,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B827,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_18,
    18,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B835,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_19,
    19,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B8C5,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_20,
    20,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B8CF,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_21,
    21,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B219,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_22,
    22,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B391,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_23,
    23,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B6B2,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_24,
    24,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B422,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_25,
    25,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B597,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_26,
    26,
    vec![] as Vec<PokemonLearnset>,
    0x3B5A3,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_27,
    27,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B600,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_28,
    28,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B60C,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_29,
    29,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B29B,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_30,
    30,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B8EF,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_31,
    31,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B2A9,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_32,
    32,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B1F6,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_33,
    33,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B8DD,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_34,
    34,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B234,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_35,
    35,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B208,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_36,
    36,
    vec![] as Vec<PokemonLearnset>,
    0x3B7E0,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_37,
    37,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B585,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_38,
    38,
    vec![] as Vec<PokemonLearnset>,
    0x3B591,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_39,
    39,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B633,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_40,
    40,
    vec![] as Vec<PokemonLearnset>,
    0x3B643,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_41,
    41,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B6A3,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_42,
    42,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B785,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_43,
    43,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B996,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_44,
    44,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B9A8,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_45,
    45,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B9B6,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_46,
    46,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B6C1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_47,
    47,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B42E,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_48,
    48,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B4F3,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_49,
    49,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B725,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_50,
    50,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B4C6,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_51,
    51,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B719,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_52,
    52,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B560,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_53,
    53,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B7EE,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_54,
    54,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B43D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_55,
    55,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B76B,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_56,
    56,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B4A8,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_57,
    57,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B70D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_58,
    58,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B379,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_59,
    59,
    vec![] as Vec<PokemonLearnset>,
    0x3B2DF,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_60,
    60,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B527,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_61,
    61,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B6D1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_62,
    62,
    vec![PokemonLearnset::new(1, 2), PokemonLearnset::new(1, 2),],
    0x3B6DF,
    vec![0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_63,
    63,
    vec![] as Vec<PokemonLearnset>,
    0x3B814,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_64,
    64,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B3C2,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_65,
    65,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B816,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_66,
    66,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B694,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_67,
    67,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B3F2,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_68,
    68,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B75D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_69,
    69,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B9C1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_70,
    70,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B9D5,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_71,
    71,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B9E5,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_72,
    72,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B30A,
    vec![
        0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01,
        0x02,
    ]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_73,
    73,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B868,
    vec![
        0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01,
        0x02,
    ]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_74,
    74,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B900,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_75,
    75,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B3D3,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_76,
    76,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B45A,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_77,
    77,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B8A6,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_78,
    78,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B8B4,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_79,
    79,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B3B1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_80,
    80,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B23C,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_81,
    81,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B927,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_82,
    82,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B486,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_83,
    83,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B4E4,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_84,
    84,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B516,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_85,
    85,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B6FF,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_86,
    86,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B4B7,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_87,
    87,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B733,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_88,
    88,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B284,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_89,
    89,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B7B2,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_90,
    90,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B2FB,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_91,
    91,
    vec![PokemonLearnset::new(1, 2),],
    0x3B7CE,
    vec![0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_92,
    92,
    vec![PokemonLearnset::new(1, 2), PokemonLearnset::new(1, 2),],
    0x3B31F,
    vec![0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_93,
    93,
    vec![PokemonLearnset::new(1, 2), PokemonLearnset::new(1, 2),],
    0x3B80B,
    vec![0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_94,
    94,
    vec![PokemonLearnset::new(1, 2), PokemonLearnset::new(1, 2),],
    0x3B292,
    vec![0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_95,
    95,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B385,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_96,
    96,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B44C,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_97,
    97,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B777,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_98,
    98,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B56F,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_99,
    99,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B7C2,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_100,
    100,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B228,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_101,
    101,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B7D4,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_102,
    102,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B273,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_103,
    103,
    vec![PokemonLearnset::new(1, 2),],
    0x3B25F,
    vec![0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_104,
    104,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B2B4,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_105,
    105,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B7FA,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_106,
    106,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B40A,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_107,
    107,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B416,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_108,
    108,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B263,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_109,
    109,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B497,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_110,
    110,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B7E2,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_111,
    111,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B2C3,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_112,
    112,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B1D9,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_113,
    113,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B3E1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_114,
    114,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B363,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_115,
    115,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B1E7,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_116,
    116,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B5E1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_117,
    117,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B5ED,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_118,
    118,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B87F,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_119,
    119,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B88D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_120,
    120,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B337,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_121,
    121,
    vec![] as Vec<PokemonLearnset>,
    0x3B843,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_122,
    122,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B3FE,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_123,
    123,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B325,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_124,
    124,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B535,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_125,
    125,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B47A,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_126,
    126,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B46A,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_127,
    127,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B355,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_128,
    128,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B4D2,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_129,
    129,
    vec![PokemonLearnset::new(1, 2),],
    0x3B7AA,
    vec![0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_130,
    130,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B2EB,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_131,
    131,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B2D1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_132,
    132,
    vec![] as Vec<PokemonLearnset>,
    0x3B55B,
    vec![] as Vec<u8>
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_133,
    133,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B651,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_134,
    134,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B67F,
    vec![
        0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01,
        0x02,
    ]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_135,
    135,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B66D,
    vec![
        0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01,
        0x02,
    ]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_136,
    136,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B65B,
    vec![
        0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01,
        0x02,
    ]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_137,
    137,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B90E,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_138,
    138,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B61B,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_139,
    139,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B625,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_140,
    140,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B5CA,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_141,
    141,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B5D4,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_142,
    142,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B918,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_143,
    143,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B79D,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_144,
    144,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B54B,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_145,
    145,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B553,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_146,
    146,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B543,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_147,
    147,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B5AC,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_148,
    148,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B5BB,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_149,
    149,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B501,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_150,
    150,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B791,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
set_pokemon_learnset_test!(
    set_pokemon_learnset_151,
    151,
    vec![
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
        PokemonLearnset::new(1, 2),
    ],
    0x3B2E1,
    vec![0x01, 0x02, 0x01, 0x02, 0x01, 0x02, 0x01, 0x02,]
);
