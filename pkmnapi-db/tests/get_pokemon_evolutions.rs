use pkmnapi_db::*;

mod common;

macro_rules! get_pokemon_evolutions_test {
    ($test_name:ident, $pokedex_id:expr, $pokemon_evolutions:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_pokemon_evolutions(&$pokedex_id) {
                Ok(pokemon_evolutions) => assert_eq!(
                    pokemon_evolutions, $pokemon_evolutions,
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

get_pokemon_evolutions_test!(
    get_pokemon_evolutions_1,
    1,
    vec![PokemonEvolutionLevel::new(2, 16)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_2,
    2,
    vec![PokemonEvolutionLevel::new(3, 32)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_3, 3, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_4,
    4,
    vec![PokemonEvolutionLevel::new(5, 16)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_5,
    5,
    vec![PokemonEvolutionLevel::new(6, 36)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_6, 6, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_7,
    7,
    vec![PokemonEvolutionLevel::new(8, 16)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_8,
    8,
    vec![PokemonEvolutionLevel::new(9, 36)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_9, 9, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_10,
    10,
    vec![PokemonEvolutionLevel::new(11, 7)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_11,
    11,
    vec![PokemonEvolutionLevel::new(12, 10)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_12, 12, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_13,
    13,
    vec![PokemonEvolutionLevel::new(14, 7)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_14,
    14,
    vec![PokemonEvolutionLevel::new(15, 10)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_15, 15, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_16,
    16,
    vec![PokemonEvolutionLevel::new(17, 18)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_17,
    17,
    vec![PokemonEvolutionLevel::new(18, 36)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_18, 18, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_19,
    19,
    vec![PokemonEvolutionLevel::new(20, 20)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_20, 20, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_21,
    21,
    vec![PokemonEvolutionLevel::new(22, 20)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_22, 22, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_23,
    23,
    vec![PokemonEvolutionLevel::new(24, 22)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_24, 24, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_25,
    25,
    vec![PokemonEvolutionItem::new(26, 33)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_26, 26, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_27,
    27,
    vec![PokemonEvolutionLevel::new(28, 22)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_28, 28, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_29,
    29,
    vec![PokemonEvolutionLevel::new(30, 16)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_30,
    30,
    vec![PokemonEvolutionItem::new(31, 10)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_31, 31, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_32,
    32,
    vec![PokemonEvolutionLevel::new(33, 16)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_33,
    33,
    vec![PokemonEvolutionItem::new(34, 10)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_34, 34, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_35,
    35,
    vec![PokemonEvolutionItem::new(36, 10)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_36, 36, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_37,
    37,
    vec![PokemonEvolutionItem::new(38, 32)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_38, 38, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_39,
    39,
    vec![PokemonEvolutionItem::new(40, 10)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_40, 40, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_41,
    41,
    vec![PokemonEvolutionLevel::new(42, 22)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_42, 42, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_43,
    43,
    vec![PokemonEvolutionLevel::new(44, 21)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_44,
    44,
    vec![PokemonEvolutionItem::new(45, 47)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_45, 45, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_46,
    46,
    vec![PokemonEvolutionLevel::new(47, 24)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_47, 47, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_48,
    48,
    vec![PokemonEvolutionLevel::new(49, 31)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_49, 49, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_50,
    50,
    vec![PokemonEvolutionLevel::new(51, 26)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_51, 51, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_52,
    52,
    vec![PokemonEvolutionLevel::new(53, 28)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_53, 53, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_54,
    54,
    vec![PokemonEvolutionLevel::new(55, 33)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_55, 55, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_56,
    56,
    vec![PokemonEvolutionLevel::new(57, 28)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_57, 57, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_58,
    58,
    vec![PokemonEvolutionItem::new(59, 32)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_59, 59, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_60,
    60,
    vec![PokemonEvolutionLevel::new(61, 25)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_61,
    61,
    vec![PokemonEvolutionItem::new(62, 34)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_62, 62, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_63,
    63,
    vec![PokemonEvolutionLevel::new(64, 16)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_64,
    64,
    vec![PokemonEvolutionTrade::new(65)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_65, 65, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_66,
    66,
    vec![PokemonEvolutionLevel::new(67, 28)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_67,
    67,
    vec![PokemonEvolutionTrade::new(68)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_68, 68, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_69,
    69,
    vec![PokemonEvolutionLevel::new(70, 21)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_70,
    70,
    vec![PokemonEvolutionItem::new(71, 47)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_71, 71, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_72,
    72,
    vec![PokemonEvolutionLevel::new(73, 30)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_73, 73, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_74,
    74,
    vec![PokemonEvolutionLevel::new(75, 25)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_75,
    75,
    vec![PokemonEvolutionTrade::new(76)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_76, 76, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_77,
    77,
    vec![PokemonEvolutionLevel::new(78, 40)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_78, 78, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_79,
    79,
    vec![PokemonEvolutionLevel::new(80, 37)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_80, 80, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_81,
    81,
    vec![PokemonEvolutionLevel::new(82, 30)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_82, 82, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_83, 83, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_84,
    84,
    vec![PokemonEvolutionLevel::new(85, 31)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_85, 85, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_86,
    86,
    vec![PokemonEvolutionLevel::new(87, 34)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_87, 87, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_88,
    88,
    vec![PokemonEvolutionLevel::new(89, 38)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_89, 89, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_90,
    90,
    vec![PokemonEvolutionItem::new(91, 34)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_91, 91, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_92,
    92,
    vec![PokemonEvolutionLevel::new(93, 25)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_93,
    93,
    vec![PokemonEvolutionTrade::new(94)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_94, 94, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_95, 95, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_96,
    96,
    vec![PokemonEvolutionLevel::new(97, 26)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_97, 97, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_98,
    98,
    vec![PokemonEvolutionLevel::new(99, 28)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_99, 99, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_100,
    100,
    vec![PokemonEvolutionLevel::new(101, 30)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_101, 101, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_102,
    102,
    vec![PokemonEvolutionItem::new(103, 47)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_103, 103, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_104,
    104,
    vec![PokemonEvolutionLevel::new(105, 28)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_105, 105, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_106, 106, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_107, 107, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_108, 108, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_109,
    109,
    vec![PokemonEvolutionLevel::new(110, 35)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_110, 110, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_111,
    111,
    vec![PokemonEvolutionLevel::new(112, 42)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_112, 112, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_113, 113, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_114, 114, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_115, 115, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_116,
    116,
    vec![PokemonEvolutionLevel::new(117, 32)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_117, 117, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_118,
    118,
    vec![PokemonEvolutionLevel::new(119, 33)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_119, 119, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_120,
    120,
    vec![PokemonEvolutionItem::new(121, 34)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_121, 121, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_122, 122, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_123, 123, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_124, 124, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_125, 125, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_126, 126, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_127, 127, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_128, 128, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_129,
    129,
    vec![PokemonEvolutionLevel::new(130, 20)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_130, 130, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_131, 131, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_132, 132, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_133,
    133,
    vec![
        PokemonEvolutionItem::new(136, 32),
        PokemonEvolutionItem::new(135, 33),
        PokemonEvolutionItem::new(134, 34),
    ]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_134, 134, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_135, 135, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_136, 136, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_137, 137, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_138,
    138,
    vec![PokemonEvolutionLevel::new(139, 40)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_139, 139, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_140,
    140,
    vec![PokemonEvolutionLevel::new(141, 40)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_141, 141, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_142, 142, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_143, 143, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_144, 144, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_145, 145, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_146, 146, vec![]);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_147,
    147,
    vec![PokemonEvolutionLevel::new(148, 30)]
);
get_pokemon_evolutions_test!(
    get_pokemon_evolutions_148,
    148,
    vec![PokemonEvolutionLevel::new(149, 55)]
);
get_pokemon_evolutions_test!(get_pokemon_evolutions_149, 149, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_150, 150, vec![]);
get_pokemon_evolutions_test!(get_pokemon_evolutions_151, 151, vec![]);
