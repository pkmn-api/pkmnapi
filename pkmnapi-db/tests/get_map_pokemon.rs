use pkmnapi_db::types::*;

mod common;

macro_rules! get_map_pokemon_test {
    (
        $test_name:ident,
        $map_id:expr,
        $grass_encounter_rate:expr,
        $grass_pokemon:expr,
        $water_encounter_rate:expr,
        $water_pokemon:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_map_pokemon(&$map_id) {
                Ok(map_pokemon) => assert_eq!(
                    map_pokemon,
                    MapPokemon {
                        grass: MapPokemonArea {
                            encounter_rate: $grass_encounter_rate,
                            pokemon: $grass_pokemon
                        },
                        water: MapPokemonArea {
                            encounter_rate: $water_encounter_rate,
                            pokemon: $water_pokemon
                        },
                    },
                    "Searched for map ID: {}",
                    $map_id
                ),
                Err(_) => panic!(format!("Could not find map ID: {}", $map_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_map_pokemon_test)]

get_map_pokemon_test!(get_map_pokemon_0, 0, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_1, 1, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_2, 2, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_3, 3, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_4, 4, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_5, 5, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_6, 6, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_7, 7, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_8, 8, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_9, 9, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_10, 10, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_11, 11, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_12,
    12,
    25,
    vec![
        MapPokemonInfo::new(3, 16),
        MapPokemonInfo::new(3, 19),
        MapPokemonInfo::new(3, 19),
        MapPokemonInfo::new(2, 19),
        MapPokemonInfo::new(2, 16),
        MapPokemonInfo::new(3, 16),
        MapPokemonInfo::new(3, 16),
        MapPokemonInfo::new(4, 19),
        MapPokemonInfo::new(4, 16),
        MapPokemonInfo::new(5, 16),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_13,
    13,
    25,
    vec![
        MapPokemonInfo::new(3, 19),
        MapPokemonInfo::new(3, 16),
        MapPokemonInfo::new(4, 16),
        MapPokemonInfo::new(4, 19),
        MapPokemonInfo::new(5, 16),
        MapPokemonInfo::new(3, 13),
        MapPokemonInfo::new(2, 19),
        MapPokemonInfo::new(5, 19),
        MapPokemonInfo::new(4, 13),
        MapPokemonInfo::new(5, 13),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_14,
    14,
    20,
    vec![
        MapPokemonInfo::new(6, 16),
        MapPokemonInfo::new(5, 21),
        MapPokemonInfo::new(7, 16),
        MapPokemonInfo::new(6, 21),
        MapPokemonInfo::new(7, 21),
        MapPokemonInfo::new(8, 16),
        MapPokemonInfo::new(8, 21),
        MapPokemonInfo::new(3, 39),
        MapPokemonInfo::new(5, 39),
        MapPokemonInfo::new(7, 39),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_15,
    15,
    20,
    vec![
        MapPokemonInfo::new(10, 19),
        MapPokemonInfo::new(10, 21),
        MapPokemonInfo::new(8, 19),
        MapPokemonInfo::new(6, 23),
        MapPokemonInfo::new(8, 21),
        MapPokemonInfo::new(10, 23),
        MapPokemonInfo::new(12, 19),
        MapPokemonInfo::new(12, 21),
        MapPokemonInfo::new(8, 23),
        MapPokemonInfo::new(12, 23),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_16,
    16,
    15,
    vec![
        MapPokemonInfo::new(13, 43),
        MapPokemonInfo::new(13, 16),
        MapPokemonInfo::new(15, 16),
        MapPokemonInfo::new(10, 56),
        MapPokemonInfo::new(12, 56),
        MapPokemonInfo::new(15, 43),
        MapPokemonInfo::new(16, 43),
        MapPokemonInfo::new(16, 16),
        MapPokemonInfo::new(14, 56),
        MapPokemonInfo::new(16, 56),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_17,
    17,
    15,
    vec![
        MapPokemonInfo::new(13, 43),
        MapPokemonInfo::new(13, 16),
        MapPokemonInfo::new(15, 16),
        MapPokemonInfo::new(10, 56),
        MapPokemonInfo::new(12, 56),
        MapPokemonInfo::new(15, 43),
        MapPokemonInfo::new(16, 43),
        MapPokemonInfo::new(16, 16),
        MapPokemonInfo::new(14, 56),
        MapPokemonInfo::new(16, 56),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_18,
    18,
    15,
    vec![
        MapPokemonInfo::new(19, 16),
        MapPokemonInfo::new(19, 43),
        MapPokemonInfo::new(17, 56),
        MapPokemonInfo::new(22, 43),
        MapPokemonInfo::new(22, 16),
        MapPokemonInfo::new(18, 56),
        MapPokemonInfo::new(18, 58),
        MapPokemonInfo::new(20, 58),
        MapPokemonInfo::new(19, 56),
        MapPokemonInfo::new(20, 56),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_19,
    19,
    15,
    vec![
        MapPokemonInfo::new(18, 16),
        MapPokemonInfo::new(18, 56),
        MapPokemonInfo::new(17, 23),
        MapPokemonInfo::new(16, 58),
        MapPokemonInfo::new(20, 16),
        MapPokemonInfo::new(20, 56),
        MapPokemonInfo::new(19, 23),
        MapPokemonInfo::new(17, 58),
        MapPokemonInfo::new(15, 58),
        MapPokemonInfo::new(18, 58),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_20,
    20,
    15,
    vec![
        MapPokemonInfo::new(16, 19),
        MapPokemonInfo::new(16, 21),
        MapPokemonInfo::new(14, 19),
        MapPokemonInfo::new(11, 23),
        MapPokemonInfo::new(13, 21),
        MapPokemonInfo::new(15, 23),
        MapPokemonInfo::new(17, 19),
        MapPokemonInfo::new(17, 21),
        MapPokemonInfo::new(13, 23),
        MapPokemonInfo::new(17, 23),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_21,
    21,
    15,
    vec![
        MapPokemonInfo::new(16, 100),
        MapPokemonInfo::new(16, 21),
        MapPokemonInfo::new(14, 100),
        MapPokemonInfo::new(11, 23),
        MapPokemonInfo::new(13, 21),
        MapPokemonInfo::new(15, 23),
        MapPokemonInfo::new(17, 100),
        MapPokemonInfo::new(17, 21),
        MapPokemonInfo::new(13, 23),
        MapPokemonInfo::new(17, 23),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_22,
    22,
    15,
    vec![
        MapPokemonInfo::new(14, 23),
        MapPokemonInfo::new(15, 21),
        MapPokemonInfo::new(12, 23),
        MapPokemonInfo::new(9, 96),
        MapPokemonInfo::new(13, 21),
        MapPokemonInfo::new(13, 96),
        MapPokemonInfo::new(15, 23),
        MapPokemonInfo::new(17, 21),
        MapPokemonInfo::new(11, 96),
        MapPokemonInfo::new(15, 96),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_23,
    23,
    15,
    vec![
        MapPokemonInfo::new(24, 43),
        MapPokemonInfo::new(25, 16),
        MapPokemonInfo::new(23, 16),
        MapPokemonInfo::new(24, 48),
        MapPokemonInfo::new(22, 43),
        MapPokemonInfo::new(26, 48),
        MapPokemonInfo::new(26, 43),
        MapPokemonInfo::new(27, 16),
        MapPokemonInfo::new(28, 44),
        MapPokemonInfo::new(30, 44),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_24,
    24,
    20,
    vec![
        MapPokemonInfo::new(24, 43),
        MapPokemonInfo::new(25, 16),
        MapPokemonInfo::new(27, 16),
        MapPokemonInfo::new(24, 48),
        MapPokemonInfo::new(22, 43),
        MapPokemonInfo::new(26, 48),
        MapPokemonInfo::new(26, 43),
        MapPokemonInfo::new(25, 132),
        MapPokemonInfo::new(28, 44),
        MapPokemonInfo::new(30, 44),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_25,
    25,
    15,
    vec![
        MapPokemonInfo::new(24, 43),
        MapPokemonInfo::new(26, 16),
        MapPokemonInfo::new(23, 132),
        MapPokemonInfo::new(24, 48),
        MapPokemonInfo::new(22, 43),
        MapPokemonInfo::new(26, 48),
        MapPokemonInfo::new(26, 43),
        MapPokemonInfo::new(30, 44),
        MapPokemonInfo::new(28, 17),
        MapPokemonInfo::new(30, 17),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_26,
    26,
    15,
    vec![
        MapPokemonInfo::new(24, 43),
        MapPokemonInfo::new(26, 132),
        MapPokemonInfo::new(23, 16),
        MapPokemonInfo::new(26, 48),
        MapPokemonInfo::new(22, 43),
        MapPokemonInfo::new(28, 48),
        MapPokemonInfo::new(26, 43),
        MapPokemonInfo::new(30, 44),
        MapPokemonInfo::new(28, 17),
        MapPokemonInfo::new(30, 17),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_27,
    27,
    25,
    vec![
        MapPokemonInfo::new(20, 21),
        MapPokemonInfo::new(22, 21),
        MapPokemonInfo::new(18, 19),
        MapPokemonInfo::new(20, 84),
        MapPokemonInfo::new(20, 19),
        MapPokemonInfo::new(18, 84),
        MapPokemonInfo::new(22, 84),
        MapPokemonInfo::new(22, 19),
        MapPokemonInfo::new(23, 20),
        MapPokemonInfo::new(25, 20),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_28,
    28,
    25,
    vec![
        MapPokemonInfo::new(20, 21),
        MapPokemonInfo::new(22, 21),
        MapPokemonInfo::new(25, 20),
        MapPokemonInfo::new(24, 84),
        MapPokemonInfo::new(27, 20),
        MapPokemonInfo::new(26, 84),
        MapPokemonInfo::new(28, 84),
        MapPokemonInfo::new(29, 20),
        MapPokemonInfo::new(25, 22),
        MapPokemonInfo::new(27, 22),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_29,
    29,
    25,
    vec![
        MapPokemonInfo::new(20, 21),
        MapPokemonInfo::new(22, 21),
        MapPokemonInfo::new(25, 20),
        MapPokemonInfo::new(24, 84),
        MapPokemonInfo::new(25, 22),
        MapPokemonInfo::new(26, 84),
        MapPokemonInfo::new(28, 84),
        MapPokemonInfo::new(29, 20),
        MapPokemonInfo::new(27, 22),
        MapPokemonInfo::new(29, 22),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_30,
    30,
    0,
    vec![],
    5,
    vec![
        MapPokemonInfo::new(5, 72),
        MapPokemonInfo::new(10, 72),
        MapPokemonInfo::new(15, 72),
        MapPokemonInfo::new(5, 72),
        MapPokemonInfo::new(10, 72),
        MapPokemonInfo::new(15, 72),
        MapPokemonInfo::new(20, 72),
        MapPokemonInfo::new(30, 72),
        MapPokemonInfo::new(35, 72),
        MapPokemonInfo::new(40, 72),
    ]
);
get_map_pokemon_test!(
    get_map_pokemon_31,
    31,
    0,
    vec![],
    5,
    vec![
        MapPokemonInfo::new(5, 72),
        MapPokemonInfo::new(10, 72),
        MapPokemonInfo::new(15, 72),
        MapPokemonInfo::new(5, 72),
        MapPokemonInfo::new(10, 72),
        MapPokemonInfo::new(15, 72),
        MapPokemonInfo::new(20, 72),
        MapPokemonInfo::new(30, 72),
        MapPokemonInfo::new(35, 72),
        MapPokemonInfo::new(40, 72),
    ]
);
get_map_pokemon_test!(
    get_map_pokemon_32,
    32,
    25,
    vec![
        MapPokemonInfo::new(21, 19),
        MapPokemonInfo::new(23, 16),
        MapPokemonInfo::new(30, 20),
        MapPokemonInfo::new(23, 19),
        MapPokemonInfo::new(21, 16),
        MapPokemonInfo::new(30, 17),
        MapPokemonInfo::new(32, 17),
        MapPokemonInfo::new(28, 114),
        MapPokemonInfo::new(30, 114),
        MapPokemonInfo::new(32, 114),
    ],
    5,
    vec![
        MapPokemonInfo::new(5, 72),
        MapPokemonInfo::new(10, 72),
        MapPokemonInfo::new(15, 72),
        MapPokemonInfo::new(5, 72),
        MapPokemonInfo::new(10, 72),
        MapPokemonInfo::new(15, 72),
        MapPokemonInfo::new(20, 72),
        MapPokemonInfo::new(30, 72),
        MapPokemonInfo::new(35, 72),
        MapPokemonInfo::new(40, 72),
    ]
);
get_map_pokemon_test!(
    get_map_pokemon_33,
    33,
    25,
    vec![
        MapPokemonInfo::new(3, 19),
        MapPokemonInfo::new(3, 32),
        MapPokemonInfo::new(4, 19),
        MapPokemonInfo::new(4, 32),
        MapPokemonInfo::new(2, 19),
        MapPokemonInfo::new(2, 32),
        MapPokemonInfo::new(3, 21),
        MapPokemonInfo::new(5, 21),
        MapPokemonInfo::new(3, 29),
        MapPokemonInfo::new(4, 29),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_34,
    34,
    10,
    vec![
        MapPokemonInfo::new(26, 23),
        MapPokemonInfo::new(33, 132),
        MapPokemonInfo::new(26, 21),
        MapPokemonInfo::new(38, 22),
        MapPokemonInfo::new(38, 132),
        MapPokemonInfo::new(38, 22),
        MapPokemonInfo::new(41, 24),
        MapPokemonInfo::new(43, 132),
        MapPokemonInfo::new(41, 22),
        MapPokemonInfo::new(43, 22),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_35,
    35,
    25,
    vec![
        MapPokemonInfo::new(7, 13),
        MapPokemonInfo::new(8, 14),
        MapPokemonInfo::new(12, 16),
        MapPokemonInfo::new(12, 43),
        MapPokemonInfo::new(13, 43),
        MapPokemonInfo::new(10, 63),
        MapPokemonInfo::new(14, 43),
        MapPokemonInfo::new(13, 16),
        MapPokemonInfo::new(8, 63),
        MapPokemonInfo::new(12, 63),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_36,
    36,
    15,
    vec![
        MapPokemonInfo::new(8, 13),
        MapPokemonInfo::new(9, 14),
        MapPokemonInfo::new(13, 16),
        MapPokemonInfo::new(12, 43),
        MapPokemonInfo::new(13, 43),
        MapPokemonInfo::new(12, 63),
        MapPokemonInfo::new(14, 43),
        MapPokemonInfo::new(10, 63),
        MapPokemonInfo::new(7, 11),
        MapPokemonInfo::new(8, 10),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_37, 37, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_38, 38, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_39, 39, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_40, 40, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_41, 41, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_42, 42, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_43, 43, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_44, 44, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_45, 45, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_46, 46, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_47, 47, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_48, 48, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_49, 49, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_50, 50, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_51,
    51,
    8,
    vec![
        MapPokemonInfo::new(4, 13),
        MapPokemonInfo::new(5, 14),
        MapPokemonInfo::new(3, 13),
        MapPokemonInfo::new(5, 13),
        MapPokemonInfo::new(4, 14),
        MapPokemonInfo::new(6, 14),
        MapPokemonInfo::new(4, 11),
        MapPokemonInfo::new(3, 10),
        MapPokemonInfo::new(3, 25),
        MapPokemonInfo::new(5, 25),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_52, 52, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_53, 53, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_54, 54, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_55, 55, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_56, 56, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_57, 57, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_58, 58, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_59,
    59,
    10,
    vec![
        MapPokemonInfo::new(8, 41),
        MapPokemonInfo::new(7, 41),
        MapPokemonInfo::new(9, 41),
        MapPokemonInfo::new(8, 74),
        MapPokemonInfo::new(6, 41),
        MapPokemonInfo::new(10, 41),
        MapPokemonInfo::new(10, 74),
        MapPokemonInfo::new(8, 46),
        MapPokemonInfo::new(11, 41),
        MapPokemonInfo::new(8, 35),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_60,
    60,
    10,
    vec![
        MapPokemonInfo::new(8, 41),
        MapPokemonInfo::new(7, 41),
        MapPokemonInfo::new(7, 74),
        MapPokemonInfo::new(8, 74),
        MapPokemonInfo::new(9, 41),
        MapPokemonInfo::new(10, 46),
        MapPokemonInfo::new(10, 41),
        MapPokemonInfo::new(11, 41),
        MapPokemonInfo::new(9, 35),
        MapPokemonInfo::new(9, 74),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_61,
    61,
    10,
    vec![
        MapPokemonInfo::new(9, 41),
        MapPokemonInfo::new(9, 74),
        MapPokemonInfo::new(10, 41),
        MapPokemonInfo::new(10, 74),
        MapPokemonInfo::new(11, 41),
        MapPokemonInfo::new(10, 46),
        MapPokemonInfo::new(12, 46),
        MapPokemonInfo::new(10, 35),
        MapPokemonInfo::new(12, 41),
        MapPokemonInfo::new(12, 35),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_62, 62, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_63, 63, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_64, 64, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_65, 65, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_66, 66, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_67, 67, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_68, 68, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_69, 69, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_70, 70, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_71, 71, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_72, 72, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_73, 73, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_74, 74, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_75, 75, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_76, 76, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_77, 77, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_78, 78, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_79, 79, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_80, 80, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_81, 81, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_82,
    82,
    15,
    vec![
        MapPokemonInfo::new(16, 41),
        MapPokemonInfo::new(17, 41),
        MapPokemonInfo::new(17, 74),
        MapPokemonInfo::new(15, 66),
        MapPokemonInfo::new(16, 74),
        MapPokemonInfo::new(18, 41),
        MapPokemonInfo::new(15, 41),
        MapPokemonInfo::new(17, 66),
        MapPokemonInfo::new(13, 95),
        MapPokemonInfo::new(15, 95),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_83,
    83,
    10,
    vec![
        MapPokemonInfo::new(21, 100),
        MapPokemonInfo::new(21, 81),
        MapPokemonInfo::new(20, 25),
        MapPokemonInfo::new(24, 25),
        MapPokemonInfo::new(23, 81),
        MapPokemonInfo::new(23, 100),
        MapPokemonInfo::new(32, 82),
        MapPokemonInfo::new(35, 82),
        MapPokemonInfo::new(33, 125),
        MapPokemonInfo::new(36, 125),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_84, 84, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_85, 85, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_86, 86, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_87, 87, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_88, 88, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_89, 89, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_90, 90, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_91, 91, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_92, 92, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_93, 93, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_94, 94, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_95, 95, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_96, 96, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_97, 97, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_98, 98, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_99, 99, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_100, 100, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_101, 101, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_102, 102, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_103, 103, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_104, 104, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_105, 105, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_106, 106, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_107, 107, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_108,
    108,
    15,
    vec![
        MapPokemonInfo::new(24, 66),
        MapPokemonInfo::new(26, 74),
        MapPokemonInfo::new(22, 41),
        MapPokemonInfo::new(36, 95),
        MapPokemonInfo::new(39, 95),
        MapPokemonInfo::new(42, 95),
        MapPokemonInfo::new(41, 75),
        MapPokemonInfo::new(41, 42),
        MapPokemonInfo::new(42, 67),
        MapPokemonInfo::new(43, 105),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_109, 109, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_110, 110, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_111, 111, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_112, 112, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_113, 113, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_114, 114, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_115, 115, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_116, 116, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_117, 117, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_118, 118, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_119, 119, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_120, 120, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_121, 121, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_122, 122, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_123, 123, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_124, 124, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_125, 125, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_126, 126, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_127, 127, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_128, 128, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_129, 129, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_130, 130, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_131, 131, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_132, 132, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_133, 133, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_134, 134, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_135, 135, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_136, 136, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_137, 137, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_138, 138, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_139, 139, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_140, 140, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_141, 141, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_142, 142, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_143, 143, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_144,
    144,
    10,
    vec![
        MapPokemonInfo::new(20, 92),
        MapPokemonInfo::new(21, 92),
        MapPokemonInfo::new(22, 92),
        MapPokemonInfo::new(23, 92),
        MapPokemonInfo::new(19, 92),
        MapPokemonInfo::new(18, 92),
        MapPokemonInfo::new(24, 92),
        MapPokemonInfo::new(20, 104),
        MapPokemonInfo::new(22, 104),
        MapPokemonInfo::new(25, 93),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_145,
    145,
    10,
    vec![
        MapPokemonInfo::new(20, 92),
        MapPokemonInfo::new(21, 92),
        MapPokemonInfo::new(22, 92),
        MapPokemonInfo::new(23, 92),
        MapPokemonInfo::new(19, 92),
        MapPokemonInfo::new(18, 92),
        MapPokemonInfo::new(25, 93),
        MapPokemonInfo::new(20, 104),
        MapPokemonInfo::new(22, 104),
        MapPokemonInfo::new(24, 92),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_146,
    146,
    10,
    vec![
        MapPokemonInfo::new(20, 92),
        MapPokemonInfo::new(21, 92),
        MapPokemonInfo::new(22, 92),
        MapPokemonInfo::new(23, 92),
        MapPokemonInfo::new(19, 92),
        MapPokemonInfo::new(18, 92),
        MapPokemonInfo::new(25, 93),
        MapPokemonInfo::new(20, 104),
        MapPokemonInfo::new(22, 104),
        MapPokemonInfo::new(24, 92),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_147,
    147,
    15,
    vec![
        MapPokemonInfo::new(21, 92),
        MapPokemonInfo::new(22, 92),
        MapPokemonInfo::new(23, 92),
        MapPokemonInfo::new(24, 92),
        MapPokemonInfo::new(20, 92),
        MapPokemonInfo::new(19, 92),
        MapPokemonInfo::new(26, 93),
        MapPokemonInfo::new(22, 104),
        MapPokemonInfo::new(24, 104),
        MapPokemonInfo::new(28, 93),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_148,
    148,
    15,
    vec![
        MapPokemonInfo::new(21, 92),
        MapPokemonInfo::new(22, 92),
        MapPokemonInfo::new(23, 92),
        MapPokemonInfo::new(24, 92),
        MapPokemonInfo::new(20, 92),
        MapPokemonInfo::new(28, 93),
        MapPokemonInfo::new(22, 104),
        MapPokemonInfo::new(24, 104),
        MapPokemonInfo::new(28, 93),
        MapPokemonInfo::new(30, 93),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_149, 149, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_150, 150, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_151, 151, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_152, 152, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_153, 153, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_154, 154, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_155, 155, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_156, 156, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_157, 157, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_158, 158, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_159,
    159,
    10,
    vec![
        MapPokemonInfo::new(30, 120),
        MapPokemonInfo::new(30, 116),
        MapPokemonInfo::new(32, 90),
        MapPokemonInfo::new(32, 116),
        MapPokemonInfo::new(28, 79),
        MapPokemonInfo::new(30, 86),
        MapPokemonInfo::new(30, 79),
        MapPokemonInfo::new(28, 86),
        MapPokemonInfo::new(38, 87),
        MapPokemonInfo::new(37, 117),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_160,
    160,
    10,
    vec![
        MapPokemonInfo::new(30, 86),
        MapPokemonInfo::new(30, 79),
        MapPokemonInfo::new(32, 86),
        MapPokemonInfo::new(32, 79),
        MapPokemonInfo::new(28, 116),
        MapPokemonInfo::new(30, 120),
        MapPokemonInfo::new(30, 116),
        MapPokemonInfo::new(28, 90),
        MapPokemonInfo::new(30, 42),
        MapPokemonInfo::new(37, 80),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_161,
    161,
    10,
    vec![
        MapPokemonInfo::new(31, 79),
        MapPokemonInfo::new(31, 86),
        MapPokemonInfo::new(33, 79),
        MapPokemonInfo::new(33, 86),
        MapPokemonInfo::new(29, 116),
        MapPokemonInfo::new(31, 90),
        MapPokemonInfo::new(31, 116),
        MapPokemonInfo::new(29, 90),
        MapPokemonInfo::new(39, 117),
        MapPokemonInfo::new(37, 87),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_162,
    162,
    10,
    vec![
        MapPokemonInfo::new(31, 116),
        MapPokemonInfo::new(31, 90),
        MapPokemonInfo::new(33, 116),
        MapPokemonInfo::new(33, 90),
        MapPokemonInfo::new(29, 79),
        MapPokemonInfo::new(31, 86),
        MapPokemonInfo::new(31, 79),
        MapPokemonInfo::new(29, 86),
        MapPokemonInfo::new(39, 80),
        MapPokemonInfo::new(32, 42),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_163, 163, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_164, 164, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_165,
    165,
    10,
    vec![
        MapPokemonInfo::new(32, 109),
        MapPokemonInfo::new(30, 109),
        MapPokemonInfo::new(34, 77),
        MapPokemonInfo::new(30, 77),
        MapPokemonInfo::new(34, 58),
        MapPokemonInfo::new(32, 77),
        MapPokemonInfo::new(30, 88),
        MapPokemonInfo::new(28, 77),
        MapPokemonInfo::new(37, 110),
        MapPokemonInfo::new(39, 89),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_166, 166, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_167, 167, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_168, 168, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_169, 169, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_170, 170, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_171, 171, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_172, 172, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_173, 173, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_174, 174, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_175, 175, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_176, 176, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_177, 177, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_178, 178, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_179, 179, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_180, 180, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_181, 181, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_182, 182, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_183, 183, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_184, 184, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_185, 185, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_186, 186, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_187, 187, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_188, 188, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_189, 189, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_190, 190, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_191, 191, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_192,
    192,
    15,
    vec![
        MapPokemonInfo::new(30, 86),
        MapPokemonInfo::new(30, 79),
        MapPokemonInfo::new(30, 90),
        MapPokemonInfo::new(30, 116),
        MapPokemonInfo::new(28, 116),
        MapPokemonInfo::new(21, 41),
        MapPokemonInfo::new(29, 42),
        MapPokemonInfo::new(28, 54),
        MapPokemonInfo::new(28, 90),
        MapPokemonInfo::new(38, 55),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_193, 193, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_194,
    194,
    10,
    vec![
        MapPokemonInfo::new(22, 66),
        MapPokemonInfo::new(24, 74),
        MapPokemonInfo::new(26, 41),
        MapPokemonInfo::new(36, 95),
        MapPokemonInfo::new(39, 95),
        MapPokemonInfo::new(42, 95),
        MapPokemonInfo::new(41, 67),
        MapPokemonInfo::new(40, 42),
        MapPokemonInfo::new(40, 105),
        MapPokemonInfo::new(43, 75),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_195, 195, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_196, 196, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_197,
    197,
    20,
    vec![
        MapPokemonInfo::new(18, 50),
        MapPokemonInfo::new(19, 50),
        MapPokemonInfo::new(17, 50),
        MapPokemonInfo::new(20, 50),
        MapPokemonInfo::new(16, 50),
        MapPokemonInfo::new(15, 50),
        MapPokemonInfo::new(21, 50),
        MapPokemonInfo::new(22, 50),
        MapPokemonInfo::new(29, 51),
        MapPokemonInfo::new(31, 51),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_198,
    198,
    15,
    vec![
        MapPokemonInfo::new(24, 66),
        MapPokemonInfo::new(26, 74),
        MapPokemonInfo::new(22, 41),
        MapPokemonInfo::new(42, 95),
        MapPokemonInfo::new(40, 49),
        MapPokemonInfo::new(45, 95),
        MapPokemonInfo::new(43, 75),
        MapPokemonInfo::new(41, 42),
        MapPokemonInfo::new(42, 67),
        MapPokemonInfo::new(45, 67),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_199, 199, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_200, 200, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_201, 201, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_202, 202, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_203, 203, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_204, 204, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_205, 205, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_206, 206, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_207, 207, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_208, 208, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_209, 209, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_210, 210, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_211, 211, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_212, 212, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_213, 213, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_214,
    214,
    10,
    vec![
        MapPokemonInfo::new(32, 58),
        MapPokemonInfo::new(34, 109),
        MapPokemonInfo::new(34, 109),
        MapPokemonInfo::new(30, 77),
        MapPokemonInfo::new(30, 109),
        MapPokemonInfo::new(32, 77),
        MapPokemonInfo::new(30, 88),
        MapPokemonInfo::new(28, 77),
        MapPokemonInfo::new(39, 110),
        MapPokemonInfo::new(37, 89),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_215,
    215,
    10,
    vec![
        MapPokemonInfo::new(31, 109),
        MapPokemonInfo::new(33, 58),
        MapPokemonInfo::new(35, 109),
        MapPokemonInfo::new(32, 77),
        MapPokemonInfo::new(34, 77),
        MapPokemonInfo::new(40, 110),
        MapPokemonInfo::new(34, 88),
        MapPokemonInfo::new(38, 110),
        MapPokemonInfo::new(36, 77),
        MapPokemonInfo::new(42, 89),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_216,
    216,
    10,
    vec![
        MapPokemonInfo::new(33, 109),
        MapPokemonInfo::new(31, 109),
        MapPokemonInfo::new(35, 58),
        MapPokemonInfo::new(32, 77),
        MapPokemonInfo::new(31, 109),
        MapPokemonInfo::new(40, 110),
        MapPokemonInfo::new(34, 77),
        MapPokemonInfo::new(35, 88),
        MapPokemonInfo::new(42, 110),
        MapPokemonInfo::new(42, 89),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_217,
    217,
    30,
    vec![
        MapPokemonInfo::new(24, 32),
        MapPokemonInfo::new(26, 84),
        MapPokemonInfo::new(22, 46),
        MapPokemonInfo::new(25, 102),
        MapPokemonInfo::new(33, 33),
        MapPokemonInfo::new(23, 102),
        MapPokemonInfo::new(24, 29),
        MapPokemonInfo::new(25, 47),
        MapPokemonInfo::new(25, 115),
        MapPokemonInfo::new(28, 123),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_218,
    218,
    30,
    vec![
        MapPokemonInfo::new(22, 32),
        MapPokemonInfo::new(26, 111),
        MapPokemonInfo::new(23, 46),
        MapPokemonInfo::new(25, 102),
        MapPokemonInfo::new(30, 33),
        MapPokemonInfo::new(27, 102),
        MapPokemonInfo::new(30, 30),
        MapPokemonInfo::new(32, 49),
        MapPokemonInfo::new(26, 113),
        MapPokemonInfo::new(28, 128),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_219,
    219,
    30,
    vec![
        MapPokemonInfo::new(25, 32),
        MapPokemonInfo::new(26, 84),
        MapPokemonInfo::new(23, 48),
        MapPokemonInfo::new(24, 102),
        MapPokemonInfo::new(33, 33),
        MapPokemonInfo::new(26, 102),
        MapPokemonInfo::new(25, 29),
        MapPokemonInfo::new(31, 49),
        MapPokemonInfo::new(26, 128),
        MapPokemonInfo::new(28, 115),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_220,
    220,
    30,
    vec![
        MapPokemonInfo::new(22, 32),
        MapPokemonInfo::new(25, 111),
        MapPokemonInfo::new(22, 48),
        MapPokemonInfo::new(24, 102),
        MapPokemonInfo::new(31, 33),
        MapPokemonInfo::new(25, 102),
        MapPokemonInfo::new(31, 30),
        MapPokemonInfo::new(30, 47),
        MapPokemonInfo::new(23, 123),
        MapPokemonInfo::new(23, 113),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_221, 221, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_222, 222, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_223, 223, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_224, 224, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_225, 225, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_226,
    226,
    15,
    vec![
        MapPokemonInfo::new(51, 85),
        MapPokemonInfo::new(51, 49),
        MapPokemonInfo::new(51, 64),
        MapPokemonInfo::new(52, 112),
        MapPokemonInfo::new(52, 105),
        MapPokemonInfo::new(52, 101),
        MapPokemonInfo::new(56, 113),
        MapPokemonInfo::new(54, 40),
        MapPokemonInfo::new(55, 132),
        MapPokemonInfo::new(60, 132),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_227,
    227,
    25,
    vec![
        MapPokemonInfo::new(55, 112),
        MapPokemonInfo::new(55, 105),
        MapPokemonInfo::new(55, 101),
        MapPokemonInfo::new(64, 113),
        MapPokemonInfo::new(64, 47),
        MapPokemonInfo::new(64, 26),
        MapPokemonInfo::new(57, 24),
        MapPokemonInfo::new(65, 132),
        MapPokemonInfo::new(63, 132),
        MapPokemonInfo::new(67, 132),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(
    get_map_pokemon_228,
    228,
    10,
    vec![
        MapPokemonInfo::new(46, 42),
        MapPokemonInfo::new(46, 97),
        MapPokemonInfo::new(46, 82),
        MapPokemonInfo::new(49, 85),
        MapPokemonInfo::new(49, 49),
        MapPokemonInfo::new(52, 24),
        MapPokemonInfo::new(49, 64),
        MapPokemonInfo::new(52, 47),
        MapPokemonInfo::new(53, 26),
        MapPokemonInfo::new(53, 132),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_229, 229, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_230, 230, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_231, 231, 0, vec![], 0, vec![]);
get_map_pokemon_test!(
    get_map_pokemon_232,
    232,
    15,
    vec![
        MapPokemonInfo::new(16, 41),
        MapPokemonInfo::new(17, 41),
        MapPokemonInfo::new(17, 74),
        MapPokemonInfo::new(15, 66),
        MapPokemonInfo::new(16, 74),
        MapPokemonInfo::new(18, 41),
        MapPokemonInfo::new(17, 66),
        MapPokemonInfo::new(17, 95),
        MapPokemonInfo::new(13, 95),
        MapPokemonInfo::new(18, 74),
    ],
    0,
    vec![]
);
get_map_pokemon_test!(get_map_pokemon_233, 233, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_234, 234, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_235, 235, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_236, 236, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_237, 237, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_238, 238, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_239, 239, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_240, 240, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_241, 241, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_242, 242, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_243, 243, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_244, 244, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_245, 245, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_246, 246, 0, vec![], 0, vec![]);
get_map_pokemon_test!(get_map_pokemon_247, 247, 0, vec![], 0, vec![]);
