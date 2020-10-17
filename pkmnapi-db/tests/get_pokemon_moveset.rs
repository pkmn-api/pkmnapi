mod common;

macro_rules! get_pokemon_moveset_test {
    ($test_name:ident, $pokedex_id:expr, $pokemon_moveset:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_pokemon_moveset(&$pokedex_id) {
                Ok(pokemon_moveset) => assert_eq!(
                    pokemon_moveset, $pokemon_moveset,
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

get_pokemon_moveset_test!(get_pokemon_moveset_1, 1, vec![0x21, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_2, 2, vec![0x21, 0x2D, 0x49]);
get_pokemon_moveset_test!(get_pokemon_moveset_3, 3, vec![0x21, 0x2D, 0x49, 0x16]);
get_pokemon_moveset_test!(get_pokemon_moveset_4, 4, vec![0x0A, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_5, 5, vec![0x0A, 0x2D, 0x34]);
get_pokemon_moveset_test!(get_pokemon_moveset_6, 6, vec![0x0A, 0x2D, 0x34, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_7, 7, vec![0x21, 0x27]);
get_pokemon_moveset_test!(get_pokemon_moveset_8, 8, vec![0x21, 0x27, 0x91]);
get_pokemon_moveset_test!(get_pokemon_moveset_9, 9, vec![0x21, 0x27, 0x91, 0x37]);
get_pokemon_moveset_test!(get_pokemon_moveset_10, 10, vec![0x21, 0x51]);
get_pokemon_moveset_test!(get_pokemon_moveset_11, 11, vec![0x6A]);
get_pokemon_moveset_test!(get_pokemon_moveset_12, 12, vec![0x5D]);
get_pokemon_moveset_test!(get_pokemon_moveset_13, 13, vec![0x28, 0x51]);
get_pokemon_moveset_test!(get_pokemon_moveset_14, 14, vec![0x6A]);
get_pokemon_moveset_test!(get_pokemon_moveset_15, 15, vec![0x1F]);
get_pokemon_moveset_test!(get_pokemon_moveset_16, 16, vec![0x10]);
get_pokemon_moveset_test!(get_pokemon_moveset_17, 17, vec![0x10, 0x1C]);
get_pokemon_moveset_test!(get_pokemon_moveset_18, 18, vec![0x10, 0x1C, 0x62]);
get_pokemon_moveset_test!(get_pokemon_moveset_19, 19, vec![0x21, 0x27]);
get_pokemon_moveset_test!(get_pokemon_moveset_20, 20, vec![0x21, 0x27, 0x62]);
get_pokemon_moveset_test!(get_pokemon_moveset_21, 21, vec![0x40, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_22, 22, vec![0x40, 0x2D, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_23, 23, vec![0x23, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_24, 24, vec![0x23, 0x2B, 0x28]);
get_pokemon_moveset_test!(get_pokemon_moveset_25, 25, vec![0x54, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_26, 26, vec![0x54, 0x2D, 0x56]);
get_pokemon_moveset_test!(get_pokemon_moveset_27, 27, vec![0x0A]);
get_pokemon_moveset_test!(get_pokemon_moveset_28, 28, vec![0x0A, 0x1C]);
get_pokemon_moveset_test!(get_pokemon_moveset_29, 29, vec![0x2D, 0x21]);
get_pokemon_moveset_test!(get_pokemon_moveset_30, 30, vec![0x2D, 0x21, 0x0A]);
get_pokemon_moveset_test!(get_pokemon_moveset_31, 31, vec![0x21, 0x0A, 0x27, 0x22]);
get_pokemon_moveset_test!(get_pokemon_moveset_32, 32, vec![0x2B, 0x21]);
get_pokemon_moveset_test!(get_pokemon_moveset_33, 33, vec![0x2B, 0x21, 0x1E]);
get_pokemon_moveset_test!(get_pokemon_moveset_34, 34, vec![0x21, 0x1E, 0x28, 0x25]);
get_pokemon_moveset_test!(get_pokemon_moveset_35, 35, vec![0x01, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_36, 36, vec![0x2F, 0x03, 0x6B, 0x76]);
get_pokemon_moveset_test!(get_pokemon_moveset_37, 37, vec![0x34, 0x27]);
get_pokemon_moveset_test!(get_pokemon_moveset_38, 38, vec![0x34, 0x27, 0x62, 0x2E]);
get_pokemon_moveset_test!(get_pokemon_moveset_39, 39, vec![0x2F]);
get_pokemon_moveset_test!(get_pokemon_moveset_40, 40, vec![0x2F, 0x32, 0x6F, 0x03]);
get_pokemon_moveset_test!(get_pokemon_moveset_41, 41, vec![0x8D]);
get_pokemon_moveset_test!(get_pokemon_moveset_42, 42, vec![0x8D, 0x67, 0x2C]);
get_pokemon_moveset_test!(get_pokemon_moveset_43, 43, vec![0x47]);
get_pokemon_moveset_test!(get_pokemon_moveset_44, 44, vec![0x47, 0x4D, 0x4E]);
get_pokemon_moveset_test!(get_pokemon_moveset_45, 45, vec![0x4E, 0x4F, 0x33, 0x50]);
get_pokemon_moveset_test!(get_pokemon_moveset_46, 46, vec![0x0A]);
get_pokemon_moveset_test!(get_pokemon_moveset_47, 47, vec![0x0A, 0x4E, 0x8D]);
get_pokemon_moveset_test!(get_pokemon_moveset_48, 48, vec![0x21, 0x32]);
get_pokemon_moveset_test!(get_pokemon_moveset_49, 49, vec![0x21, 0x32, 0x4D, 0x8D]);
get_pokemon_moveset_test!(get_pokemon_moveset_50, 50, vec![0x0A]);
get_pokemon_moveset_test!(get_pokemon_moveset_51, 51, vec![0x0A, 0x2D, 0x5B]);
get_pokemon_moveset_test!(get_pokemon_moveset_52, 52, vec![0x0A, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_53, 53, vec![0x0A, 0x2D, 0x2C, 0x67]);
get_pokemon_moveset_test!(get_pokemon_moveset_54, 54, vec![0x0A]);
get_pokemon_moveset_test!(get_pokemon_moveset_55, 55, vec![0x0A, 0x27, 0x32]);
get_pokemon_moveset_test!(get_pokemon_moveset_56, 56, vec![0x0A, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_57, 57, vec![0x0A, 0x2B, 0x02, 0x9A]);
get_pokemon_moveset_test!(get_pokemon_moveset_58, 58, vec![0x2C, 0x2E]);
get_pokemon_moveset_test!(get_pokemon_moveset_59, 59, vec![0x2E, 0x34, 0x2B, 0x24]);
get_pokemon_moveset_test!(get_pokemon_moveset_60, 60, vec![0x91]);
get_pokemon_moveset_test!(get_pokemon_moveset_61, 61, vec![0x91, 0x5F, 0x37]);
get_pokemon_moveset_test!(get_pokemon_moveset_62, 62, vec![0x5F, 0x37, 0x03, 0x22]);
get_pokemon_moveset_test!(get_pokemon_moveset_63, 63, vec![0x64]);
get_pokemon_moveset_test!(get_pokemon_moveset_64, 64, vec![0x64, 0x5D, 0x32]);
get_pokemon_moveset_test!(get_pokemon_moveset_65, 65, vec![0x64, 0x5D, 0x32]);
get_pokemon_moveset_test!(get_pokemon_moveset_66, 66, vec![0x02]);
get_pokemon_moveset_test!(get_pokemon_moveset_67, 67, vec![0x02, 0x43, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_68, 68, vec![0x02, 0x43, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_69, 69, vec![0x16, 0x4A]);
get_pokemon_moveset_test!(get_pokemon_moveset_70, 70, vec![0x16, 0x4A, 0x23]);
get_pokemon_moveset_test!(get_pokemon_moveset_71, 71, vec![0x4F, 0x4E, 0x33, 0x4B]);
get_pokemon_moveset_test!(get_pokemon_moveset_72, 72, vec![0x33]);
get_pokemon_moveset_test!(get_pokemon_moveset_73, 73, vec![0x33, 0x30, 0x23]);
get_pokemon_moveset_test!(get_pokemon_moveset_74, 74, vec![0x21]);
get_pokemon_moveset_test!(get_pokemon_moveset_75, 75, vec![0x21, 0x6F]);
get_pokemon_moveset_test!(get_pokemon_moveset_76, 76, vec![0x21, 0x6F]);
get_pokemon_moveset_test!(get_pokemon_moveset_77, 77, vec![0x34]);
get_pokemon_moveset_test!(get_pokemon_moveset_78, 78, vec![0x34, 0x27, 0x17, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_79, 79, vec![0x5D]);
get_pokemon_moveset_test!(get_pokemon_moveset_80, 80, vec![0x5D, 0x32, 0x1D]);
get_pokemon_moveset_test!(get_pokemon_moveset_81, 81, vec![0x21]);
get_pokemon_moveset_test!(get_pokemon_moveset_82, 82, vec![0x21, 0x31, 0x54]);
get_pokemon_moveset_test!(get_pokemon_moveset_83, 83, vec![0x40, 0x1C]);
get_pokemon_moveset_test!(get_pokemon_moveset_84, 84, vec![0x40]);
get_pokemon_moveset_test!(get_pokemon_moveset_85, 85, vec![0x40, 0x2D, 0x1F]);
get_pokemon_moveset_test!(get_pokemon_moveset_86, 86, vec![0x1D]);
get_pokemon_moveset_test!(get_pokemon_moveset_87, 87, vec![0x1D, 0x2D, 0x3E]);
get_pokemon_moveset_test!(get_pokemon_moveset_88, 88, vec![0x01, 0x32]);
get_pokemon_moveset_test!(get_pokemon_moveset_89, 89, vec![0x01, 0x32, 0x8B]);
get_pokemon_moveset_test!(get_pokemon_moveset_90, 90, vec![0x21, 0x6E]);
get_pokemon_moveset_test!(get_pokemon_moveset_91, 91, vec![0x6E, 0x30, 0x80, 0x3E]);
get_pokemon_moveset_test!(get_pokemon_moveset_92, 92, vec![0x7A, 0x6D, 0x65]);
get_pokemon_moveset_test!(get_pokemon_moveset_93, 93, vec![0x7A, 0x6D, 0x65]);
get_pokemon_moveset_test!(get_pokemon_moveset_94, 94, vec![0x7A, 0x6D, 0x65]);
get_pokemon_moveset_test!(get_pokemon_moveset_95, 95, vec![0x21, 0x67]);
get_pokemon_moveset_test!(get_pokemon_moveset_96, 96, vec![0x01, 0x5F]);
get_pokemon_moveset_test!(get_pokemon_moveset_97, 97, vec![0x01, 0x5F, 0x32, 0x5D]);
get_pokemon_moveset_test!(get_pokemon_moveset_98, 98, vec![0x91, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_99, 99, vec![0x91, 0x2B, 0x0B]);
get_pokemon_moveset_test!(get_pokemon_moveset_100, 100, vec![0x21, 0x67]);
get_pokemon_moveset_test!(get_pokemon_moveset_101, 101, vec![0x21, 0x67, 0x31]);
get_pokemon_moveset_test!(get_pokemon_moveset_102, 102, vec![0x8C, 0x5F]);
get_pokemon_moveset_test!(get_pokemon_moveset_103, 103, vec![0x8C, 0x5F]);
get_pokemon_moveset_test!(get_pokemon_moveset_104, 104, vec![0x7D, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_105, 105, vec![0x7D, 0x2D, 0x2B, 0x74]);
get_pokemon_moveset_test!(get_pokemon_moveset_106, 106, vec![0x18, 0x60]);
get_pokemon_moveset_test!(get_pokemon_moveset_107, 107, vec![0x04, 0x61]);
get_pokemon_moveset_test!(get_pokemon_moveset_108, 108, vec![0x23, 0x30]);
get_pokemon_moveset_test!(get_pokemon_moveset_109, 109, vec![0x21, 0x7B]);
get_pokemon_moveset_test!(get_pokemon_moveset_110, 110, vec![0x21, 0x7B, 0x7C]);
get_pokemon_moveset_test!(get_pokemon_moveset_111, 111, vec![0x1E]);
get_pokemon_moveset_test!(get_pokemon_moveset_112, 112, vec![0x1E, 0x17, 0x27, 0x1F]);
get_pokemon_moveset_test!(get_pokemon_moveset_113, 113, vec![0x01, 0x03]);
get_pokemon_moveset_test!(get_pokemon_moveset_114, 114, vec![0x84, 0x14]);
get_pokemon_moveset_test!(get_pokemon_moveset_115, 115, vec![0x04, 0x63]);
get_pokemon_moveset_test!(get_pokemon_moveset_116, 116, vec![0x91]);
get_pokemon_moveset_test!(get_pokemon_moveset_117, 117, vec![0x91, 0x6C]);
get_pokemon_moveset_test!(get_pokemon_moveset_118, 118, vec![0x40, 0x27]);
get_pokemon_moveset_test!(get_pokemon_moveset_119, 119, vec![0x40, 0x27, 0x30]);
get_pokemon_moveset_test!(get_pokemon_moveset_120, 120, vec![0x21]);
get_pokemon_moveset_test!(get_pokemon_moveset_121, 121, vec![0x21, 0x37, 0x6A]);
get_pokemon_moveset_test!(get_pokemon_moveset_122, 122, vec![0x5D, 0x70]);
get_pokemon_moveset_test!(get_pokemon_moveset_123, 123, vec![0x62]);
get_pokemon_moveset_test!(get_pokemon_moveset_124, 124, vec![0x01, 0x8E]);
get_pokemon_moveset_test!(get_pokemon_moveset_125, 125, vec![0x62, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_126, 126, vec![0x34]);
get_pokemon_moveset_test!(get_pokemon_moveset_127, 127, vec![0x0B]);
get_pokemon_moveset_test!(get_pokemon_moveset_128, 128, vec![0x21]);
get_pokemon_moveset_test!(get_pokemon_moveset_129, 129, vec![0x96]);
get_pokemon_moveset_test!(get_pokemon_moveset_130, 130, vec![0x2C, 0x52, 0x2B, 0x38]);
get_pokemon_moveset_test!(get_pokemon_moveset_131, 131, vec![0x37, 0x2D]);
get_pokemon_moveset_test!(get_pokemon_moveset_132, 132, vec![0x90]);
get_pokemon_moveset_test!(get_pokemon_moveset_133, 133, vec![0x21, 0x1C]);
get_pokemon_moveset_test!(get_pokemon_moveset_134, 134, vec![0x21, 0x1C, 0x62, 0x37]);
get_pokemon_moveset_test!(get_pokemon_moveset_135, 135, vec![0x21, 0x1C, 0x62, 0x54]);
get_pokemon_moveset_test!(get_pokemon_moveset_136, 136, vec![0x21, 0x1C, 0x62, 0x34]);
get_pokemon_moveset_test!(get_pokemon_moveset_137, 137, vec![0x21, 0x9F, 0xA0]);
get_pokemon_moveset_test!(get_pokemon_moveset_138, 138, vec![0x37, 0x6E]);
get_pokemon_moveset_test!(get_pokemon_moveset_139, 139, vec![0x37, 0x6E, 0x1E]);
get_pokemon_moveset_test!(get_pokemon_moveset_140, 140, vec![0x0A, 0x6A]);
get_pokemon_moveset_test!(get_pokemon_moveset_141, 141, vec![0x0A, 0x6A, 0x47]);
get_pokemon_moveset_test!(get_pokemon_moveset_142, 142, vec![0x11, 0x61]);
get_pokemon_moveset_test!(get_pokemon_moveset_143, 143, vec![0x1D, 0x85, 0x9C]);
get_pokemon_moveset_test!(get_pokemon_moveset_144, 144, vec![0x40, 0x3A]);
get_pokemon_moveset_test!(get_pokemon_moveset_145, 145, vec![0x54, 0x41]);
get_pokemon_moveset_test!(get_pokemon_moveset_146, 146, vec![0x40, 0x53]);
get_pokemon_moveset_test!(get_pokemon_moveset_147, 147, vec![0x23, 0x2B]);
get_pokemon_moveset_test!(get_pokemon_moveset_148, 148, vec![0x23, 0x2B, 0x56]);
get_pokemon_moveset_test!(get_pokemon_moveset_149, 149, vec![0x23, 0x2B, 0x56, 0x61]);
get_pokemon_moveset_test!(get_pokemon_moveset_150, 150, vec![0x5D, 0x32, 0x81, 0x5E]);
get_pokemon_moveset_test!(get_pokemon_moveset_151, 151, vec![0x01]);
