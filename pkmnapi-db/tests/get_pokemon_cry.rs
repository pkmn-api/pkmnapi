use pkmnapi_db::cry::*;

mod common;

macro_rules! get_pokemon_cry_test {
    ($test_name:ident, $pokedex_id:expr, $base:expr, $pitch:expr, $length:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_pokemon_cry(&$pokedex_id) {
                Ok(pokemon_name) => assert_eq!(
                    Cry {
                        base: pokemon_name.base,
                        pitch: pokemon_name.pitch,
                        length: pokemon_name.length,
                        ..Default::default()
                    },
                    Cry {
                        base: $base,
                        pitch: $pitch,
                        length: $length,
                        ..Default::default()
                    },
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_pokemon_cry_test)]

get_pokemon_cry_test!(get_pokemon_cry_1, 1, 0x0F, 0x80, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_2, 2, 0x0F, 0x20, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_3, 3, 0x0F, 0x00, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_4, 4, 0x04, 0x60, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_5, 5, 0x04, 0x20, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_6, 6, 0x04, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_7, 7, 0x1D, 0x60, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_8, 8, 0x1D, 0x20, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_9, 9, 0x13, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_10, 10, 0x16, 0x80, 0x20);
get_pokemon_cry_test!(get_pokemon_cry_11, 11, 0x1C, 0xCC, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_12, 12, 0x16, 0x77, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_13, 13, 0x15, 0xEE, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_14, 14, 0x13, 0xFF, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_15, 15, 0x13, 0x60, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_16, 16, 0x0E, 0xDF, 0x04);
get_pokemon_cry_test!(get_pokemon_cry_17, 17, 0x14, 0x28, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_18, 18, 0x14, 0x11, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_19, 19, 0x22, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_20, 20, 0x22, 0x20, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_21, 21, 0x10, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_22, 22, 0x18, 0x40, 0xA0);
get_pokemon_cry_test!(get_pokemon_cry_23, 23, 0x17, 0x12, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_24, 24, 0x17, 0xE0, 0x10);
get_pokemon_cry_test!(get_pokemon_cry_25, 25, 0x0F, 0xEE, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_26, 26, 0x09, 0xEE, 0x08);
get_pokemon_cry_test!(get_pokemon_cry_27, 27, 0x00, 0x20, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_28, 28, 0x00, 0xFF, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_29, 29, 0x01, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_30, 30, 0x01, 0x2C, 0xE0);
get_pokemon_cry_test!(get_pokemon_cry_31, 31, 0x0A, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_32, 32, 0x00, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_33, 33, 0x00, 0x2C, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_34, 34, 0x09, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_35, 35, 0x19, 0xCC, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_36, 36, 0x19, 0xAA, 0x20);
get_pokemon_cry_test!(get_pokemon_cry_37, 37, 0x24, 0x4F, 0x10);
get_pokemon_cry_test!(get_pokemon_cry_38, 38, 0x24, 0x88, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_39, 39, 0x0E, 0xFF, 0x35);
get_pokemon_cry_test!(get_pokemon_cry_40, 40, 0x0E, 0x68, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_41, 41, 0x1D, 0xE0, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_42, 42, 0x1D, 0xFA, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_43, 43, 0x08, 0xDD, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_44, 44, 0x08, 0xAA, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_45, 45, 0x23, 0x22, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_46, 46, 0x1E, 0x20, 0xE0);
get_pokemon_cry_test!(get_pokemon_cry_47, 47, 0x1E, 0x42, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_48, 48, 0x1A, 0x44, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_49, 49, 0x1A, 0x29, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_50, 50, 0x0B, 0xAA, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_51, 51, 0x0B, 0x2A, 0x10);
get_pokemon_cry_test!(get_pokemon_cry_52, 52, 0x19, 0x77, 0x10);
get_pokemon_cry_test!(get_pokemon_cry_53, 53, 0x19, 0x99, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_54, 54, 0x21, 0x20, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_55, 55, 0x21, 0xFF, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_56, 56, 0x0A, 0xDD, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_57, 57, 0x0A, 0xAF, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_58, 58, 0x1F, 0x20, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_59, 59, 0x15, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_60, 60, 0x0E, 0xFF, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_61, 61, 0x0E, 0x77, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_62, 62, 0x0E, 0x00, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_63, 63, 0x1C, 0xC0, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_64, 64, 0x1C, 0xA8, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_65, 65, 0x1C, 0x98, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_66, 66, 0x1F, 0xEE, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_67, 67, 0x1F, 0x48, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_68, 68, 0x1F, 0x08, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_69, 69, 0x21, 0x55, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_70, 70, 0x25, 0x44, 0x20);
get_pokemon_cry_test!(get_pokemon_cry_71, 71, 0x25, 0x66, 0xCC);
get_pokemon_cry_test!(get_pokemon_cry_72, 72, 0x1A, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_73, 73, 0x1A, 0xEE, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_74, 74, 0x24, 0xF0, 0x10);
get_pokemon_cry_test!(get_pokemon_cry_75, 75, 0x24, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_76, 76, 0x12, 0xE0, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_77, 77, 0x25, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_78, 78, 0x25, 0x20, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_79, 79, 0x02, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_80, 80, 0x1F, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_81, 81, 0x1C, 0x80, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_82, 82, 0x1C, 0x20, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_83, 83, 0x10, 0xDD, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_84, 84, 0x0B, 0xBB, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_85, 85, 0x0B, 0x99, 0x20);
get_pokemon_cry_test!(get_pokemon_cry_86, 86, 0x0C, 0x88, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_87, 87, 0x0C, 0x23, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_88, 88, 0x05, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_89, 89, 0x07, 0xEF, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_90, 90, 0x18, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_91, 91, 0x18, 0x6F, 0xE0);
get_pokemon_cry_test!(get_pokemon_cry_92, 92, 0x1C, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_93, 93, 0x1C, 0x30, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_94, 94, 0x07, 0x00, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_95, 95, 0x17, 0xFF, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_96, 96, 0x0D, 0x88, 0x20);
get_pokemon_cry_test!(get_pokemon_cry_97, 97, 0x0D, 0xEE, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_98, 98, 0x20, 0x20, 0xE0);
get_pokemon_cry_test!(get_pokemon_cry_99, 99, 0x20, 0xEE, 0xE0);
get_pokemon_cry_test!(get_pokemon_cry_100, 100, 0x06, 0xED, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_101, 101, 0x06, 0xA8, 0x90);
get_pokemon_cry_test!(get_pokemon_cry_102, 102, 0x0B, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_103, 103, 0x0D, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_104, 104, 0x19, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_105, 105, 0x08, 0x4F, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_106, 106, 0x12, 0x80, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_107, 107, 0x0C, 0xEE, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_108, 108, 0x0C, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_109, 109, 0x12, 0xE6, 0xDD);
get_pokemon_cry_test!(get_pokemon_cry_110, 110, 0x12, 0xFF, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_111, 111, 0x04, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_112, 112, 0x11, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_113, 113, 0x14, 0x0A, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_114, 114, 0x12, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_115, 115, 0x03, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_116, 116, 0x19, 0x99, 0x10);
get_pokemon_cry_test!(get_pokemon_cry_117, 117, 0x19, 0x3C, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_118, 118, 0x16, 0x80, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_119, 119, 0x16, 0x10, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_120, 120, 0x1E, 0x02, 0x20);
get_pokemon_cry_test!(get_pokemon_cry_121, 121, 0x1E, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_122, 122, 0x20, 0x08, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_123, 123, 0x16, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_124, 124, 0x0D, 0xFF, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_125, 125, 0x06, 0x8F, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_126, 126, 0x04, 0xFF, 0x30);
get_pokemon_cry_test!(get_pokemon_cry_127, 127, 0x14, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_128, 128, 0x1D, 0x11, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_129, 129, 0x17, 0x80, 0x00);
get_pokemon_cry_test!(get_pokemon_cry_130, 130, 0x17, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_131, 131, 0x1B, 0x00, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_132, 132, 0x0E, 0xFF, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_133, 133, 0x1A, 0x88, 0x60);
get_pokemon_cry_test!(get_pokemon_cry_134, 134, 0x1A, 0xAA, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_135, 135, 0x1A, 0x3D, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_136, 136, 0x1A, 0x10, 0x20);
get_pokemon_cry_test!(get_pokemon_cry_137, 137, 0x25, 0xAA, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_138, 138, 0x1F, 0xF0, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_139, 139, 0x1F, 0xFF, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_140, 140, 0x16, 0xBB, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_141, 141, 0x18, 0xEE, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_142, 142, 0x23, 0x20, 0xF0);
get_pokemon_cry_test!(get_pokemon_cry_143, 143, 0x05, 0x55, 0x01);
get_pokemon_cry_test!(get_pokemon_cry_144, 144, 0x09, 0x80, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_145, 145, 0x18, 0xFF, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_146, 146, 0x09, 0xF8, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_147, 147, 0x0F, 0x60, 0x40);
get_pokemon_cry_test!(get_pokemon_cry_148, 148, 0x0F, 0x40, 0x80);
get_pokemon_cry_test!(get_pokemon_cry_149, 149, 0x0F, 0x3C, 0xC0);
get_pokemon_cry_test!(get_pokemon_cry_150, 150, 0x1E, 0x99, 0xFF);
get_pokemon_cry_test!(get_pokemon_cry_151, 151, 0x1E, 0xEE, 0xFF);
