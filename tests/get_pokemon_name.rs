use pkmnapi::db::string::*;
use pkmnapi::db::types::*;

mod common;

macro_rules! get_pokemon_name_test {
    ($test_name: ident, $pokedex_id: expr, $pokemon_name: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_pokemon_name($pokedex_id) {
                Ok(pokemon_name) => assert_eq!(
                    pokemon_name,
                    PokemonName {
                        name: ROMString::from($pokemon_name),
                    },
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_pokemon_name_test)]

get_pokemon_name_test!(get_pokemon_name_1, 1, "BULBASAUR");
get_pokemon_name_test!(get_pokemon_name_2, 2, "IVYSAUR");
get_pokemon_name_test!(get_pokemon_name_3, 3, "VENUSAUR");
get_pokemon_name_test!(get_pokemon_name_4, 4, "CHARMANDER");
get_pokemon_name_test!(get_pokemon_name_5, 5, "CHARMELEON");
get_pokemon_name_test!(get_pokemon_name_6, 6, "CHARIZARD");
get_pokemon_name_test!(get_pokemon_name_7, 7, "SQUIRTLE");
get_pokemon_name_test!(get_pokemon_name_8, 8, "WARTORTLE");
get_pokemon_name_test!(get_pokemon_name_9, 9, "BLASTOISE");
get_pokemon_name_test!(get_pokemon_name_10, 10, "CATERPIE");
get_pokemon_name_test!(get_pokemon_name_11, 11, "METAPOD");
get_pokemon_name_test!(get_pokemon_name_12, 12, "BUTTERFREE");
get_pokemon_name_test!(get_pokemon_name_13, 13, "WEEDLE");
get_pokemon_name_test!(get_pokemon_name_14, 14, "KAKUNA");
get_pokemon_name_test!(get_pokemon_name_15, 15, "BEEDRILL");
get_pokemon_name_test!(get_pokemon_name_16, 16, "PIDGEY");
get_pokemon_name_test!(get_pokemon_name_17, 17, "PIDGEOTTO");
get_pokemon_name_test!(get_pokemon_name_18, 18, "PIDGEOT");
get_pokemon_name_test!(get_pokemon_name_19, 19, "RATTATA");
get_pokemon_name_test!(get_pokemon_name_20, 20, "RATICATE");
get_pokemon_name_test!(get_pokemon_name_21, 21, "SPEAROW");
get_pokemon_name_test!(get_pokemon_name_22, 22, "FEAROW");
get_pokemon_name_test!(get_pokemon_name_23, 23, "EKANS");
get_pokemon_name_test!(get_pokemon_name_24, 24, "ARBOK");
get_pokemon_name_test!(get_pokemon_name_25, 25, "PIKACHU");
get_pokemon_name_test!(get_pokemon_name_26, 26, "RAICHU");
get_pokemon_name_test!(get_pokemon_name_27, 27, "SANDSHREW");
get_pokemon_name_test!(get_pokemon_name_28, 28, "SANDSLASH");
get_pokemon_name_test!(get_pokemon_name_29, 29, "NIDORAN♀");
get_pokemon_name_test!(get_pokemon_name_30, 30, "NIDORINA");
get_pokemon_name_test!(get_pokemon_name_31, 31, "NIDOQUEEN");
get_pokemon_name_test!(get_pokemon_name_32, 32, "NIDORAN♂");
get_pokemon_name_test!(get_pokemon_name_33, 33, "NIDORINO");
get_pokemon_name_test!(get_pokemon_name_34, 34, "NIDOKING");
get_pokemon_name_test!(get_pokemon_name_35, 35, "CLEFAIRY");
get_pokemon_name_test!(get_pokemon_name_36, 36, "CLEFABLE");
get_pokemon_name_test!(get_pokemon_name_37, 37, "VULPIX");
get_pokemon_name_test!(get_pokemon_name_38, 38, "NINETALES");
get_pokemon_name_test!(get_pokemon_name_39, 39, "JIGGLYPUFF");
get_pokemon_name_test!(get_pokemon_name_40, 40, "WIGGLYTUFF");
get_pokemon_name_test!(get_pokemon_name_41, 41, "ZUBAT");
get_pokemon_name_test!(get_pokemon_name_42, 42, "GOLBAT");
get_pokemon_name_test!(get_pokemon_name_43, 43, "ODDISH");
get_pokemon_name_test!(get_pokemon_name_44, 44, "GLOOM");
get_pokemon_name_test!(get_pokemon_name_45, 45, "VILEPLUME");
get_pokemon_name_test!(get_pokemon_name_46, 46, "PARAS");
get_pokemon_name_test!(get_pokemon_name_47, 47, "PARASECT");
get_pokemon_name_test!(get_pokemon_name_48, 48, "VENONAT");
get_pokemon_name_test!(get_pokemon_name_49, 49, "VENOMOTH");
get_pokemon_name_test!(get_pokemon_name_50, 50, "DIGLETT");
get_pokemon_name_test!(get_pokemon_name_51, 51, "DUGTRIO");
get_pokemon_name_test!(get_pokemon_name_52, 52, "MEOWTH");
get_pokemon_name_test!(get_pokemon_name_53, 53, "PERSIAN");
get_pokemon_name_test!(get_pokemon_name_54, 54, "PSYDUCK");
get_pokemon_name_test!(get_pokemon_name_55, 55, "GOLDUCK");
get_pokemon_name_test!(get_pokemon_name_56, 56, "MANKEY");
get_pokemon_name_test!(get_pokemon_name_57, 57, "PRIMEAPE");
get_pokemon_name_test!(get_pokemon_name_58, 58, "GROWLITHE");
get_pokemon_name_test!(get_pokemon_name_59, 59, "ARCANINE");
get_pokemon_name_test!(get_pokemon_name_60, 60, "POLIWAG");
get_pokemon_name_test!(get_pokemon_name_61, 61, "POLIWHIRL");
get_pokemon_name_test!(get_pokemon_name_62, 62, "POLIWRATH");
get_pokemon_name_test!(get_pokemon_name_63, 63, "ABRA");
get_pokemon_name_test!(get_pokemon_name_64, 64, "KADABRA");
get_pokemon_name_test!(get_pokemon_name_65, 65, "ALAKAZAM");
get_pokemon_name_test!(get_pokemon_name_66, 66, "MACHOP");
get_pokemon_name_test!(get_pokemon_name_67, 67, "MACHOKE");
get_pokemon_name_test!(get_pokemon_name_68, 68, "MACHAMP");
get_pokemon_name_test!(get_pokemon_name_69, 69, "BELLSPROUT");
get_pokemon_name_test!(get_pokemon_name_70, 70, "WEEPINBELL");
get_pokemon_name_test!(get_pokemon_name_71, 71, "VICTREEBEL");
get_pokemon_name_test!(get_pokemon_name_72, 72, "TENTACOOL");
get_pokemon_name_test!(get_pokemon_name_73, 73, "TENTACRUEL");
get_pokemon_name_test!(get_pokemon_name_74, 74, "GEODUDE");
get_pokemon_name_test!(get_pokemon_name_75, 75, "GRAVELER");
get_pokemon_name_test!(get_pokemon_name_76, 76, "GOLEM");
get_pokemon_name_test!(get_pokemon_name_77, 77, "PONYTA");
get_pokemon_name_test!(get_pokemon_name_78, 78, "RAPIDASH");
get_pokemon_name_test!(get_pokemon_name_79, 79, "SLOWPOKE");
get_pokemon_name_test!(get_pokemon_name_80, 80, "SLOWBRO");
get_pokemon_name_test!(get_pokemon_name_81, 81, "MAGNEMITE");
get_pokemon_name_test!(get_pokemon_name_82, 82, "MAGNETON");
get_pokemon_name_test!(get_pokemon_name_83, 83, "FARFETCH'D");
get_pokemon_name_test!(get_pokemon_name_84, 84, "DODUO");
get_pokemon_name_test!(get_pokemon_name_85, 85, "DODRIO");
get_pokemon_name_test!(get_pokemon_name_86, 86, "SEEL");
get_pokemon_name_test!(get_pokemon_name_87, 87, "DEWGONG");
get_pokemon_name_test!(get_pokemon_name_88, 88, "GRIMER");
get_pokemon_name_test!(get_pokemon_name_89, 89, "MUK");
get_pokemon_name_test!(get_pokemon_name_90, 90, "SHELLDER");
get_pokemon_name_test!(get_pokemon_name_91, 91, "CLOYSTER");
get_pokemon_name_test!(get_pokemon_name_92, 92, "GASTLY");
get_pokemon_name_test!(get_pokemon_name_93, 93, "HAUNTER");
get_pokemon_name_test!(get_pokemon_name_94, 94, "GENGAR");
get_pokemon_name_test!(get_pokemon_name_95, 95, "ONIX");
get_pokemon_name_test!(get_pokemon_name_96, 96, "DROWZEE");
get_pokemon_name_test!(get_pokemon_name_97, 97, "HYPNO");
get_pokemon_name_test!(get_pokemon_name_98, 98, "KRABBY");
get_pokemon_name_test!(get_pokemon_name_99, 99, "KINGLER");
get_pokemon_name_test!(get_pokemon_name_100, 100, "VOLTORB");
get_pokemon_name_test!(get_pokemon_name_101, 101, "ELECTRODE");
get_pokemon_name_test!(get_pokemon_name_102, 102, "EXEGGCUTE");
get_pokemon_name_test!(get_pokemon_name_103, 103, "EXEGGUTOR");
get_pokemon_name_test!(get_pokemon_name_104, 104, "CUBONE");
get_pokemon_name_test!(get_pokemon_name_105, 105, "MAROWAK");
get_pokemon_name_test!(get_pokemon_name_106, 106, "HITMONLEE");
get_pokemon_name_test!(get_pokemon_name_107, 107, "HITMONCHAN");
get_pokemon_name_test!(get_pokemon_name_108, 108, "LICKITUNG");
get_pokemon_name_test!(get_pokemon_name_109, 109, "KOFFING");
get_pokemon_name_test!(get_pokemon_name_110, 110, "WEEZING");
get_pokemon_name_test!(get_pokemon_name_111, 111, "RHYHORN");
get_pokemon_name_test!(get_pokemon_name_112, 112, "RHYDON");
get_pokemon_name_test!(get_pokemon_name_113, 113, "CHANSEY");
get_pokemon_name_test!(get_pokemon_name_114, 114, "TANGELA");
get_pokemon_name_test!(get_pokemon_name_115, 115, "KANGASKHAN");
get_pokemon_name_test!(get_pokemon_name_116, 116, "HORSEA");
get_pokemon_name_test!(get_pokemon_name_117, 117, "SEADRA");
get_pokemon_name_test!(get_pokemon_name_118, 118, "GOLDEEN");
get_pokemon_name_test!(get_pokemon_name_119, 119, "SEAKING");
get_pokemon_name_test!(get_pokemon_name_120, 120, "STARYU");
get_pokemon_name_test!(get_pokemon_name_121, 121, "STARMIE");
get_pokemon_name_test!(get_pokemon_name_122, 122, "MR.MIME");
get_pokemon_name_test!(get_pokemon_name_123, 123, "SCYTHER");
get_pokemon_name_test!(get_pokemon_name_124, 124, "JYNX");
get_pokemon_name_test!(get_pokemon_name_125, 125, "ELECTABUZZ");
get_pokemon_name_test!(get_pokemon_name_126, 126, "MAGMAR");
get_pokemon_name_test!(get_pokemon_name_127, 127, "PINSIR");
get_pokemon_name_test!(get_pokemon_name_128, 128, "TAUROS");
get_pokemon_name_test!(get_pokemon_name_129, 129, "MAGIKARP");
get_pokemon_name_test!(get_pokemon_name_130, 130, "GYARADOS");
get_pokemon_name_test!(get_pokemon_name_131, 131, "LAPRAS");
get_pokemon_name_test!(get_pokemon_name_132, 132, "DITTO");
get_pokemon_name_test!(get_pokemon_name_133, 133, "EEVEE");
get_pokemon_name_test!(get_pokemon_name_134, 134, "VAPOREON");
get_pokemon_name_test!(get_pokemon_name_135, 135, "JOLTEON");
get_pokemon_name_test!(get_pokemon_name_136, 136, "FLAREON");
get_pokemon_name_test!(get_pokemon_name_137, 137, "PORYGON");
get_pokemon_name_test!(get_pokemon_name_138, 138, "OMANYTE");
get_pokemon_name_test!(get_pokemon_name_139, 139, "OMASTAR");
get_pokemon_name_test!(get_pokemon_name_140, 140, "KABUTO");
get_pokemon_name_test!(get_pokemon_name_141, 141, "KABUTOPS");
get_pokemon_name_test!(get_pokemon_name_142, 142, "AERODACTYL");
get_pokemon_name_test!(get_pokemon_name_143, 143, "SNORLAX");
get_pokemon_name_test!(get_pokemon_name_144, 144, "ARTICUNO");
get_pokemon_name_test!(get_pokemon_name_145, 145, "ZAPDOS");
get_pokemon_name_test!(get_pokemon_name_146, 146, "MOLTRES");
get_pokemon_name_test!(get_pokemon_name_147, 147, "DRATINI");
get_pokemon_name_test!(get_pokemon_name_148, 148, "DRAGONAIR");
get_pokemon_name_test!(get_pokemon_name_149, 149, "DRAGONITE");
get_pokemon_name_test!(get_pokemon_name_150, 150, "MEWTWO");
get_pokemon_name_test!(get_pokemon_name_151, 151, "MEW");
