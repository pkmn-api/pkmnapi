mod common;

macro_rules! get_map_pic_test {
    ($test_name:ident, $map_id:expr, $map_width:expr, $map_height:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_map_pic(&$map_id) {
                Ok(map) => {
                    assert_eq!(map.width, $map_width);
                    assert_eq!(map.height, $map_height);
                }
                Err(e) => panic!(e.to_string()),
            };
        }
    };
    ($test_name:ident, $map_id:expr, $map_width:expr, $map_height:expr, $panic_msg:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        #[should_panic(expected = $panic_msg)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_map_pic(&$map_id) {
                Ok(map) => {
                    assert_eq!(map.width, $map_width);
                    assert_eq!(map.height, $map_height);
                }
                Err(e) => panic!(e.to_string()),
            };
        }
    };
}

#[rustfmt::skip::macros(get_map_pic_test)]

get_map_pic_test!(get_map_pic_0, 0, 40, 36);
get_map_pic_test!(get_map_pic_1, 1, 80, 72);
get_map_pic_test!(get_map_pic_2, 2, 80, 72);
get_map_pic_test!(get_map_pic_3, 3, 80, 72);
get_map_pic_test!(get_map_pic_4, 4, 40, 36);
get_map_pic_test!(get_map_pic_5, 5, 80, 72);
get_map_pic_test!(get_map_pic_6, 6, 100, 72);
get_map_pic_test!(get_map_pic_7, 7, 80, 72);
get_map_pic_test!(get_map_pic_8, 8, 40, 36);
get_map_pic_test!(get_map_pic_9, 9, 40, 36);
get_map_pic_test!(get_map_pic_10, 10, 80, 72);
get_map_pic_test!(get_map_pic_11, 11, 0, 0, "Invalid map ID: 11");
get_map_pic_test!(get_map_pic_12, 12, 40, 72);
get_map_pic_test!(get_map_pic_13, 13, 40, 144);
get_map_pic_test!(get_map_pic_14, 14, 140, 36);
get_map_pic_test!(get_map_pic_15, 15, 180, 36);
get_map_pic_test!(get_map_pic_16, 16, 40, 72);
get_map_pic_test!(get_map_pic_17, 17, 40, 72);
get_map_pic_test!(get_map_pic_18, 18, 40, 36);
get_map_pic_test!(get_map_pic_19, 19, 120, 36);
get_map_pic_test!(get_map_pic_20, 20, 120, 36);
get_map_pic_test!(get_map_pic_21, 21, 40, 144);
get_map_pic_test!(get_map_pic_22, 22, 120, 36);
get_map_pic_test!(get_map_pic_23, 23, 40, 216);
get_map_pic_test!(get_map_pic_24, 24, 120, 36);
get_map_pic_test!(get_map_pic_25, 25, 40, 108);
get_map_pic_test!(get_map_pic_26, 26, 120, 36);
get_map_pic_test!(get_map_pic_27, 27, 80, 36);
get_map_pic_test!(get_map_pic_28, 28, 40, 288);
get_map_pic_test!(get_map_pic_29, 29, 100, 36);
get_map_pic_test!(get_map_pic_30, 30, 40, 108);
get_map_pic_test!(get_map_pic_31, 31, 200, 36);
get_map_pic_test!(get_map_pic_32, 32, 40, 180);
get_map_pic_test!(get_map_pic_33, 33, 80, 36);
get_map_pic_test!(get_map_pic_34, 34, 40, 288);
get_map_pic_test!(get_map_pic_35, 35, 40, 72);
get_map_pic_test!(get_map_pic_36, 36, 120, 36);
get_map_pic_test!(get_map_pic_37, 37, 16, 16);
get_map_pic_test!(get_map_pic_38, 38, 16, 16);
get_map_pic_test!(get_map_pic_39, 39, 16, 16);
get_map_pic_test!(get_map_pic_40, 40, 20, 24);
get_map_pic_test!(get_map_pic_41, 41, 28, 16);
get_map_pic_test!(get_map_pic_42, 42, 16, 16);
get_map_pic_test!(get_map_pic_43, 43, 16, 16);
get_map_pic_test!(get_map_pic_44, 44, 16, 16);
get_map_pic_test!(get_map_pic_45, 45, 40, 36);
get_map_pic_test!(get_map_pic_46, 46, 16, 16);
get_map_pic_test!(get_map_pic_47, 47, 20, 16);
get_map_pic_test!(get_map_pic_48, 48, 16, 16);
get_map_pic_test!(get_map_pic_49, 49, 20, 16);
get_map_pic_test!(get_map_pic_50, 50, 20, 16);
get_map_pic_test!(get_map_pic_51, 51, 68, 96);
get_map_pic_test!(get_map_pic_52, 52, 40, 16);
get_map_pic_test!(get_map_pic_53, 53, 28, 16);
get_map_pic_test!(get_map_pic_54, 54, 20, 28);
get_map_pic_test!(get_map_pic_55, 55, 16, 16);
get_map_pic_test!(get_map_pic_56, 56, 16, 16);
get_map_pic_test!(get_map_pic_57, 57, 16, 16);
get_map_pic_test!(get_map_pic_58, 58, 28, 16);
get_map_pic_test!(get_map_pic_59, 59, 80, 72);
get_map_pic_test!(get_map_pic_60, 60, 56, 56);
get_map_pic_test!(get_map_pic_61, 61, 80, 72);
get_map_pic_test!(get_map_pic_62, 62, 16, 16);
get_map_pic_test!(get_map_pic_63, 63, 16, 16);
get_map_pic_test!(get_map_pic_64, 64, 28, 16);
get_map_pic_test!(get_map_pic_65, 65, 20, 28);
get_map_pic_test!(get_map_pic_66, 66, 16, 16);
get_map_pic_test!(get_map_pic_67, 67, 16, 16);
get_map_pic_test!(get_map_pic_68, 68, 28, 16);
get_map_pic_test!(get_map_pic_69, 69, 16, 16);
get_map_pic_test!(get_map_pic_70, 70, 16, 12);
get_map_pic_test!(get_map_pic_71, 71, 16, 16);
get_map_pic_test!(get_map_pic_72, 72, 16, 16);
get_map_pic_test!(get_map_pic_73, 73, 16, 12);
get_map_pic_test!(get_map_pic_74, 74, 16, 16);
get_map_pic_test!(get_map_pic_75, 75, 16, 16);
get_map_pic_test!(get_map_pic_76, 76, 12, 16);
get_map_pic_test!(get_map_pic_77, 77, 16, 16);
get_map_pic_test!(get_map_pic_78, 78, 16, 16);
get_map_pic_test!(get_map_pic_79, 79, 12, 16);
get_map_pic_test!(get_map_pic_80, 80, 16, 16);
get_map_pic_test!(get_map_pic_81, 81, 28, 16);
get_map_pic_test!(get_map_pic_82, 82, 80, 72);
get_map_pic_test!(get_map_pic_83, 83, 80, 72);
get_map_pic_test!(get_map_pic_84, 84, 16, 20);
get_map_pic_test!(get_map_pic_85, 85, 16, 16);
get_map_pic_test!(get_map_pic_86, 86, 16, 16);
get_map_pic_test!(get_map_pic_87, 87, 20, 16);
get_map_pic_test!(get_map_pic_88, 88, 16, 16);
get_map_pic_test!(get_map_pic_89, 89, 28, 16);
get_map_pic_test!(get_map_pic_90, 90, 16, 16);
get_map_pic_test!(get_map_pic_91, 91, 16, 16);
get_map_pic_test!(get_map_pic_92, 92, 20, 36);
get_map_pic_test!(get_map_pic_93, 93, 16, 16);
get_map_pic_test!(get_map_pic_94, 94, 56, 24);
get_map_pic_test!(get_map_pic_95, 95, 80, 36);
get_map_pic_test!(get_map_pic_96, 96, 80, 36);
get_map_pic_test!(get_map_pic_97, 97, 40, 12);
get_map_pic_test!(get_map_pic_98, 98, 60, 16);
get_map_pic_test!(get_map_pic_99, 99, 40, 28);
get_map_pic_test!(get_map_pic_100, 100, 28, 32);
get_map_pic_test!(get_map_pic_101, 101, 12, 16);
get_map_pic_test!(get_map_pic_102, 102, 48, 32);
get_map_pic_test!(get_map_pic_103, 103, 48, 32);
get_map_pic_test!(get_map_pic_104, 104, 48, 32);
get_map_pic_test!(get_map_pic_105, 105, 0, 0, "Invalid map ID: 105");
get_map_pic_test!(get_map_pic_106, 106, 0, 0, "Invalid map ID: 106");
get_map_pic_test!(get_map_pic_107, 107, 0, 0, "Invalid map ID: 107");
get_map_pic_test!(get_map_pic_108, 108, 40, 36);
get_map_pic_test!(get_map_pic_109, 109, 0, 0, "Invalid map ID: 109");
get_map_pic_test!(get_map_pic_110, 110, 0, 0, "Invalid map ID: 110");
get_map_pic_test!(get_map_pic_111, 111, 0, 0, "Invalid map ID: 111");
get_map_pic_test!(get_map_pic_112, 112, 0, 0, "Invalid map ID: 112");
get_map_pic_test!(get_map_pic_113, 113, 52, 52);
get_map_pic_test!(get_map_pic_114, 114, 0, 0, "Invalid map ID: 114");
get_map_pic_test!(get_map_pic_115, 115, 0, 0, "Invalid map ID: 115");
get_map_pic_test!(get_map_pic_116, 116, 0, 0, "Invalid map ID: 116");
get_map_pic_test!(get_map_pic_117, 117, 0, 0, "Invalid map ID: 117");
get_map_pic_test!(get_map_pic_118, 118, 20, 16);
get_map_pic_test!(get_map_pic_119, 119, 16, 96);
get_map_pic_test!(get_map_pic_120, 120, 16, 16);
get_map_pic_test!(get_map_pic_121, 121, 100, 16);
get_map_pic_test!(get_map_pic_122, 122, 40, 16);
get_map_pic_test!(get_map_pic_123, 123, 40, 16);
get_map_pic_test!(get_map_pic_124, 124, 40, 16);
get_map_pic_test!(get_map_pic_125, 125, 40, 16);
get_map_pic_test!(get_map_pic_126, 126, 40, 16);
get_map_pic_test!(get_map_pic_127, 127, 8, 8);
get_map_pic_test!(get_map_pic_128, 128, 16, 24);
get_map_pic_test!(get_map_pic_129, 129, 16, 24);
get_map_pic_test!(get_map_pic_130, 130, 16, 24);
get_map_pic_test!(get_map_pic_131, 131, 16, 24);
get_map_pic_test!(get_map_pic_132, 132, 16, 16);
get_map_pic_test!(get_map_pic_133, 133, 28, 16);
get_map_pic_test!(get_map_pic_134, 134, 20, 36);
get_map_pic_test!(get_map_pic_135, 135, 40, 36);
get_map_pic_test!(get_map_pic_136, 136, 40, 16);
get_map_pic_test!(get_map_pic_137, 137, 20, 16);
get_map_pic_test!(get_map_pic_138, 138, 20, 16);
get_map_pic_test!(get_map_pic_139, 139, 16, 16);
get_map_pic_test!(get_map_pic_140, 140, 28, 16);
get_map_pic_test!(get_map_pic_141, 141, 28, 16);
get_map_pic_test!(get_map_pic_142, 142, 40, 36);
get_map_pic_test!(get_map_pic_143, 143, 40, 36);
get_map_pic_test!(get_map_pic_144, 144, 40, 36);
get_map_pic_test!(get_map_pic_145, 145, 40, 36);
get_map_pic_test!(get_map_pic_146, 146, 40, 36);
get_map_pic_test!(get_map_pic_147, 147, 40, 36);
get_map_pic_test!(get_map_pic_148, 148, 40, 36);
get_map_pic_test!(get_map_pic_149, 149, 16, 16);
get_map_pic_test!(get_map_pic_150, 150, 16, 16);
get_map_pic_test!(get_map_pic_151, 151, 16, 16);
get_map_pic_test!(get_map_pic_152, 152, 16, 16);
get_map_pic_test!(get_map_pic_153, 153, 16, 16);
get_map_pic_test!(get_map_pic_154, 154, 28, 16);
get_map_pic_test!(get_map_pic_155, 155, 20, 16);
get_map_pic_test!(get_map_pic_156, 156, 16, 12);
get_map_pic_test!(get_map_pic_157, 157, 20, 36);
get_map_pic_test!(get_map_pic_158, 158, 28, 16);
get_map_pic_test!(get_map_pic_159, 159, 60, 36);
get_map_pic_test!(get_map_pic_160, 160, 60, 36);
get_map_pic_test!(get_map_pic_161, 161, 60, 36);
get_map_pic_test!(get_map_pic_162, 162, 60, 36);
get_map_pic_test!(get_map_pic_163, 163, 16, 16);
get_map_pic_test!(get_map_pic_164, 164, 16, 16);
get_map_pic_test!(get_map_pic_165, 165, 60, 56);
get_map_pic_test!(get_map_pic_166, 166, 40, 36);
get_map_pic_test!(get_map_pic_167, 167, 36, 16);
get_map_pic_test!(get_map_pic_168, 168, 16, 16);
get_map_pic_test!(get_map_pic_169, 169, 16, 16);
get_map_pic_test!(get_map_pic_170, 170, 16, 16);
get_map_pic_test!(get_map_pic_171, 171, 28, 16);
get_map_pic_test!(get_map_pic_172, 172, 16, 16);
get_map_pic_test!(get_map_pic_173, 173, 16, 16);
get_map_pic_test!(get_map_pic_174, 174, 32, 24);
get_map_pic_test!(get_map_pic_175, 175, 16, 16);
get_map_pic_test!(get_map_pic_176, 176, 16, 16);
get_map_pic_test!(get_map_pic_177, 177, 20, 24);
get_map_pic_test!(get_map_pic_178, 178, 40, 36);
get_map_pic_test!(get_map_pic_179, 179, 16, 16);
get_map_pic_test!(get_map_pic_180, 180, 16, 16);
get_map_pic_test!(get_map_pic_181, 181, 60, 36);
get_map_pic_test!(get_map_pic_182, 182, 28, 16);
get_map_pic_test!(get_map_pic_183, 183, 16, 16);
get_map_pic_test!(get_map_pic_184, 184, 16, 20);
get_map_pic_test!(get_map_pic_185, 185, 16, 16);
get_map_pic_test!(get_map_pic_186, 186, 16, 28);
get_map_pic_test!(get_map_pic_187, 187, 16, 16);
get_map_pic_test!(get_map_pic_188, 188, 16, 16);
get_map_pic_test!(get_map_pic_189, 189, 16, 16);
get_map_pic_test!(get_map_pic_190, 190, 16, 20);
get_map_pic_test!(get_map_pic_191, 191, 16, 16);
get_map_pic_test!(get_map_pic_192, 192, 60, 36);
get_map_pic_test!(get_map_pic_193, 193, 20, 16);
get_map_pic_test!(get_map_pic_194, 194, 60, 36);
get_map_pic_test!(get_map_pic_195, 195, 16, 16);
get_map_pic_test!(get_map_pic_196, 196, 16, 16);
get_map_pic_test!(get_map_pic_197, 197, 80, 72);
get_map_pic_test!(get_map_pic_198, 198, 60, 36);
get_map_pic_test!(get_map_pic_199, 199, 60, 56);
get_map_pic_test!(get_map_pic_200, 200, 60, 56);
get_map_pic_test!(get_map_pic_201, 201, 60, 56);
get_map_pic_test!(get_map_pic_202, 202, 60, 48);
get_map_pic_test!(get_map_pic_203, 203, 12, 16);
get_map_pic_test!(get_map_pic_204, 204, 0, 0, "Invalid map ID: 204");
get_map_pic_test!(get_map_pic_205, 205, 0, 0, "Invalid map ID: 205");
get_map_pic_test!(get_map_pic_206, 206, 0, 0, "Invalid map ID: 206");
get_map_pic_test!(get_map_pic_207, 207, 60, 36);
get_map_pic_test!(get_map_pic_208, 208, 60, 36);
get_map_pic_test!(get_map_pic_209, 209, 60, 36);
get_map_pic_test!(get_map_pic_210, 210, 60, 36);
get_map_pic_test!(get_map_pic_211, 211, 52, 36);
get_map_pic_test!(get_map_pic_212, 212, 52, 36);
get_map_pic_test!(get_map_pic_213, 213, 52, 36);
get_map_pic_test!(get_map_pic_214, 214, 60, 56);
get_map_pic_test!(get_map_pic_215, 215, 60, 36);
get_map_pic_test!(get_map_pic_216, 216, 60, 56);
get_map_pic_test!(get_map_pic_217, 217, 60, 52);
get_map_pic_test!(get_map_pic_218, 218, 80, 72);
get_map_pic_test!(get_map_pic_219, 219, 60, 52);
get_map_pic_test!(get_map_pic_220, 220, 60, 52);
get_map_pic_test!(get_map_pic_221, 221, 16, 16);
get_map_pic_test!(get_map_pic_222, 222, 16, 16);
get_map_pic_test!(get_map_pic_223, 223, 16, 16);
get_map_pic_test!(get_map_pic_224, 224, 16, 16);
get_map_pic_test!(get_map_pic_225, 225, 16, 16);
get_map_pic_test!(get_map_pic_226, 226, 60, 36);
get_map_pic_test!(get_map_pic_227, 227, 60, 36);
get_map_pic_test!(get_map_pic_228, 228, 60, 36);
get_map_pic_test!(get_map_pic_229, 229, 16, 16);
get_map_pic_test!(get_map_pic_230, 230, 16, 16);
get_map_pic_test!(get_map_pic_231, 231, 0, 0, "Invalid map ID: 231");
get_map_pic_test!(get_map_pic_232, 232, 80, 72);
get_map_pic_test!(get_map_pic_233, 233, 52, 36);
get_map_pic_test!(get_map_pic_234, 234, 32, 36);
get_map_pic_test!(get_map_pic_235, 235, 36, 36);
get_map_pic_test!(get_map_pic_236, 236, 8, 8);
get_map_pic_test!(get_map_pic_237, 237, 0, 0, "Invalid map ID: 237");
get_map_pic_test!(get_map_pic_238, 238, 0, 0, "Invalid map ID: 238");
get_map_pic_test!(get_map_pic_239, 239, 20, 16);
get_map_pic_test!(get_map_pic_240, 240, 20, 16);
get_map_pic_test!(get_map_pic_241, 241, 0, 0, "Invalid map ID: 241");
get_map_pic_test!(get_map_pic_242, 242, 0, 0, "Invalid map ID: 242");
get_map_pic_test!(get_map_pic_243, 243, 0, 0, "Invalid map ID: 243");
get_map_pic_test!(get_map_pic_244, 244, 0, 0, "Invalid map ID: 244");
get_map_pic_test!(get_map_pic_245, 245, 20, 24);
get_map_pic_test!(get_map_pic_246, 246, 20, 24);
get_map_pic_test!(get_map_pic_247, 247, 20, 24);
