mod common;

macro_rules! pokedex_id_to_internal_id_test {
    ($test_name: ident, $pokedex_id: expr, $internal_id: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.pokedex_id_to_internal_id($pokedex_id) {
                Ok(internal_id) => assert_eq!(
                    internal_id, $internal_id,
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(pokedex_id_to_internal_id_test)]

pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_1, 1, 152);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_2, 2, 8);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_3, 3, 153);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_4, 4, 175);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_5, 5, 177);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_6, 6, 179);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_7, 7, 176);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_8, 8, 178);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_9, 9, 27);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_10, 10, 122);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_11, 11, 123);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_12, 12, 124);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_13, 13, 111);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_14, 14, 112);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_15, 15, 113);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_16, 16, 35);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_17, 17, 149);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_18, 18, 150);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_19, 19, 164);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_20, 20, 165);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_21, 21, 4);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_22, 22, 34);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_23, 23, 107);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_24, 24, 44);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_25, 25, 83);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_26, 26, 84);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_27, 27, 95);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_28, 28, 96);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_29, 29, 14);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_30, 30, 167);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_31, 31, 15);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_32, 32, 2);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_33, 33, 166);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_34, 34, 6);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_35, 35, 3);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_36, 36, 141);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_37, 37, 81);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_38, 38, 82);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_39, 39, 99);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_40, 40, 100);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_41, 41, 106);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_42, 42, 129);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_43, 43, 184);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_44, 44, 185);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_45, 45, 186);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_46, 46, 108);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_47, 47, 45);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_48, 48, 64);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_49, 49, 118);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_50, 50, 58);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_51, 51, 117);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_52, 52, 76);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_53, 53, 143);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_54, 54, 46);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_55, 55, 127);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_56, 56, 56);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_57, 57, 116);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_58, 58, 32);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_59, 59, 19);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_60, 60, 70);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_61, 61, 109);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_62, 62, 110);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_63, 63, 147);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_64, 64, 37);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_65, 65, 148);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_66, 66, 105);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_67, 67, 40);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_68, 68, 125);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_69, 69, 187);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_70, 70, 188);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_71, 71, 189);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_72, 72, 23);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_73, 73, 154);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_74, 74, 168);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_75, 75, 38);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_76, 76, 48);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_77, 77, 162);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_78, 78, 163);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_79, 79, 36);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_80, 80, 7);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_81, 81, 172);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_82, 82, 53);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_83, 83, 63);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_84, 84, 69);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_85, 85, 115);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_86, 86, 57);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_87, 87, 119);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_88, 88, 12);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_89, 89, 135);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_90, 90, 22);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_91, 91, 138);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_92, 92, 24);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_93, 93, 146);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_94, 94, 13);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_95, 95, 33);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_96, 96, 47);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_97, 97, 128);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_98, 98, 77);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_99, 99, 137);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_100, 100, 5);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_101, 101, 140);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_102, 102, 11);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_103, 103, 9);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_104, 104, 16);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_105, 105, 144);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_106, 106, 42);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_107, 107, 43);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_108, 108, 10);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_109, 109, 54);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_110, 110, 142);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_111, 111, 17);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_112, 112, 0);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_113, 113, 39);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_114, 114, 29);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_115, 115, 1);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_116, 116, 91);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_117, 117, 92);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_118, 118, 156);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_119, 119, 157);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_120, 120, 26);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_121, 121, 151);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_122, 122, 41);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_123, 123, 25);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_124, 124, 71);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_125, 125, 52);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_126, 126, 50);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_127, 127, 28);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_128, 128, 59);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_129, 129, 132);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_130, 130, 21);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_131, 131, 18);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_132, 132, 75);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_133, 133, 101);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_134, 134, 104);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_135, 135, 103);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_136, 136, 102);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_137, 137, 169);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_138, 138, 97);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_139, 139, 98);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_140, 140, 89);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_141, 141, 90);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_142, 142, 170);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_143, 143, 131);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_144, 144, 73);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_145, 145, 74);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_146, 146, 72);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_147, 147, 87);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_148, 148, 88);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_149, 149, 65);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_150, 150, 130);
pokedex_id_to_internal_id_test!(pokedex_id_to_internal_id_151, 151, 20);
