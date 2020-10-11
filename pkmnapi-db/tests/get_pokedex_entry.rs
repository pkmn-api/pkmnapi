use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! get_pokedex_entry_test {
    ($test_name:ident, $pokedex_id:expr, $species:expr, $height:expr, $weight:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_pokedex_entry(&$pokedex_id) {
                Ok(pokedex_entry) => assert_eq!(
                    pokedex_entry,
                    PokedexEntry {
                        species: ROMString::from($species),
                        height: $height,
                        weight: $weight
                    },
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_pokedex_entry_test)]

get_pokedex_entry_test!(get_pokedex_entry_1, 1, "SEED", 28, 150);
get_pokedex_entry_test!(get_pokedex_entry_2, 2, "SEED", 39, 290);
get_pokedex_entry_test!(get_pokedex_entry_3, 3, "SEED", 79, 2210);
get_pokedex_entry_test!(get_pokedex_entry_4, 4, "LIZARD", 24, 190);
get_pokedex_entry_test!(get_pokedex_entry_5, 5, "FLAME", 43, 420);
get_pokedex_entry_test!(get_pokedex_entry_6, 6, "FLAME", 67, 2000);
get_pokedex_entry_test!(get_pokedex_entry_7, 7, "TINYTURTLE", 20, 200);
get_pokedex_entry_test!(get_pokedex_entry_8, 8, "TURTLE", 39, 500);
get_pokedex_entry_test!(get_pokedex_entry_9, 9, "SHELLFISH", 63, 1890);
get_pokedex_entry_test!(get_pokedex_entry_10, 10, "WORM", 12, 60);
get_pokedex_entry_test!(get_pokedex_entry_11, 11, "COCOON", 28, 220);
get_pokedex_entry_test!(get_pokedex_entry_12, 12, "BUTTERFLY", 43, 710);
get_pokedex_entry_test!(get_pokedex_entry_13, 13, "HAIRY BUG", 12, 70);
get_pokedex_entry_test!(get_pokedex_entry_14, 14, "COCOON", 24, 220);
get_pokedex_entry_test!(get_pokedex_entry_15, 15, "POISON BEE", 39, 650);
get_pokedex_entry_test!(get_pokedex_entry_16, 16, "TINY BIRD", 12, 40);
get_pokedex_entry_test!(get_pokedex_entry_17, 17, "BIRD", 43, 660);
get_pokedex_entry_test!(get_pokedex_entry_18, 18, "BIRD", 59, 870);
get_pokedex_entry_test!(get_pokedex_entry_19, 19, "RAT", 12, 80);
get_pokedex_entry_test!(get_pokedex_entry_20, 20, "RAT", 28, 410);
get_pokedex_entry_test!(get_pokedex_entry_21, 21, "TINY BIRD", 12, 40);
get_pokedex_entry_test!(get_pokedex_entry_22, 22, "BEAK", 47, 840);
get_pokedex_entry_test!(get_pokedex_entry_23, 23, "SNAKE", 79, 150);
get_pokedex_entry_test!(get_pokedex_entry_24, 24, "COBRA", 138, 1430);
get_pokedex_entry_test!(get_pokedex_entry_25, 25, "MOUSE", 16, 130);
get_pokedex_entry_test!(get_pokedex_entry_26, 26, "MOUSE", 31, 660);
get_pokedex_entry_test!(get_pokedex_entry_27, 27, "MOUSE", 24, 260);
get_pokedex_entry_test!(get_pokedex_entry_28, 28, "MOUSE", 39, 650);
get_pokedex_entry_test!(get_pokedex_entry_29, 29, "POISON PIN", 16, 150);
get_pokedex_entry_test!(get_pokedex_entry_30, 30, "POISON PIN", 31, 440);
get_pokedex_entry_test!(get_pokedex_entry_31, 31, "DRILL", 51, 1320);
get_pokedex_entry_test!(get_pokedex_entry_32, 32, "POISON PIN", 20, 200);
get_pokedex_entry_test!(get_pokedex_entry_33, 33, "POISON PIN", 35, 430);
get_pokedex_entry_test!(get_pokedex_entry_34, 34, "DRILL", 55, 1370);
get_pokedex_entry_test!(get_pokedex_entry_35, 35, "FAIRY", 24, 170);
get_pokedex_entry_test!(get_pokedex_entry_36, 36, "FAIRY", 51, 880);
get_pokedex_entry_test!(get_pokedex_entry_37, 37, "FOX", 24, 220);
get_pokedex_entry_test!(get_pokedex_entry_38, 38, "FOX", 43, 440);
get_pokedex_entry_test!(get_pokedex_entry_39, 39, "BALLOON", 20, 120);
get_pokedex_entry_test!(get_pokedex_entry_40, 40, "BALLOON", 39, 260);
get_pokedex_entry_test!(get_pokedex_entry_41, 41, "BAT", 31, 170);
get_pokedex_entry_test!(get_pokedex_entry_42, 42, "BAT", 63, 1210);
get_pokedex_entry_test!(get_pokedex_entry_43, 43, "WEED", 20, 120);
get_pokedex_entry_test!(get_pokedex_entry_44, 44, "WEED", 31, 190);
get_pokedex_entry_test!(get_pokedex_entry_45, 45, "FLOWER", 47, 410);
get_pokedex_entry_test!(get_pokedex_entry_46, 46, "MUSHROOM", 12, 120);
get_pokedex_entry_test!(get_pokedex_entry_47, 47, "MUSHROOM", 39, 650);
get_pokedex_entry_test!(get_pokedex_entry_48, 48, "INSECT", 39, 660);
get_pokedex_entry_test!(get_pokedex_entry_49, 49, "POISONMOTH", 59, 280);
get_pokedex_entry_test!(get_pokedex_entry_50, 50, "MOLE", 8, 20);
get_pokedex_entry_test!(get_pokedex_entry_51, 51, "MOLE", 28, 730);
get_pokedex_entry_test!(get_pokedex_entry_52, 52, "SCRATCHCAT", 16, 90);
get_pokedex_entry_test!(get_pokedex_entry_53, 53, "CLASSY CAT", 39, 710);
get_pokedex_entry_test!(get_pokedex_entry_54, 54, "DUCK", 31, 430);
get_pokedex_entry_test!(get_pokedex_entry_55, 55, "DUCK", 67, 1690);
get_pokedex_entry_test!(get_pokedex_entry_56, 56, "PIG MONKEY", 20, 620);
get_pokedex_entry_test!(get_pokedex_entry_57, 57, "PIG MONKEY", 39, 710);
get_pokedex_entry_test!(get_pokedex_entry_58, 58, "PUPPY", 28, 420);
get_pokedex_entry_test!(get_pokedex_entry_59, 59, "LEGENDARY", 75, 3420);
get_pokedex_entry_test!(get_pokedex_entry_60, 60, "TADPOLE", 24, 270);
get_pokedex_entry_test!(get_pokedex_entry_61, 61, "TADPOLE", 39, 440);
get_pokedex_entry_test!(get_pokedex_entry_62, 62, "TADPOLE", 51, 1190);
get_pokedex_entry_test!(get_pokedex_entry_63, 63, "PSI", 35, 430);
get_pokedex_entry_test!(get_pokedex_entry_64, 64, "PSI", 51, 1250);
get_pokedex_entry_test!(get_pokedex_entry_65, 65, "PSI", 59, 1060);
get_pokedex_entry_test!(get_pokedex_entry_66, 66, "SUPERPOWER", 31, 430);
get_pokedex_entry_test!(get_pokedex_entry_67, 67, "SUPERPOWER", 59, 1550);
get_pokedex_entry_test!(get_pokedex_entry_68, 68, "SUPERPOWER", 63, 2870);
get_pokedex_entry_test!(get_pokedex_entry_69, 69, "FLOWER", 28, 90);
get_pokedex_entry_test!(get_pokedex_entry_70, 70, "FLYCATCHER", 39, 140);
get_pokedex_entry_test!(get_pokedex_entry_71, 71, "FLYCATCHER", 67, 340);
get_pokedex_entry_test!(get_pokedex_entry_72, 72, "JELLYFISH", 35, 1000);
get_pokedex_entry_test!(get_pokedex_entry_73, 73, "JELLYFISH", 63, 1210);
get_pokedex_entry_test!(get_pokedex_entry_74, 74, "ROCK", 16, 440);
get_pokedex_entry_test!(get_pokedex_entry_75, 75, "ROCK", 39, 2320);
get_pokedex_entry_test!(get_pokedex_entry_76, 76, "MEGATON", 55, 6620);
get_pokedex_entry_test!(get_pokedex_entry_77, 77, "FIRE HORSE", 39, 660);
get_pokedex_entry_test!(get_pokedex_entry_78, 78, "FIRE HORSE", 67, 2090);
get_pokedex_entry_test!(get_pokedex_entry_79, 79, "DOPEY", 47, 790);
get_pokedex_entry_test!(get_pokedex_entry_80, 80, "HERMITCRAB", 63, 1730);
get_pokedex_entry_test!(get_pokedex_entry_81, 81, "MAGNET", 12, 130);
get_pokedex_entry_test!(get_pokedex_entry_82, 82, "MAGNET", 39, 1320);
get_pokedex_entry_test!(get_pokedex_entry_83, 83, "WILD DUCK", 31, 330);
get_pokedex_entry_test!(get_pokedex_entry_84, 84, "TWIN BIRD", 55, 860);
get_pokedex_entry_test!(get_pokedex_entry_85, 85, "TRIPLEBIRD", 71, 1880);
get_pokedex_entry_test!(get_pokedex_entry_86, 86, "SEA LION", 43, 1980);
get_pokedex_entry_test!(get_pokedex_entry_87, 87, "SEA LION", 67, 2650);
get_pokedex_entry_test!(get_pokedex_entry_88, 88, "SLUDGE", 35, 660);
get_pokedex_entry_test!(get_pokedex_entry_89, 89, "SLUDGE", 47, 660);
get_pokedex_entry_test!(get_pokedex_entry_90, 90, "BIVALVE", 12, 90);
get_pokedex_entry_test!(get_pokedex_entry_91, 91, "BIVALVE", 59, 2920);
get_pokedex_entry_test!(get_pokedex_entry_92, 92, "GAS", 51, 2);
get_pokedex_entry_test!(get_pokedex_entry_93, 93, "GAS", 63, 2);
get_pokedex_entry_test!(get_pokedex_entry_94, 94, "SHADOW", 59, 890);
get_pokedex_entry_test!(get_pokedex_entry_95, 95, "ROCK SNAKE", 346, 4630);
get_pokedex_entry_test!(get_pokedex_entry_96, 96, "HYPNOSIS", 39, 710);
get_pokedex_entry_test!(get_pokedex_entry_97, 97, "HYPNOSIS", 63, 1670);
get_pokedex_entry_test!(get_pokedex_entry_98, 98, "RIVER CRAB", 16, 140);
get_pokedex_entry_test!(get_pokedex_entry_99, 99, "PINCER", 51, 1320);
get_pokedex_entry_test!(get_pokedex_entry_100, 100, "BALL", 20, 230);
get_pokedex_entry_test!(get_pokedex_entry_101, 101, "BALL", 47, 1470);
get_pokedex_entry_test!(get_pokedex_entry_102, 102, "EGG", 16, 60);
get_pokedex_entry_test!(get_pokedex_entry_103, 103, "COCONUT", 79, 2650);
get_pokedex_entry_test!(get_pokedex_entry_104, 104, "LONELY", 16, 140);
get_pokedex_entry_test!(get_pokedex_entry_105, 105, "BONEKEEPER", 39, 990);
get_pokedex_entry_test!(get_pokedex_entry_106, 106, "KICKING", 59, 1100);
get_pokedex_entry_test!(get_pokedex_entry_107, 107, "PUNCHING", 55, 1110);
get_pokedex_entry_test!(get_pokedex_entry_108, 108, "LICKING", 47, 1440);
get_pokedex_entry_test!(get_pokedex_entry_109, 109, "POISON GAS", 24, 20);
get_pokedex_entry_test!(get_pokedex_entry_110, 110, "POISON GAS", 47, 210);
get_pokedex_entry_test!(get_pokedex_entry_111, 111, "SPIKES", 39, 2540);
get_pokedex_entry_test!(get_pokedex_entry_112, 112, "DRILL", 75, 2650);
get_pokedex_entry_test!(get_pokedex_entry_113, 113, "EGG", 43, 760);
get_pokedex_entry_test!(get_pokedex_entry_114, 114, "VINE", 39, 770);
get_pokedex_entry_test!(get_pokedex_entry_115, 115, "PARENT", 87, 1760);
get_pokedex_entry_test!(get_pokedex_entry_116, 116, "DRAGON", 16, 180);
get_pokedex_entry_test!(get_pokedex_entry_117, 117, "DRAGON", 47, 550);
get_pokedex_entry_test!(get_pokedex_entry_118, 118, "GOLDFISH", 24, 330);
get_pokedex_entry_test!(get_pokedex_entry_119, 119, "GOLDFISH", 51, 860);
get_pokedex_entry_test!(get_pokedex_entry_120, 120, "STARSHAPE", 31, 760);
get_pokedex_entry_test!(get_pokedex_entry_121, 121, "MYSTERIOUS", 43, 1760);
get_pokedex_entry_test!(get_pokedex_entry_122, 122, "BARRIER", 51, 1200);
get_pokedex_entry_test!(get_pokedex_entry_123, 123, "MANTIS", 59, 1230);
get_pokedex_entry_test!(get_pokedex_entry_124, 124, "HUMANSHAPE", 55, 900);
get_pokedex_entry_test!(get_pokedex_entry_125, 125, "ELECTRIC", 43, 660);
get_pokedex_entry_test!(get_pokedex_entry_126, 126, "SPITFIRE", 51, 980);
get_pokedex_entry_test!(get_pokedex_entry_127, 127, "STAGBEETLE", 59, 1210);
get_pokedex_entry_test!(get_pokedex_entry_128, 128, "WILD BULL", 55, 1950);
get_pokedex_entry_test!(get_pokedex_entry_129, 129, "FISH", 35, 220);
get_pokedex_entry_test!(get_pokedex_entry_130, 130, "ATROCIOUS", 256, 5180);
get_pokedex_entry_test!(get_pokedex_entry_131, 131, "TRANSPORT", 98, 4850);
get_pokedex_entry_test!(get_pokedex_entry_132, 132, "TRANSFORM", 12, 90);
get_pokedex_entry_test!(get_pokedex_entry_133, 133, "EVOLUTION", 12, 140);
get_pokedex_entry_test!(get_pokedex_entry_134, 134, "BUBBLE JET", 39, 640);
get_pokedex_entry_test!(get_pokedex_entry_135, 135, "LIGHTNING", 31, 540);
get_pokedex_entry_test!(get_pokedex_entry_136, 136, "FLAME", 35, 550);
get_pokedex_entry_test!(get_pokedex_entry_137, 137, "VIRTUAL", 31, 800);
get_pokedex_entry_test!(get_pokedex_entry_138, 138, "SPIRAL", 16, 170);
get_pokedex_entry_test!(get_pokedex_entry_139, 139, "SPIRAL", 39, 770);
get_pokedex_entry_test!(get_pokedex_entry_140, 140, "SHELLFISH", 20, 250);
get_pokedex_entry_test!(get_pokedex_entry_141, 141, "SHELLFISH", 51, 890);
get_pokedex_entry_test!(get_pokedex_entry_142, 142, "FOSSIL", 71, 1300);
get_pokedex_entry_test!(get_pokedex_entry_143, 143, "SLEEPING", 83, 10140);
get_pokedex_entry_test!(get_pokedex_entry_144, 144, "FREEZE", 67, 1220);
get_pokedex_entry_test!(get_pokedex_entry_145, 145, "ELECTRIC", 63, 1160);
get_pokedex_entry_test!(get_pokedex_entry_146, 146, "FLAME", 79, 1320);
get_pokedex_entry_test!(get_pokedex_entry_147, 147, "DRAGON", 71, 70);
get_pokedex_entry_test!(get_pokedex_entry_148, 148, "DRAGON", 157, 360);
get_pokedex_entry_test!(get_pokedex_entry_149, 149, "DRAGON", 87, 4630);
get_pokedex_entry_test!(get_pokedex_entry_150, 150, "GENETIC", 79, 2690);
get_pokedex_entry_test!(get_pokedex_entry_151, 151, "NEW SPECIE", 16, 90);
