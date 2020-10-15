use pkmnapi_db::*;

mod common;

macro_rules! get_pokemon_learnset_test {
    ($test_name:ident, $pokedex_id:expr, $pokemon_learnset:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_pokemon_learnset(&$pokedex_id) {
                Ok(pokemon_learnset) => assert_eq!(
                    pokemon_learnset, $pokemon_learnset,
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

get_pokemon_learnset_test!(
    get_pokemon_learnset_1,
    1,
    vec![
        PokemonLearnset::new(7, 73),
        PokemonLearnset::new(13, 22),
        PokemonLearnset::new(20, 77),
        PokemonLearnset::new(27, 75),
        PokemonLearnset::new(34, 74),
        PokemonLearnset::new(41, 79),
        PokemonLearnset::new(48, 76),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_2,
    2,
    vec![
        PokemonLearnset::new(7, 73),
        PokemonLearnset::new(13, 22),
        PokemonLearnset::new(22, 77),
        PokemonLearnset::new(30, 75),
        PokemonLearnset::new(38, 74),
        PokemonLearnset::new(46, 79),
        PokemonLearnset::new(54, 76),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_3,
    3,
    vec![
        PokemonLearnset::new(7, 73),
        PokemonLearnset::new(13, 22),
        PokemonLearnset::new(22, 77),
        PokemonLearnset::new(30, 75),
        PokemonLearnset::new(43, 74),
        PokemonLearnset::new(55, 79),
        PokemonLearnset::new(65, 76),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_4,
    4,
    vec![
        PokemonLearnset::new(9, 52),
        PokemonLearnset::new(15, 43),
        PokemonLearnset::new(22, 99),
        PokemonLearnset::new(30, 163),
        PokemonLearnset::new(38, 53),
        PokemonLearnset::new(46, 83),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_5,
    5,
    vec![
        PokemonLearnset::new(9, 52),
        PokemonLearnset::new(15, 43),
        PokemonLearnset::new(24, 99),
        PokemonLearnset::new(33, 163),
        PokemonLearnset::new(42, 53),
        PokemonLearnset::new(56, 83),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_6,
    6,
    vec![
        PokemonLearnset::new(9, 52),
        PokemonLearnset::new(15, 43),
        PokemonLearnset::new(24, 99),
        PokemonLearnset::new(36, 163),
        PokemonLearnset::new(46, 53),
        PokemonLearnset::new(55, 83),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_7,
    7,
    vec![
        PokemonLearnset::new(8, 145),
        PokemonLearnset::new(15, 55),
        PokemonLearnset::new(22, 44),
        PokemonLearnset::new(28, 110),
        PokemonLearnset::new(35, 130),
        PokemonLearnset::new(42, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_8,
    8,
    vec![
        PokemonLearnset::new(8, 145),
        PokemonLearnset::new(15, 55),
        PokemonLearnset::new(24, 44),
        PokemonLearnset::new(31, 110),
        PokemonLearnset::new(39, 130),
        PokemonLearnset::new(47, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_9,
    9,
    vec![
        PokemonLearnset::new(8, 145),
        PokemonLearnset::new(15, 55),
        PokemonLearnset::new(24, 44),
        PokemonLearnset::new(31, 110),
        PokemonLearnset::new(42, 130),
        PokemonLearnset::new(52, 56),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_10, 10, vec![]);
get_pokemon_learnset_test!(get_pokemon_learnset_11, 11, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_12,
    12,
    vec![
        PokemonLearnset::new(12, 93),
        PokemonLearnset::new(15, 77),
        PokemonLearnset::new(16, 78),
        PokemonLearnset::new(17, 79),
        PokemonLearnset::new(21, 48),
        PokemonLearnset::new(26, 18),
        PokemonLearnset::new(32, 60),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_13, 13, vec![]);
get_pokemon_learnset_test!(get_pokemon_learnset_14, 14, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_15,
    15,
    vec![
        PokemonLearnset::new(12, 31),
        PokemonLearnset::new(16, 116),
        PokemonLearnset::new(20, 41),
        PokemonLearnset::new(25, 99),
        PokemonLearnset::new(30, 42),
        PokemonLearnset::new(35, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_16,
    16,
    vec![
        PokemonLearnset::new(5, 28),
        PokemonLearnset::new(12, 98),
        PokemonLearnset::new(19, 18),
        PokemonLearnset::new(28, 17),
        PokemonLearnset::new(36, 97),
        PokemonLearnset::new(44, 119),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_17,
    17,
    vec![
        PokemonLearnset::new(5, 28),
        PokemonLearnset::new(12, 98),
        PokemonLearnset::new(21, 18),
        PokemonLearnset::new(31, 17),
        PokemonLearnset::new(40, 97),
        PokemonLearnset::new(49, 119),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_18,
    18,
    vec![
        PokemonLearnset::new(5, 28),
        PokemonLearnset::new(12, 98),
        PokemonLearnset::new(21, 18),
        PokemonLearnset::new(31, 17),
        PokemonLearnset::new(44, 97),
        PokemonLearnset::new(54, 119),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_19,
    19,
    vec![
        PokemonLearnset::new(7, 98),
        PokemonLearnset::new(14, 158),
        PokemonLearnset::new(23, 116),
        PokemonLearnset::new(34, 162),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_20,
    20,
    vec![
        PokemonLearnset::new(7, 98),
        PokemonLearnset::new(14, 158),
        PokemonLearnset::new(27, 116),
        PokemonLearnset::new(41, 162),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_21,
    21,
    vec![
        PokemonLearnset::new(9, 43),
        PokemonLearnset::new(15, 31),
        PokemonLearnset::new(22, 119),
        PokemonLearnset::new(29, 65),
        PokemonLearnset::new(36, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_22,
    22,
    vec![
        PokemonLearnset::new(9, 43),
        PokemonLearnset::new(15, 31),
        PokemonLearnset::new(25, 119),
        PokemonLearnset::new(34, 65),
        PokemonLearnset::new(43, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_23,
    23,
    vec![
        PokemonLearnset::new(10, 40),
        PokemonLearnset::new(17, 44),
        PokemonLearnset::new(24, 137),
        PokemonLearnset::new(31, 103),
        PokemonLearnset::new(38, 51),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_24,
    24,
    vec![
        PokemonLearnset::new(10, 40),
        PokemonLearnset::new(17, 44),
        PokemonLearnset::new(27, 137),
        PokemonLearnset::new(36, 103),
        PokemonLearnset::new(47, 51),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_25,
    25,
    vec![
        PokemonLearnset::new(9, 86),
        PokemonLearnset::new(16, 98),
        PokemonLearnset::new(26, 129),
        PokemonLearnset::new(33, 97),
        PokemonLearnset::new(43, 87),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_26, 26, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_27,
    27,
    vec![
        PokemonLearnset::new(10, 28),
        PokemonLearnset::new(17, 163),
        PokemonLearnset::new(24, 40),
        PokemonLearnset::new(31, 129),
        PokemonLearnset::new(38, 154),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_28,
    28,
    vec![
        PokemonLearnset::new(10, 28),
        PokemonLearnset::new(17, 163),
        PokemonLearnset::new(27, 40),
        PokemonLearnset::new(36, 129),
        PokemonLearnset::new(47, 154),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_29,
    29,
    vec![
        PokemonLearnset::new(8, 10),
        PokemonLearnset::new(14, 40),
        PokemonLearnset::new(21, 39),
        PokemonLearnset::new(29, 44),
        PokemonLearnset::new(36, 154),
        PokemonLearnset::new(43, 24),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_30,
    30,
    vec![
        PokemonLearnset::new(8, 10),
        PokemonLearnset::new(14, 40),
        PokemonLearnset::new(23, 39),
        PokemonLearnset::new(32, 44),
        PokemonLearnset::new(41, 154),
        PokemonLearnset::new(50, 24),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_31,
    31,
    vec![
        PokemonLearnset::new(8, 10),
        PokemonLearnset::new(14, 40),
        PokemonLearnset::new(23, 34),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_32,
    32,
    vec![
        PokemonLearnset::new(8, 30),
        PokemonLearnset::new(14, 40),
        PokemonLearnset::new(21, 116),
        PokemonLearnset::new(29, 31),
        PokemonLearnset::new(36, 32),
        PokemonLearnset::new(43, 24),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_33,
    33,
    vec![
        PokemonLearnset::new(8, 30),
        PokemonLearnset::new(14, 40),
        PokemonLearnset::new(23, 116),
        PokemonLearnset::new(32, 31),
        PokemonLearnset::new(41, 32),
        PokemonLearnset::new(50, 24),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_34,
    34,
    vec![
        PokemonLearnset::new(8, 30),
        PokemonLearnset::new(14, 40),
        PokemonLearnset::new(23, 37),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_35,
    35,
    vec![
        PokemonLearnset::new(13, 47),
        PokemonLearnset::new(18, 3),
        PokemonLearnset::new(24, 107),
        PokemonLearnset::new(31, 118),
        PokemonLearnset::new(39, 111),
        PokemonLearnset::new(48, 113),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_36, 36, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_37,
    37,
    vec![
        PokemonLearnset::new(16, 98),
        PokemonLearnset::new(21, 46),
        PokemonLearnset::new(28, 109),
        PokemonLearnset::new(35, 53),
        PokemonLearnset::new(42, 83),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_38, 38, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_39,
    39,
    vec![
        PokemonLearnset::new(9, 1),
        PokemonLearnset::new(14, 50),
        PokemonLearnset::new(19, 111),
        PokemonLearnset::new(24, 3),
        PokemonLearnset::new(29, 156),
        PokemonLearnset::new(34, 34),
        PokemonLearnset::new(39, 38),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_40, 40, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_41,
    41,
    vec![
        PokemonLearnset::new(10, 48),
        PokemonLearnset::new(15, 44),
        PokemonLearnset::new(21, 109),
        PokemonLearnset::new(28, 17),
        PokemonLearnset::new(36, 114),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_42,
    42,
    vec![
        PokemonLearnset::new(10, 48),
        PokemonLearnset::new(15, 44),
        PokemonLearnset::new(21, 109),
        PokemonLearnset::new(32, 17),
        PokemonLearnset::new(43, 114),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_43,
    43,
    vec![
        PokemonLearnset::new(15, 77),
        PokemonLearnset::new(17, 78),
        PokemonLearnset::new(19, 79),
        PokemonLearnset::new(24, 51),
        PokemonLearnset::new(33, 80),
        PokemonLearnset::new(46, 76),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_44,
    44,
    vec![
        PokemonLearnset::new(15, 77),
        PokemonLearnset::new(17, 78),
        PokemonLearnset::new(19, 79),
        PokemonLearnset::new(28, 51),
        PokemonLearnset::new(38, 80),
        PokemonLearnset::new(52, 76),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_45,
    45,
    vec![
        PokemonLearnset::new(15, 77),
        PokemonLearnset::new(17, 78),
        PokemonLearnset::new(19, 79),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_46,
    46,
    vec![
        PokemonLearnset::new(13, 78),
        PokemonLearnset::new(20, 141),
        PokemonLearnset::new(27, 147),
        PokemonLearnset::new(34, 163),
        PokemonLearnset::new(41, 74),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_47,
    47,
    vec![
        PokemonLearnset::new(13, 78),
        PokemonLearnset::new(20, 141),
        PokemonLearnset::new(30, 147),
        PokemonLearnset::new(39, 163),
        PokemonLearnset::new(48, 74),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_48,
    48,
    vec![
        PokemonLearnset::new(24, 77),
        PokemonLearnset::new(27, 141),
        PokemonLearnset::new(30, 78),
        PokemonLearnset::new(35, 60),
        PokemonLearnset::new(38, 79),
        PokemonLearnset::new(43, 94),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_49,
    49,
    vec![
        PokemonLearnset::new(24, 77),
        PokemonLearnset::new(27, 141),
        PokemonLearnset::new(30, 78),
        PokemonLearnset::new(38, 60),
        PokemonLearnset::new(43, 79),
        PokemonLearnset::new(50, 94),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_50,
    50,
    vec![
        PokemonLearnset::new(15, 45),
        PokemonLearnset::new(19, 91),
        PokemonLearnset::new(24, 28),
        PokemonLearnset::new(31, 163),
        PokemonLearnset::new(40, 89),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_51,
    51,
    vec![
        PokemonLearnset::new(15, 45),
        PokemonLearnset::new(19, 91),
        PokemonLearnset::new(24, 28),
        PokemonLearnset::new(35, 163),
        PokemonLearnset::new(47, 89),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_52,
    52,
    vec![
        PokemonLearnset::new(12, 44),
        PokemonLearnset::new(17, 6),
        PokemonLearnset::new(24, 103),
        PokemonLearnset::new(33, 154),
        PokemonLearnset::new(44, 163),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_53,
    53,
    vec![
        PokemonLearnset::new(12, 44),
        PokemonLearnset::new(17, 6),
        PokemonLearnset::new(24, 103),
        PokemonLearnset::new(37, 154),
        PokemonLearnset::new(51, 163),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_54,
    54,
    vec![
        PokemonLearnset::new(28, 39),
        PokemonLearnset::new(31, 50),
        PokemonLearnset::new(36, 93),
        PokemonLearnset::new(43, 154),
        PokemonLearnset::new(52, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_55,
    55,
    vec![
        PokemonLearnset::new(28, 39),
        PokemonLearnset::new(31, 50),
        PokemonLearnset::new(39, 93),
        PokemonLearnset::new(48, 154),
        PokemonLearnset::new(59, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_56,
    56,
    vec![
        PokemonLearnset::new(15, 2),
        PokemonLearnset::new(21, 154),
        PokemonLearnset::new(27, 116),
        PokemonLearnset::new(33, 69),
        PokemonLearnset::new(39, 37),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_57,
    57,
    vec![
        PokemonLearnset::new(15, 2),
        PokemonLearnset::new(21, 154),
        PokemonLearnset::new(27, 116),
        PokemonLearnset::new(37, 69),
        PokemonLearnset::new(46, 37),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_58,
    58,
    vec![
        PokemonLearnset::new(18, 52),
        PokemonLearnset::new(23, 43),
        PokemonLearnset::new(30, 36),
        PokemonLearnset::new(39, 97),
        PokemonLearnset::new(50, 53),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_59, 59, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_60,
    60,
    vec![
        PokemonLearnset::new(16, 95),
        PokemonLearnset::new(19, 55),
        PokemonLearnset::new(25, 3),
        PokemonLearnset::new(31, 34),
        PokemonLearnset::new(38, 133),
        PokemonLearnset::new(45, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_61,
    61,
    vec![
        PokemonLearnset::new(16, 95),
        PokemonLearnset::new(19, 55),
        PokemonLearnset::new(26, 3),
        PokemonLearnset::new(33, 34),
        PokemonLearnset::new(41, 133),
        PokemonLearnset::new(49, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_62,
    62,
    vec![PokemonLearnset::new(16, 95), PokemonLearnset::new(19, 55),]
);
get_pokemon_learnset_test!(get_pokemon_learnset_63, 63, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_64,
    64,
    vec![
        PokemonLearnset::new(16, 93),
        PokemonLearnset::new(20, 50),
        PokemonLearnset::new(27, 60),
        PokemonLearnset::new(31, 105),
        PokemonLearnset::new(38, 94),
        PokemonLearnset::new(42, 115),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_65,
    65,
    vec![
        PokemonLearnset::new(16, 93),
        PokemonLearnset::new(20, 50),
        PokemonLearnset::new(27, 60),
        PokemonLearnset::new(31, 105),
        PokemonLearnset::new(38, 94),
        PokemonLearnset::new(42, 115),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_66,
    66,
    vec![
        PokemonLearnset::new(20, 67),
        PokemonLearnset::new(25, 43),
        PokemonLearnset::new(32, 116),
        PokemonLearnset::new(39, 69),
        PokemonLearnset::new(46, 66),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_67,
    67,
    vec![
        PokemonLearnset::new(20, 67),
        PokemonLearnset::new(25, 43),
        PokemonLearnset::new(36, 116),
        PokemonLearnset::new(44, 69),
        PokemonLearnset::new(52, 66),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_68,
    68,
    vec![
        PokemonLearnset::new(20, 67),
        PokemonLearnset::new(25, 43),
        PokemonLearnset::new(36, 116),
        PokemonLearnset::new(44, 69),
        PokemonLearnset::new(52, 66),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_69,
    69,
    vec![
        PokemonLearnset::new(13, 35),
        PokemonLearnset::new(15, 77),
        PokemonLearnset::new(18, 79),
        PokemonLearnset::new(21, 78),
        PokemonLearnset::new(26, 51),
        PokemonLearnset::new(33, 75),
        PokemonLearnset::new(42, 21),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_70,
    70,
    vec![
        PokemonLearnset::new(13, 35),
        PokemonLearnset::new(15, 77),
        PokemonLearnset::new(18, 79),
        PokemonLearnset::new(23, 78),
        PokemonLearnset::new(29, 51),
        PokemonLearnset::new(38, 75),
        PokemonLearnset::new(49, 21),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_71,
    71,
    vec![
        PokemonLearnset::new(13, 35),
        PokemonLearnset::new(15, 77),
        PokemonLearnset::new(18, 79),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_72,
    72,
    vec![
        PokemonLearnset::new(7, 48),
        PokemonLearnset::new(13, 35),
        PokemonLearnset::new(18, 40),
        PokemonLearnset::new(22, 55),
        PokemonLearnset::new(27, 132),
        PokemonLearnset::new(33, 112),
        PokemonLearnset::new(40, 103),
        PokemonLearnset::new(48, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_73,
    73,
    vec![
        PokemonLearnset::new(7, 48),
        PokemonLearnset::new(13, 35),
        PokemonLearnset::new(18, 40),
        PokemonLearnset::new(22, 55),
        PokemonLearnset::new(27, 132),
        PokemonLearnset::new(35, 112),
        PokemonLearnset::new(43, 103),
        PokemonLearnset::new(50, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_74,
    74,
    vec![
        PokemonLearnset::new(11, 111),
        PokemonLearnset::new(16, 88),
        PokemonLearnset::new(21, 120),
        PokemonLearnset::new(26, 106),
        PokemonLearnset::new(31, 89),
        PokemonLearnset::new(36, 153),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_75,
    75,
    vec![
        PokemonLearnset::new(11, 111),
        PokemonLearnset::new(16, 88),
        PokemonLearnset::new(21, 120),
        PokemonLearnset::new(29, 106),
        PokemonLearnset::new(36, 89),
        PokemonLearnset::new(43, 153),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_76,
    76,
    vec![
        PokemonLearnset::new(11, 111),
        PokemonLearnset::new(16, 88),
        PokemonLearnset::new(21, 120),
        PokemonLearnset::new(29, 106),
        PokemonLearnset::new(36, 89),
        PokemonLearnset::new(43, 153),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_77,
    77,
    vec![
        PokemonLearnset::new(30, 39),
        PokemonLearnset::new(32, 23),
        PokemonLearnset::new(35, 45),
        PokemonLearnset::new(39, 83),
        PokemonLearnset::new(43, 36),
        PokemonLearnset::new(48, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_78,
    78,
    vec![
        PokemonLearnset::new(30, 39),
        PokemonLearnset::new(32, 23),
        PokemonLearnset::new(35, 45),
        PokemonLearnset::new(39, 83),
        PokemonLearnset::new(47, 36),
        PokemonLearnset::new(55, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_79,
    79,
    vec![
        PokemonLearnset::new(18, 50),
        PokemonLearnset::new(22, 29),
        PokemonLearnset::new(27, 45),
        PokemonLearnset::new(33, 55),
        PokemonLearnset::new(40, 133),
        PokemonLearnset::new(48, 94),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_80,
    80,
    vec![
        PokemonLearnset::new(18, 50),
        PokemonLearnset::new(22, 29),
        PokemonLearnset::new(27, 45),
        PokemonLearnset::new(33, 55),
        PokemonLearnset::new(37, 110),
        PokemonLearnset::new(44, 133),
        PokemonLearnset::new(55, 94),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_81,
    81,
    vec![
        PokemonLearnset::new(21, 49),
        PokemonLearnset::new(25, 84),
        PokemonLearnset::new(29, 48),
        PokemonLearnset::new(35, 86),
        PokemonLearnset::new(41, 129),
        PokemonLearnset::new(47, 103),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_82,
    82,
    vec![
        PokemonLearnset::new(21, 49),
        PokemonLearnset::new(25, 84),
        PokemonLearnset::new(29, 48),
        PokemonLearnset::new(38, 86),
        PokemonLearnset::new(46, 129),
        PokemonLearnset::new(54, 103),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_83,
    83,
    vec![
        PokemonLearnset::new(7, 43),
        PokemonLearnset::new(15, 31),
        PokemonLearnset::new(23, 14),
        PokemonLearnset::new(31, 97),
        PokemonLearnset::new(39, 163),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_84,
    84,
    vec![
        PokemonLearnset::new(20, 45),
        PokemonLearnset::new(24, 31),
        PokemonLearnset::new(30, 65),
        PokemonLearnset::new(36, 99),
        PokemonLearnset::new(40, 161),
        PokemonLearnset::new(44, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_85,
    85,
    vec![
        PokemonLearnset::new(20, 45),
        PokemonLearnset::new(24, 31),
        PokemonLearnset::new(30, 65),
        PokemonLearnset::new(39, 99),
        PokemonLearnset::new(45, 161),
        PokemonLearnset::new(51, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_86,
    86,
    vec![
        PokemonLearnset::new(30, 45),
        PokemonLearnset::new(35, 62),
        PokemonLearnset::new(40, 156),
        PokemonLearnset::new(45, 36),
        PokemonLearnset::new(50, 58),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_87,
    87,
    vec![
        PokemonLearnset::new(30, 45),
        PokemonLearnset::new(35, 62),
        PokemonLearnset::new(44, 156),
        PokemonLearnset::new(50, 36),
        PokemonLearnset::new(56, 58),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_88,
    88,
    vec![
        PokemonLearnset::new(30, 139),
        PokemonLearnset::new(33, 107),
        PokemonLearnset::new(37, 124),
        PokemonLearnset::new(42, 106),
        PokemonLearnset::new(48, 103),
        PokemonLearnset::new(55, 151),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_89,
    89,
    vec![
        PokemonLearnset::new(30, 139),
        PokemonLearnset::new(33, 107),
        PokemonLearnset::new(37, 124),
        PokemonLearnset::new(45, 106),
        PokemonLearnset::new(53, 103),
        PokemonLearnset::new(60, 151),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_90,
    90,
    vec![
        PokemonLearnset::new(18, 48),
        PokemonLearnset::new(23, 128),
        PokemonLearnset::new(30, 62),
        PokemonLearnset::new(39, 43),
        PokemonLearnset::new(50, 58),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_91,
    91,
    vec![PokemonLearnset::new(50, 131),]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_92,
    92,
    vec![PokemonLearnset::new(27, 95), PokemonLearnset::new(35, 138),]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_93,
    93,
    vec![PokemonLearnset::new(29, 95), PokemonLearnset::new(38, 138),]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_94,
    94,
    vec![PokemonLearnset::new(29, 95), PokemonLearnset::new(38, 138),]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_95,
    95,
    vec![
        PokemonLearnset::new(15, 20),
        PokemonLearnset::new(19, 88),
        PokemonLearnset::new(25, 99),
        PokemonLearnset::new(33, 21),
        PokemonLearnset::new(43, 106),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_96,
    96,
    vec![
        PokemonLearnset::new(12, 50),
        PokemonLearnset::new(17, 93),
        PokemonLearnset::new(24, 29),
        PokemonLearnset::new(29, 139),
        PokemonLearnset::new(32, 94),
        PokemonLearnset::new(37, 96),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_97,
    97,
    vec![
        PokemonLearnset::new(12, 50),
        PokemonLearnset::new(17, 93),
        PokemonLearnset::new(24, 29),
        PokemonLearnset::new(33, 139),
        PokemonLearnset::new(37, 94),
        PokemonLearnset::new(43, 96),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_98,
    98,
    vec![
        PokemonLearnset::new(20, 11),
        PokemonLearnset::new(25, 12),
        PokemonLearnset::new(30, 23),
        PokemonLearnset::new(35, 152),
        PokemonLearnset::new(40, 106),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_99,
    99,
    vec![
        PokemonLearnset::new(20, 11),
        PokemonLearnset::new(25, 12),
        PokemonLearnset::new(34, 23),
        PokemonLearnset::new(42, 152),
        PokemonLearnset::new(49, 106),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_100,
    100,
    vec![
        PokemonLearnset::new(17, 49),
        PokemonLearnset::new(22, 120),
        PokemonLearnset::new(29, 113),
        PokemonLearnset::new(36, 129),
        PokemonLearnset::new(43, 153),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_101,
    101,
    vec![
        PokemonLearnset::new(17, 49),
        PokemonLearnset::new(22, 120),
        PokemonLearnset::new(29, 113),
        PokemonLearnset::new(40, 129),
        PokemonLearnset::new(50, 153),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_102,
    102,
    vec![
        PokemonLearnset::new(25, 115),
        PokemonLearnset::new(28, 73),
        PokemonLearnset::new(32, 78),
        PokemonLearnset::new(37, 77),
        PokemonLearnset::new(42, 76),
        PokemonLearnset::new(48, 79),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_103,
    103,
    vec![PokemonLearnset::new(28, 23),]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_104,
    104,
    vec![
        PokemonLearnset::new(25, 43),
        PokemonLearnset::new(31, 116),
        PokemonLearnset::new(38, 37),
        PokemonLearnset::new(43, 155),
        PokemonLearnset::new(46, 99),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_105,
    105,
    vec![
        PokemonLearnset::new(25, 43),
        PokemonLearnset::new(33, 116),
        PokemonLearnset::new(41, 37),
        PokemonLearnset::new(48, 155),
        PokemonLearnset::new(55, 99),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_106,
    106,
    vec![
        PokemonLearnset::new(33, 27),
        PokemonLearnset::new(38, 26),
        PokemonLearnset::new(43, 116),
        PokemonLearnset::new(48, 136),
        PokemonLearnset::new(53, 25),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_107,
    107,
    vec![
        PokemonLearnset::new(33, 7),
        PokemonLearnset::new(38, 8),
        PokemonLearnset::new(43, 9),
        PokemonLearnset::new(48, 5),
        PokemonLearnset::new(53, 68),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_108,
    108,
    vec![
        PokemonLearnset::new(7, 23),
        PokemonLearnset::new(15, 50),
        PokemonLearnset::new(23, 111),
        PokemonLearnset::new(31, 21),
        PokemonLearnset::new(39, 103),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_109,
    109,
    vec![
        PokemonLearnset::new(32, 124),
        PokemonLearnset::new(37, 108),
        PokemonLearnset::new(40, 120),
        PokemonLearnset::new(45, 114),
        PokemonLearnset::new(48, 153),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_110,
    110,
    vec![
        PokemonLearnset::new(32, 124),
        PokemonLearnset::new(39, 108),
        PokemonLearnset::new(43, 120),
        PokemonLearnset::new(49, 114),
        PokemonLearnset::new(53, 153),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_111,
    111,
    vec![
        PokemonLearnset::new(30, 23),
        PokemonLearnset::new(35, 39),
        PokemonLearnset::new(40, 31),
        PokemonLearnset::new(45, 32),
        PokemonLearnset::new(50, 43),
        PokemonLearnset::new(55, 36),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_112,
    112,
    vec![
        PokemonLearnset::new(30, 23),
        PokemonLearnset::new(35, 39),
        PokemonLearnset::new(40, 31),
        PokemonLearnset::new(48, 32),
        PokemonLearnset::new(55, 43),
        PokemonLearnset::new(64, 36),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_113,
    113,
    vec![
        PokemonLearnset::new(24, 47),
        PokemonLearnset::new(30, 45),
        PokemonLearnset::new(38, 107),
        PokemonLearnset::new(44, 111),
        PokemonLearnset::new(48, 113),
        PokemonLearnset::new(54, 38),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_114,
    114,
    vec![
        PokemonLearnset::new(29, 71),
        PokemonLearnset::new(32, 77),
        PokemonLearnset::new(36, 78),
        PokemonLearnset::new(39, 79),
        PokemonLearnset::new(45, 21),
        PokemonLearnset::new(49, 74),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_115,
    115,
    vec![
        PokemonLearnset::new(26, 44),
        PokemonLearnset::new(31, 39),
        PokemonLearnset::new(36, 5),
        PokemonLearnset::new(41, 43),
        PokemonLearnset::new(46, 146),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_116,
    116,
    vec![
        PokemonLearnset::new(19, 108),
        PokemonLearnset::new(24, 43),
        PokemonLearnset::new(30, 55),
        PokemonLearnset::new(37, 97),
        PokemonLearnset::new(45, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_117,
    117,
    vec![
        PokemonLearnset::new(19, 108),
        PokemonLearnset::new(24, 43),
        PokemonLearnset::new(30, 55),
        PokemonLearnset::new(41, 97),
        PokemonLearnset::new(52, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_118,
    118,
    vec![
        PokemonLearnset::new(19, 48),
        PokemonLearnset::new(24, 30),
        PokemonLearnset::new(30, 31),
        PokemonLearnset::new(37, 127),
        PokemonLearnset::new(45, 32),
        PokemonLearnset::new(54, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_119,
    119,
    vec![
        PokemonLearnset::new(19, 48),
        PokemonLearnset::new(24, 30),
        PokemonLearnset::new(30, 31),
        PokemonLearnset::new(39, 127),
        PokemonLearnset::new(48, 32),
        PokemonLearnset::new(54, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_120,
    120,
    vec![
        PokemonLearnset::new(17, 55),
        PokemonLearnset::new(22, 106),
        PokemonLearnset::new(27, 105),
        PokemonLearnset::new(32, 129),
        PokemonLearnset::new(37, 107),
        PokemonLearnset::new(42, 113),
        PokemonLearnset::new(47, 56),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_121, 121, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_122,
    122,
    vec![
        PokemonLearnset::new(15, 93),
        PokemonLearnset::new(23, 113),
        PokemonLearnset::new(31, 3),
        PokemonLearnset::new(39, 96),
        PokemonLearnset::new(47, 164),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_123,
    123,
    vec![
        PokemonLearnset::new(17, 43),
        PokemonLearnset::new(20, 116),
        PokemonLearnset::new(24, 104),
        PokemonLearnset::new(29, 163),
        PokemonLearnset::new(35, 14),
        PokemonLearnset::new(42, 97),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_124,
    124,
    vec![
        PokemonLearnset::new(18, 122),
        PokemonLearnset::new(23, 3),
        PokemonLearnset::new(31, 8),
        PokemonLearnset::new(39, 34),
        PokemonLearnset::new(47, 37),
        PokemonLearnset::new(58, 59),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_125,
    125,
    vec![
        PokemonLearnset::new(34, 84),
        PokemonLearnset::new(37, 103),
        PokemonLearnset::new(42, 9),
        PokemonLearnset::new(49, 113),
        PokemonLearnset::new(54, 87),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_126,
    126,
    vec![
        PokemonLearnset::new(36, 43),
        PokemonLearnset::new(39, 109),
        PokemonLearnset::new(43, 7),
        PokemonLearnset::new(48, 108),
        PokemonLearnset::new(52, 123),
        PokemonLearnset::new(55, 53),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_127,
    127,
    vec![
        PokemonLearnset::new(25, 69),
        PokemonLearnset::new(30, 12),
        PokemonLearnset::new(36, 116),
        PokemonLearnset::new(43, 106),
        PokemonLearnset::new(49, 163),
        PokemonLearnset::new(54, 14),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_128,
    128,
    vec![
        PokemonLearnset::new(21, 23),
        PokemonLearnset::new(28, 39),
        PokemonLearnset::new(35, 43),
        PokemonLearnset::new(44, 99),
        PokemonLearnset::new(51, 36),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_129,
    129,
    vec![PokemonLearnset::new(15, 33),]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_130,
    130,
    vec![
        PokemonLearnset::new(20, 44),
        PokemonLearnset::new(25, 82),
        PokemonLearnset::new(32, 43),
        PokemonLearnset::new(41, 56),
        PokemonLearnset::new(52, 63),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_131,
    131,
    vec![
        PokemonLearnset::new(16, 47),
        PokemonLearnset::new(20, 54),
        PokemonLearnset::new(25, 34),
        PokemonLearnset::new(31, 109),
        PokemonLearnset::new(38, 58),
        PokemonLearnset::new(46, 56),
    ]
);
get_pokemon_learnset_test!(get_pokemon_learnset_132, 132, vec![]);
get_pokemon_learnset_test!(
    get_pokemon_learnset_133,
    133,
    vec![
        PokemonLearnset::new(27, 98),
        PokemonLearnset::new(31, 39),
        PokemonLearnset::new(37, 44),
        PokemonLearnset::new(45, 36),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_134,
    134,
    vec![
        PokemonLearnset::new(27, 98),
        PokemonLearnset::new(31, 55),
        PokemonLearnset::new(37, 39),
        PokemonLearnset::new(40, 44),
        PokemonLearnset::new(42, 151),
        PokemonLearnset::new(44, 114),
        PokemonLearnset::new(48, 54),
        PokemonLearnset::new(54, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_135,
    135,
    vec![
        PokemonLearnset::new(27, 98),
        PokemonLearnset::new(31, 84),
        PokemonLearnset::new(37, 39),
        PokemonLearnset::new(40, 86),
        PokemonLearnset::new(42, 24),
        PokemonLearnset::new(44, 97),
        PokemonLearnset::new(48, 42),
        PokemonLearnset::new(54, 87),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_136,
    136,
    vec![
        PokemonLearnset::new(27, 98),
        PokemonLearnset::new(31, 52),
        PokemonLearnset::new(37, 39),
        PokemonLearnset::new(40, 44),
        PokemonLearnset::new(42, 43),
        PokemonLearnset::new(44, 83),
        PokemonLearnset::new(48, 99),
        PokemonLearnset::new(54, 53),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_137,
    137,
    vec![
        PokemonLearnset::new(23, 60),
        PokemonLearnset::new(28, 105),
        PokemonLearnset::new(35, 97),
        PokemonLearnset::new(42, 161),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_138,
    138,
    vec![
        PokemonLearnset::new(34, 30),
        PokemonLearnset::new(39, 43),
        PokemonLearnset::new(46, 131),
        PokemonLearnset::new(53, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_139,
    139,
    vec![
        PokemonLearnset::new(34, 30),
        PokemonLearnset::new(39, 43),
        PokemonLearnset::new(44, 131),
        PokemonLearnset::new(49, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_140,
    140,
    vec![
        PokemonLearnset::new(34, 71),
        PokemonLearnset::new(39, 163),
        PokemonLearnset::new(44, 43),
        PokemonLearnset::new(49, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_141,
    141,
    vec![
        PokemonLearnset::new(34, 71),
        PokemonLearnset::new(39, 163),
        PokemonLearnset::new(46, 43),
        PokemonLearnset::new(53, 56),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_142,
    142,
    vec![
        PokemonLearnset::new(33, 48),
        PokemonLearnset::new(38, 44),
        PokemonLearnset::new(45, 36),
        PokemonLearnset::new(54, 63),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_143,
    143,
    vec![
        PokemonLearnset::new(35, 34),
        PokemonLearnset::new(41, 106),
        PokemonLearnset::new(48, 38),
        PokemonLearnset::new(56, 63),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_144,
    144,
    vec![
        PokemonLearnset::new(51, 59),
        PokemonLearnset::new(55, 97),
        PokemonLearnset::new(60, 54),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_145,
    145,
    vec![
        PokemonLearnset::new(51, 87),
        PokemonLearnset::new(55, 97),
        PokemonLearnset::new(60, 113),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_146,
    146,
    vec![
        PokemonLearnset::new(51, 43),
        PokemonLearnset::new(55, 97),
        PokemonLearnset::new(60, 143),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_147,
    147,
    vec![
        PokemonLearnset::new(10, 86),
        PokemonLearnset::new(20, 97),
        PokemonLearnset::new(30, 21),
        PokemonLearnset::new(40, 82),
        PokemonLearnset::new(50, 63),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_148,
    148,
    vec![
        PokemonLearnset::new(10, 86),
        PokemonLearnset::new(20, 97),
        PokemonLearnset::new(35, 21),
        PokemonLearnset::new(45, 82),
        PokemonLearnset::new(55, 63),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_149,
    149,
    vec![
        PokemonLearnset::new(10, 86),
        PokemonLearnset::new(20, 97),
        PokemonLearnset::new(35, 21),
        PokemonLearnset::new(45, 82),
        PokemonLearnset::new(60, 63),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_150,
    150,
    vec![
        PokemonLearnset::new(63, 112),
        PokemonLearnset::new(66, 94),
        PokemonLearnset::new(70, 105),
        PokemonLearnset::new(75, 54),
        PokemonLearnset::new(81, 133),
    ]
);
get_pokemon_learnset_test!(
    get_pokemon_learnset_151,
    151,
    vec![
        PokemonLearnset::new(10, 144),
        PokemonLearnset::new(20, 5),
        PokemonLearnset::new(30, 118),
        PokemonLearnset::new(40, 94),
    ]
);
