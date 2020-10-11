use pkmnapi_db::types::*;

mod common;

macro_rules! get_move_stats_test {
    (
        $test_name:ident,
        $move_id:expr,
        $effect:expr,
        $power:expr,
        $type_id:expr,
        $accuracy:expr,
        $pp:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_move_stats(&$move_id) {
                Ok(move_stats) => assert_eq!(
                    move_stats,
                    MoveStats {
                        move_id: $move_id,
                        effect: $effect,
                        power: $power,
                        type_id: $type_id,
                        accuracy: $accuracy,
                        pp: $pp
                    },
                    "Searched for move ID: {}",
                    $move_id
                ),
                Err(_) => panic!(format!("Could not find move ID: {}", $move_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_move_stats_test)]

get_move_stats_test!(get_move_stats_1, 1, 0, 40, 0, 255.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_2, 2, 0, 50, 0, 255.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_3, 3, 29, 15, 0, 216.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_4, 4, 29, 18, 0, 216.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_5, 5, 0, 80, 0, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_6, 6, 16, 40, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_7, 7, 4, 75, 20, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_8, 8, 5, 75, 25, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_9, 9, 6, 75, 23, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_10, 10, 0, 40, 0, 255.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_11, 11, 0, 55, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_12, 12, 38, 1, 0, 76.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_13, 13, 39, 80, 0, 191.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_14, 14, 50, 0, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_15, 15, 0, 50, 0, 242.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_16, 16, 0, 40, 0, 255.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_17, 17, 0, 35, 2, 255.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_18, 18, 28, 0, 0, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_19, 19, 43, 70, 2, 242.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_20, 20, 42, 15, 0, 191.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_21, 21, 0, 80, 0, 191.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_22, 22, 0, 35, 22, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_23, 23, 37, 65, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_24, 24, 44, 30, 1, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_25, 25, 0, 120, 0, 191.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_26, 26, 45, 70, 1, 242.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_27, 27, 37, 60, 1, 216.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_28, 28, 22, 0, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_29, 29, 37, 70, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_30, 30, 0, 65, 0, 255.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_31, 31, 29, 15, 0, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_32, 32, 38, 1, 0, 76.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_33, 33, 0, 35, 0, 242.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_34, 34, 36, 85, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_35, 35, 42, 15, 0, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_36, 36, 48, 90, 0, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_37, 37, 27, 90, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_38, 38, 48, 100, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_39, 39, 19, 0, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_40, 40, 2, 15, 3, 255.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_41, 41, 77, 25, 7, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_42, 42, 29, 14, 7, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_43, 43, 19, 0, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_44, 44, 31, 60, 0, 255.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_45, 45, 18, 0, 0, 255.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_46, 46, 28, 0, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_47, 47, 32, 0, 0, 140.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_48, 48, 49, 0, 0, 140.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_49, 49, 41, 1, 0, 229.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_50, 50, 86, 0, 0, 140.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_51, 51, 69, 40, 3, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_52, 52, 4, 40, 20, 255.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_53, 53, 4, 95, 20, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_54, 54, 46, 0, 25, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_55, 55, 0, 40, 21, 255.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_56, 56, 0, 120, 21, 204.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_57, 57, 0, 95, 21, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_58, 58, 5, 95, 25, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_59, 59, 5, 120, 25, 229.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_60, 60, 76, 65, 24, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_61, 61, 70, 65, 21, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_62, 62, 68, 65, 25, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_63, 63, 80, 150, 0, 229.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_64, 64, 0, 35, 2, 255.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_65, 65, 0, 80, 2, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_66, 66, 48, 80, 1, 204.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_67, 67, 37, 50, 1, 229.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_68, 68, 0, 1, 1, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_69, 69, 41, 1, 1, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_70, 70, 0, 80, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_71, 71, 3, 20, 22, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_72, 72, 3, 40, 22, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_73, 73, 84, 0, 22, 229.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_74, 74, 13, 0, 0, 255.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_75, 75, 0, 55, 22, 242.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_76, 76, 39, 120, 22, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_77, 77, 66, 0, 3, 191.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_78, 78, 67, 0, 22, 191.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_79, 79, 32, 0, 22, 191.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_80, 80, 27, 70, 22, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_81, 81, 20, 0, 7, 242.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_82, 82, 41, 1, 26, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_83, 83, 42, 15, 20, 178.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_84, 84, 6, 40, 23, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_85, 85, 6, 95, 23, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_86, 86, 67, 0, 23, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_87, 87, 6, 120, 23, 178.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_88, 88, 0, 50, 5, 165.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_89, 89, 0, 100, 4, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_90, 90, 38, 1, 4, 76.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_91, 91, 39, 100, 4, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_92, 92, 66, 0, 3, 216.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_93, 93, 76, 50, 24, 255.0 / 255.0, 25);
get_move_stats_test!(get_move_stats_94, 94, 71, 90, 24, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_95, 95, 32, 0, 24, 153.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_96, 96, 10, 0, 24, 255.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_97, 97, 52, 0, 24, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_98, 98, 0, 40, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_99, 99, 81, 20, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_100, 100, 28, 0, 24, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_101, 101, 41, 0, 8, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_102, 102, 82, 0, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_103, 103, 59, 0, 0, 216.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_104, 104, 15, 0, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_105, 105, 56, 0, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_106, 106, 11, 0, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_107, 107, 15, 0, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_108, 108, 22, 0, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_109, 109, 49, 0, 8, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_110, 110, 11, 0, 21, 255.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_111, 111, 11, 0, 0, 255.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_112, 112, 51, 0, 24, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_113, 113, 64, 0, 24, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_114, 114, 25, 0, 25, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_115, 115, 65, 0, 24, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_116, 116, 47, 0, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_117, 117, 26, 0, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_118, 118, 83, 0, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_119, 119, 9, 0, 2, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_120, 120, 7, 130, 0, 255.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_121, 121, 0, 100, 0, 191.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_122, 122, 36, 20, 8, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_123, 123, 33, 20, 3, 178.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_124, 124, 33, 65, 3, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_125, 125, 31, 65, 4, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_126, 126, 34, 120, 20, 216.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_127, 127, 0, 80, 21, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_128, 128, 42, 35, 21, 191.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_129, 129, 17, 60, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_130, 130, 39, 100, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_131, 131, 29, 20, 0, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_132, 132, 70, 10, 0, 255.0 / 255.0, 35);
get_move_stats_test!(get_move_stats_133, 133, 53, 0, 24, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_134, 134, 22, 0, 24, 204.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_135, 135, 56, 0, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_136, 136, 45, 85, 1, 229.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_137, 137, 67, 0, 0, 191.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_138, 138, 8, 100, 24, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_139, 139, 66, 0, 3, 140.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_140, 140, 29, 15, 0, 216.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_141, 141, 3, 20, 7, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_142, 142, 32, 0, 0, 191.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_143, 143, 39, 140, 2, 229.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_144, 144, 57, 0, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_145, 145, 70, 20, 21, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_146, 146, 0, 70, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_147, 147, 32, 0, 22, 255.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_148, 148, 22, 0, 0, 178.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_149, 149, 41, 1, 24, 204.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_150, 150, 85, 0, 0, 255.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_151, 151, 51, 0, 3, 255.0 / 255.0, 40);
get_move_stats_test!(get_move_stats_152, 152, 0, 90, 21, 216.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_153, 153, 7, 170, 0, 255.0 / 255.0, 5);
get_move_stats_test!(get_move_stats_154, 154, 29, 18, 0, 204.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_155, 155, 44, 50, 4, 229.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_156, 156, 56, 0, 24, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_157, 157, 0, 75, 5, 229.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_158, 158, 31, 80, 0, 229.0 / 255.0, 15);
get_move_stats_test!(get_move_stats_159, 159, 10, 0, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_160, 160, 24, 0, 0, 255.0 / 255.0, 30);
get_move_stats_test!(get_move_stats_161, 161, 0, 80, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_162, 162, 40, 1, 0, 229.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_163, 163, 0, 70, 0, 255.0 / 255.0, 20);
get_move_stats_test!(get_move_stats_164, 164, 79, 0, 0, 255.0 / 255.0, 10);
get_move_stats_test!(get_move_stats_165, 165, 48, 50, 0, 255.0 / 255.0, 10);
