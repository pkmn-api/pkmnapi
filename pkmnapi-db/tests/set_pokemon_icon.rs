use pkmnapi_db::patch::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! set_pokemon_icon_test {
    (
        $test_name:ident,
        $pokedex_id:expr,
        $pokemon_icon:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_pokemon_icon(&$pokedex_id, &PokemonIcon::from(&$pokemon_icon)) {
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

#[rustfmt::skip::macros(set_pokemon_icon_test)]

set_pokemon_icon_test!(set_pokemon_icon_1, 1, 0x02, 0x7190D, vec![0x27]);
set_pokemon_icon_test!(set_pokemon_icon_2, 2, 0x02, 0x7190D, vec![0x72]);
set_pokemon_icon_test!(set_pokemon_icon_3, 3, 0x02, 0x7190E, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_4, 4, 0x02, 0x7190E, vec![0x72]);
set_pokemon_icon_test!(set_pokemon_icon_5, 5, 0x02, 0x7190F, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_6, 6, 0x02, 0x7190F, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_7, 7, 0x02, 0x71910, vec![0x25]);
set_pokemon_icon_test!(set_pokemon_icon_8, 8, 0x02, 0x71910, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_9, 9, 0x02, 0x71911, vec![0x26]);
set_pokemon_icon_test!(set_pokemon_icon_10, 10, 0x02, 0x71911, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_11, 11, 0x02, 0x71912, vec![0x26]);
set_pokemon_icon_test!(set_pokemon_icon_12, 12, 0x02, 0x71912, vec![0x62]);
set_pokemon_icon_test!(set_pokemon_icon_13, 13, 0x02, 0x71913, vec![0x26]);
set_pokemon_icon_test!(set_pokemon_icon_14, 14, 0x02, 0x71913, vec![0x62]);
set_pokemon_icon_test!(set_pokemon_icon_15, 15, 0x02, 0x71914, vec![0x24]);
set_pokemon_icon_test!(set_pokemon_icon_16, 16, 0x02, 0x71914, vec![0x62]);
set_pokemon_icon_test!(set_pokemon_icon_17, 17, 0x02, 0x71915, vec![0x24]);
set_pokemon_icon_test!(set_pokemon_icon_18, 18, 0x02, 0x71915, vec![0x42]);
set_pokemon_icon_test!(set_pokemon_icon_19, 19, 0x02, 0x71916, vec![0x29]);
set_pokemon_icon_test!(set_pokemon_icon_20, 20, 0x02, 0x71916, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_21, 21, 0x02, 0x71917, vec![0x24]);
set_pokemon_icon_test!(set_pokemon_icon_22, 22, 0x02, 0x71917, vec![0x42]);
set_pokemon_icon_test!(set_pokemon_icon_23, 23, 0x02, 0x71918, vec![0x28]);
set_pokemon_icon_test!(set_pokemon_icon_24, 24, 0x02, 0x71918, vec![0x82]);
set_pokemon_icon_test!(set_pokemon_icon_25, 25, 0x02, 0x71919, vec![0x23]);
set_pokemon_icon_test!(set_pokemon_icon_26, 26, 0x02, 0x71919, vec![0x32]);
set_pokemon_icon_test!(set_pokemon_icon_27, 27, 0x02, 0x7191A, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_28, 28, 0x02, 0x7191A, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_29, 29, 0x02, 0x7191B, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_30, 30, 0x02, 0x7191B, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_31, 31, 0x02, 0x7191C, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_32, 32, 0x02, 0x7191C, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_33, 33, 0x02, 0x7191D, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_34, 34, 0x02, 0x7191D, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_35, 35, 0x02, 0x7191E, vec![0x23]);
set_pokemon_icon_test!(set_pokemon_icon_36, 36, 0x02, 0x7191E, vec![0x32]);
set_pokemon_icon_test!(set_pokemon_icon_37, 37, 0x02, 0x7191F, vec![0x29]);
set_pokemon_icon_test!(set_pokemon_icon_38, 38, 0x02, 0x7191F, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_39, 39, 0x02, 0x71920, vec![0x23]);
set_pokemon_icon_test!(set_pokemon_icon_40, 40, 0x02, 0x71920, vec![0x32]);
set_pokemon_icon_test!(set_pokemon_icon_41, 41, 0x02, 0x71921, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_42, 42, 0x02, 0x71921, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_43, 43, 0x02, 0x71922, vec![0x27]);
set_pokemon_icon_test!(set_pokemon_icon_44, 44, 0x02, 0x71922, vec![0x72]);
set_pokemon_icon_test!(set_pokemon_icon_45, 45, 0x02, 0x71923, vec![0x26]);
set_pokemon_icon_test!(set_pokemon_icon_46, 46, 0x02, 0x71923, vec![0x72]);
set_pokemon_icon_test!(set_pokemon_icon_47, 47, 0x02, 0x71924, vec![0x26]);
set_pokemon_icon_test!(set_pokemon_icon_48, 48, 0x02, 0x71924, vec![0x62]);
set_pokemon_icon_test!(set_pokemon_icon_49, 49, 0x02, 0x71925, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_50, 50, 0x02, 0x71925, vec![0x62]);
set_pokemon_icon_test!(set_pokemon_icon_51, 51, 0x02, 0x71926, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_52, 52, 0x02, 0x71926, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_53, 53, 0x02, 0x71927, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_54, 54, 0x02, 0x71927, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_55, 55, 0x02, 0x71928, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_56, 56, 0x02, 0x71928, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_57, 57, 0x02, 0x71929, vec![0x29]);
set_pokemon_icon_test!(set_pokemon_icon_58, 58, 0x02, 0x71929, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_59, 59, 0x02, 0x7192A, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_60, 60, 0x02, 0x7192A, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_61, 61, 0x02, 0x7192B, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_62, 62, 0x02, 0x7192B, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_63, 63, 0x02, 0x7192C, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_64, 64, 0x02, 0x7192C, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_65, 65, 0x02, 0x7192D, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_66, 66, 0x02, 0x7192D, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_67, 67, 0x02, 0x7192E, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_68, 68, 0x02, 0x7192E, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_69, 69, 0x02, 0x7192F, vec![0x27]);
set_pokemon_icon_test!(set_pokemon_icon_70, 70, 0x02, 0x7192F, vec![0x72]);
set_pokemon_icon_test!(set_pokemon_icon_71, 71, 0x02, 0x71930, vec![0x25]);
set_pokemon_icon_test!(set_pokemon_icon_72, 72, 0x02, 0x71930, vec![0x72]);
set_pokemon_icon_test!(set_pokemon_icon_73, 73, 0x02, 0x71931, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_74, 74, 0x02, 0x71931, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_75, 75, 0x02, 0x71932, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_76, 76, 0x02, 0x71932, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_77, 77, 0x02, 0x71933, vec![0x29]);
set_pokemon_icon_test!(set_pokemon_icon_78, 78, 0x02, 0x71933, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_79, 79, 0x02, 0x71934, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_80, 80, 0x02, 0x71934, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_81, 81, 0x02, 0x71935, vec![0x21]);
set_pokemon_icon_test!(set_pokemon_icon_82, 82, 0x02, 0x71935, vec![0x12]);
set_pokemon_icon_test!(set_pokemon_icon_83, 83, 0x02, 0x71936, vec![0x24]);
set_pokemon_icon_test!(set_pokemon_icon_84, 84, 0x02, 0x71936, vec![0x42]);
set_pokemon_icon_test!(set_pokemon_icon_85, 85, 0x02, 0x71937, vec![0x25]);
set_pokemon_icon_test!(set_pokemon_icon_86, 86, 0x02, 0x71937, vec![0x42]);
set_pokemon_icon_test!(set_pokemon_icon_87, 87, 0x02, 0x71938, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_88, 88, 0x02, 0x71938, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_89, 89, 0x02, 0x71939, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_90, 90, 0x02, 0x71939, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_91, 91, 0x02, 0x7193A, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_92, 92, 0x02, 0x7193A, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_93, 93, 0x02, 0x7193B, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_94, 94, 0x02, 0x7193B, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_95, 95, 0x02, 0x7193C, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_96, 96, 0x02, 0x7193C, vec![0x82]);
set_pokemon_icon_test!(set_pokemon_icon_97, 97, 0x02, 0x7193D, vec![0x25]);
set_pokemon_icon_test!(set_pokemon_icon_98, 98, 0x02, 0x7193D, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_99, 99, 0x02, 0x7193E, vec![0x21]);
set_pokemon_icon_test!(set_pokemon_icon_100, 100, 0x02, 0x7193E, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_101, 101, 0x02, 0x7193F, vec![0x27]);
set_pokemon_icon_test!(set_pokemon_icon_102, 102, 0x02, 0x7193F, vec![0x12]);
set_pokemon_icon_test!(set_pokemon_icon_103, 103, 0x02, 0x71940, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_104, 104, 0x02, 0x71940, vec![0x72]);
set_pokemon_icon_test!(set_pokemon_icon_105, 105, 0x02, 0x71941, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_106, 106, 0x02, 0x71941, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_107, 107, 0x02, 0x71942, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_108, 108, 0x02, 0x71942, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_109, 109, 0x02, 0x71943, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_110, 110, 0x02, 0x71943, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_111, 111, 0x02, 0x71944, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_112, 112, 0x02, 0x71944, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_113, 113, 0x02, 0x71945, vec![0x27]);
set_pokemon_icon_test!(set_pokemon_icon_114, 114, 0x02, 0x71945, vec![0x32]);
set_pokemon_icon_test!(set_pokemon_icon_115, 115, 0x02, 0x71946, vec![0x25]);
set_pokemon_icon_test!(set_pokemon_icon_116, 116, 0x02, 0x71946, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_117, 117, 0x02, 0x71947, vec![0x25]);
set_pokemon_icon_test!(set_pokemon_icon_118, 118, 0x02, 0x71947, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_119, 119, 0x02, 0x71948, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_120, 120, 0x02, 0x71948, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_121, 121, 0x02, 0x71949, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_122, 122, 0x02, 0x71949, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_123, 123, 0x02, 0x7194A, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_124, 124, 0x02, 0x7194A, vec![0x62]);
set_pokemon_icon_test!(set_pokemon_icon_125, 125, 0x02, 0x7194B, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_126, 126, 0x02, 0x7194B, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_127, 127, 0x02, 0x7194C, vec![0x29]);
set_pokemon_icon_test!(set_pokemon_icon_128, 128, 0x02, 0x7194C, vec![0x62]);
set_pokemon_icon_test!(set_pokemon_icon_129, 129, 0x02, 0x7194D, vec![0x28]);
set_pokemon_icon_test!(set_pokemon_icon_130, 130, 0x02, 0x7194D, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_131, 131, 0x02, 0x7194E, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_132, 132, 0x02, 0x7194E, vec![0x52]);
set_pokemon_icon_test!(set_pokemon_icon_133, 133, 0x02, 0x7194F, vec![0x29]);
set_pokemon_icon_test!(set_pokemon_icon_134, 134, 0x02, 0x7194F, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_135, 135, 0x02, 0x71950, vec![0x29]);
set_pokemon_icon_test!(set_pokemon_icon_136, 136, 0x02, 0x71950, vec![0x92]);
set_pokemon_icon_test!(set_pokemon_icon_137, 137, 0x02, 0x71951, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_138, 138, 0x02, 0x71951, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_139, 139, 0x02, 0x71952, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_140, 140, 0x02, 0x71952, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_141, 141, 0x02, 0x71953, vec![0x24]);
set_pokemon_icon_test!(set_pokemon_icon_142, 142, 0x02, 0x71953, vec![0x22]);
set_pokemon_icon_test!(set_pokemon_icon_143, 143, 0x02, 0x71954, vec![0x24]);
set_pokemon_icon_test!(set_pokemon_icon_144, 144, 0x02, 0x71954, vec![0x02]);
set_pokemon_icon_test!(set_pokemon_icon_145, 145, 0x02, 0x71955, vec![0x24]);
set_pokemon_icon_test!(set_pokemon_icon_146, 146, 0x02, 0x71955, vec![0x42]);
set_pokemon_icon_test!(set_pokemon_icon_147, 147, 0x02, 0x71956, vec![0x28]);
set_pokemon_icon_test!(set_pokemon_icon_148, 148, 0x02, 0x71956, vec![0x82]);
set_pokemon_icon_test!(set_pokemon_icon_149, 149, 0x02, 0x71957, vec![0x20]);
set_pokemon_icon_test!(set_pokemon_icon_150, 150, 0x02, 0x71957, vec![0x82]);
set_pokemon_icon_test!(set_pokemon_icon_151, 151, 0x02, 0x71958, vec![0x20]);
