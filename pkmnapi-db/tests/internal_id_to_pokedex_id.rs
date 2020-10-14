mod common;

macro_rules! internal_id_to_pokedex_id_test {
    ($test_name:ident, $internal_id:expr, $pokedex_id:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.internal_id_to_pokedex_id(&$internal_id) {
                Ok(pokedex_id) => assert_eq!(
                    pokedex_id, $pokedex_id,
                    "Searched for internal ID: {}",
                    $internal_id
                ),
                Err(_) => panic!(format!("Could not find internal ID: {}", $internal_id)),
            };
        }
    };
}

internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_152, 152, 1);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_8, 8, 2);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_153, 153, 3);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_175, 175, 4);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_177, 177, 5);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_179, 179, 6);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_176, 176, 7);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_178, 178, 8);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_27, 27, 9);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_122, 122, 10);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_123, 123, 11);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_124, 124, 12);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_111, 111, 13);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_112, 112, 14);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_113, 113, 15);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_35, 35, 16);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_149, 149, 17);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_150, 150, 18);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_164, 164, 19);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_165, 165, 20);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_4, 4, 21);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_34, 34, 22);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_107, 107, 23);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_44, 44, 24);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_83, 83, 25);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_84, 84, 26);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_95, 95, 27);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_96, 96, 28);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_14, 14, 29);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_167, 167, 30);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_15, 15, 31);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_2, 2, 32);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_166, 166, 33);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_6, 6, 34);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_3, 3, 35);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_141, 141, 36);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_81, 81, 37);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_82, 82, 38);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_99, 99, 39);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_100, 100, 40);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_106, 106, 41);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_129, 129, 42);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_184, 184, 43);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_185, 185, 44);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_186, 186, 45);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_108, 108, 46);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_45, 45, 47);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_64, 64, 48);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_118, 118, 49);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_58, 58, 50);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_117, 117, 51);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_76, 76, 52);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_143, 143, 53);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_46, 46, 54);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_127, 127, 55);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_56, 56, 56);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_116, 116, 57);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_32, 32, 58);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_19, 19, 59);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_70, 70, 60);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_109, 109, 61);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_110, 110, 62);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_147, 147, 63);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_37, 37, 64);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_148, 148, 65);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_105, 105, 66);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_40, 40, 67);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_125, 125, 68);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_187, 187, 69);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_188, 188, 70);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_189, 189, 71);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_23, 23, 72);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_154, 154, 73);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_168, 168, 74);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_38, 38, 75);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_48, 48, 76);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_162, 162, 77);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_163, 163, 78);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_36, 36, 79);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_7, 7, 80);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_172, 172, 81);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_53, 53, 82);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_63, 63, 83);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_69, 69, 84);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_115, 115, 85);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_57, 57, 86);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_119, 119, 87);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_12, 12, 88);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_135, 135, 89);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_22, 22, 90);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_138, 138, 91);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_24, 24, 92);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_146, 146, 93);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_13, 13, 94);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_33, 33, 95);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_47, 47, 96);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_128, 128, 97);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_77, 77, 98);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_137, 137, 99);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_5, 5, 100);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_140, 140, 101);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_11, 11, 102);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_9, 9, 103);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_16, 16, 104);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_144, 144, 105);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_42, 42, 106);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_43, 43, 107);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_10, 10, 108);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_54, 54, 109);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_142, 142, 110);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_17, 17, 111);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_0, 0, 112);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_39, 39, 113);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_29, 29, 114);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_1, 1, 115);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_91, 91, 116);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_92, 92, 117);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_156, 156, 118);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_157, 157, 119);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_26, 26, 120);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_151, 151, 121);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_41, 41, 122);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_25, 25, 123);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_71, 71, 124);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_52, 52, 125);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_50, 50, 126);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_28, 28, 127);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_59, 59, 128);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_132, 132, 129);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_21, 21, 130);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_18, 18, 131);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_75, 75, 132);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_101, 101, 133);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_104, 104, 134);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_103, 103, 135);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_102, 102, 136);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_169, 169, 137);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_97, 97, 138);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_98, 98, 139);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_89, 89, 140);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_90, 90, 141);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_170, 170, 142);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_131, 131, 143);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_73, 73, 144);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_74, 74, 145);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_72, 72, 146);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_87, 87, 147);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_88, 88, 148);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_65, 65, 149);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_130, 130, 150);
internal_id_to_pokedex_id_test!(internal_id_to_pokedex_id_20, 20, 151);
