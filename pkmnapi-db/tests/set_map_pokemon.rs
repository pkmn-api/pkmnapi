use pkmnapi_db::patch::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! set_map_pokemon_test {
    (
        $test_name:ident,
        $map_id:expr,
        $grass_encounter_rate:expr,
        $grass_pokemon:expr,
        $water_encounter_rate:expr,
        $water_pokemon:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_map_pokemon(
                &$map_id,
                &MapPokemon {
                    grass: MapPokemonArea {
                        encounter_rate: $grass_encounter_rate,
                        pokemon: $grass_pokemon,
                    },
                    water: MapPokemonArea {
                        encounter_rate: $water_encounter_rate,
                        pokemon: $water_pokemon,
                    },
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for map ID: {}",
                    $map_id
                ),
                Err(_) => panic!(format!("Could not find map ID: {}", $map_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(set_map_pokemon_test)]

set_map_pokemon_test!(set_map_pokemon_0, 0, 0, vec![], 0, vec![], 0xD0DD, vec![0x00, 0x00]);
set_map_pokemon_test!(
    set_map_pokemon_1,
    1,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_2,
    2,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_3,
    3,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_4,
    4,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_5,
    5,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_6,
    6,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_7,
    7,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_8,
    8,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_9,
    9,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_10,
    10,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_11,
    11,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_12,
    12,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD0DF,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_13,
    13,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD0F5,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_14,
    14,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD137,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_15,
    15,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD18F,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_16,
    16,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD1E7,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_17,
    17,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD1FD,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_18,
    18,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD297,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_19,
    19,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD281,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_20,
    20,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD1D1,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_21,
    21,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD255,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_22,
    22,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD213,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_23,
    23,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD26B,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_24,
    24,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD31F,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_25,
    25,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD335,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_26,
    26,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD34B,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_27,
    27,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD361,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_28,
    28,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD377,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_29,
    29,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD38D,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_30,
    30,
    0,
    vec![],
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0xD3FB,
    vec![
        0x00, 0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A,
        0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_31,
    31,
    0,
    vec![],
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0xD3FB,
    vec![
        0x00, 0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A,
        0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_32,
    32,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0xD4D7,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_33,
    33,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD10B,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_34,
    34,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD559,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_35,
    35,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD1A5,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_36,
    36,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD1BB,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_37,
    37,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_38,
    38,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_39,
    39,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_40,
    40,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_41,
    41,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_42,
    42,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_43,
    43,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_44,
    44,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_45,
    45,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_46,
    46,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_47,
    47,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_48,
    48,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_49,
    49,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_50,
    50,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_51,
    51,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD121,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_52,
    52,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_53,
    53,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_54,
    54,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_55,
    55,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_56,
    56,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_57,
    57,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_58,
    58,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_59,
    59,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD14D,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_60,
    60,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD163,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_61,
    61,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD179,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_62,
    62,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_63,
    63,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_64,
    64,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_65,
    65,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_66,
    66,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_67,
    67,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_68,
    68,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_69,
    69,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_70,
    70,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_71,
    71,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_72,
    72,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_73,
    73,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_74,
    74,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_75,
    75,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_76,
    76,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_77,
    77,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_78,
    78,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_79,
    79,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_80,
    80,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_81,
    81,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_82,
    82,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD229,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_83,
    83,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD543,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_84,
    84,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_85,
    85,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_86,
    86,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_87,
    87,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_88,
    88,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_89,
    89,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_90,
    90,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_91,
    91,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_92,
    92,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_93,
    93,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_94,
    94,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_95,
    95,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_96,
    96,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_97,
    97,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_98,
    98,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_99,
    99,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_100,
    100,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_101,
    101,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_102,
    102,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_103,
    103,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_104,
    104,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_105,
    105,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_106,
    106,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_107,
    107,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_108,
    108,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD59B,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_109,
    109,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_110,
    110,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_111,
    111,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_112,
    112,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_113,
    113,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_114,
    114,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_115,
    115,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_116,
    116,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_117,
    117,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_118,
    118,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_119,
    119,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_120,
    120,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_121,
    121,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_122,
    122,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_123,
    123,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_124,
    124,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_125,
    125,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_126,
    126,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_127,
    127,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_128,
    128,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_129,
    129,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_130,
    130,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_131,
    131,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_132,
    132,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_133,
    133,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_134,
    134,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_135,
    135,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_136,
    136,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_137,
    137,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_138,
    138,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_139,
    139,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_140,
    140,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_141,
    141,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_142,
    142,
    0,
    vec![],
    0,
    vec![],
    0xD2AD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_143,
    143,
    0,
    vec![],
    0,
    vec![],
    0xD2AF,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_144,
    144,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD2B1,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_145,
    145,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD2C7,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_146,
    146,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD2DD,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_147,
    147,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD2F3,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_148,
    148,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD309,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_149,
    149,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_150,
    150,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_151,
    151,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_152,
    152,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_153,
    153,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_154,
    154,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_155,
    155,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_156,
    156,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_157,
    157,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_158,
    158,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_159,
    159,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD427,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_160,
    160,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD43D,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_161,
    161,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD453,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_162,
    162,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD469,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_163,
    163,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_164,
    164,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_165,
    165,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD47F,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_166,
    166,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_167,
    167,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_168,
    168,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_169,
    169,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_170,
    170,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_171,
    171,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_172,
    172,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_173,
    173,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_174,
    174,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_175,
    175,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_176,
    176,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_177,
    177,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_178,
    178,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_179,
    179,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_180,
    180,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_181,
    181,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_182,
    182,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_183,
    183,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_184,
    184,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_185,
    185,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_186,
    186,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_187,
    187,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_188,
    188,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_189,
    189,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_190,
    190,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_191,
    191,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_192,
    192,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD411,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_193,
    193,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_194,
    194,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD56F,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_195,
    195,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_196,
    196,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_197,
    197,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD5B1,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_198,
    198,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD585,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_199,
    199,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_200,
    200,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_201,
    201,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_202,
    202,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_203,
    203,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_204,
    204,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_205,
    205,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_206,
    206,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_207,
    207,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_208,
    208,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_209,
    209,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_210,
    210,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_211,
    211,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_212,
    212,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_213,
    213,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_214,
    214,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD495,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_215,
    215,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD4AB,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_216,
    216,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD4C1,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_217,
    217,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD3B9,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_218,
    218,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD3CF,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_219,
    219,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD3E5,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_220,
    220,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD3A3,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_221,
    221,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_222,
    222,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_223,
    223,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_224,
    224,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_225,
    225,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_226,
    226,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD517,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_227,
    227,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD52D,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_228,
    228,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD501,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_229,
    229,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_230,
    230,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_231,
    231,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_232,
    232,
    25,
    vec![
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
        MapPokemonInfo::new(10, 112),
    ],
    0,
    vec![],
    0xD23F,
    vec![
        0x19, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01,
        0x0A, 0x01, 0x0A, 0x01, 0x0A, 0x01, 0x00
    ]
);
set_map_pokemon_test!(
    set_map_pokemon_233,
    233,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_234,
    234,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_235,
    235,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_236,
    236,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_237,
    237,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_238,
    238,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_239,
    239,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_240,
    240,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_241,
    241,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_242,
    242,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_243,
    243,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_244,
    244,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_245,
    245,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_246,
    246,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
set_map_pokemon_test!(
    set_map_pokemon_247,
    247,
    0,
    vec![],
    0,
    vec![],
    0xD0DD,
    vec![0x00, 0x00]
);
