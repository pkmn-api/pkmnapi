macro_rules! set_move_stats_test {
    ($test_name: ident, $move_id: expr, $effect: expr, $power: expr, $type_id: expr, $accuracy: expr, $pp: expr, $patch_offset: expr, $patch_data: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_move_stats(
                $move_id,
                PkmnapiDBMoveStats {
                    move_id: PkmnapiDBMoveID::from($move_id),
                    effect: $effect,
                    power: $power,
                    type_id: PkmnapiDBTypeID::from($type_id),
                    accuracy: $accuracy,
                    pp: $pp,
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    PkmnapiDBPatch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for move ID: {}",
                    $move_id
                ),
                Err(_) => panic!(format!("Could not find move ID: {}", $move_id)),
            };
        }
    };
}

#[cfg(test)]
#[rustfmt::skip::macros(set_move_stats_test)]
mod tests {
    use pkmnapi::db::patch::*;
    use pkmnapi::db::types::*;

    mod common;

    set_move_stats_test!(set_move_stats_1, 1, 0, 1, 2, 42.0 / 255.0, 20, 0x38000, vec![1, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_2, 2, 0, 1, 2, 42.0 / 255.0, 20, 0x38006, vec![2, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_3, 3, 0, 1, 2, 42.0 / 255.0, 20, 0x3800C, vec![3, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_4, 4, 0, 1, 2, 42.0 / 255.0, 20, 0x38012, vec![4, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_5, 5, 0, 1, 2, 42.0 / 255.0, 20, 0x38018, vec![5, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_6, 6, 0, 1, 2, 42.0 / 255.0, 20, 0x3801E, vec![6, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_7, 7, 0, 1, 2, 42.0 / 255.0, 20, 0x38024, vec![7, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_8, 8, 0, 1, 2, 42.0 / 255.0, 20, 0x3802A, vec![8, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_9, 9, 0, 1, 2, 42.0 / 255.0, 20, 0x38030, vec![9, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_10, 10, 0, 1, 2, 42.0 / 255.0, 20, 0x38036, vec![10, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_11, 11, 0, 1, 2, 42.0 / 255.0, 20, 0x3803C, vec![11, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_12, 12, 0, 1, 2, 42.0 / 255.0, 20, 0x38042, vec![12, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_13, 13, 0, 1, 2, 42.0 / 255.0, 20, 0x38048, vec![13, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_14, 14, 0, 1, 2, 42.0 / 255.0, 20, 0x3804E, vec![14, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_15, 15, 0, 1, 2, 42.0 / 255.0, 20, 0x38054, vec![15, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_16, 16, 0, 1, 2, 42.0 / 255.0, 20, 0x3805A, vec![16, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_17, 17, 0, 1, 2, 42.0 / 255.0, 20, 0x38060, vec![17, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_18, 18, 0, 1, 2, 42.0 / 255.0, 20, 0x38066, vec![18, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_19, 19, 0, 1, 2, 42.0 / 255.0, 20, 0x3806C, vec![19, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_20, 20, 0, 1, 2, 42.0 / 255.0, 20, 0x38072, vec![20, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_21, 21, 0, 1, 2, 42.0 / 255.0, 20, 0x38078, vec![21, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_22, 22, 0, 1, 2, 42.0 / 255.0, 20, 0x3807E, vec![22, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_23, 23, 0, 1, 2, 42.0 / 255.0, 20, 0x38084, vec![23, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_24, 24, 0, 1, 2, 42.0 / 255.0, 20, 0x3808A, vec![24, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_25, 25, 0, 1, 2, 42.0 / 255.0, 20, 0x38090, vec![25, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_26, 26, 0, 1, 2, 42.0 / 255.0, 20, 0x38096, vec![26, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_27, 27, 0, 1, 2, 42.0 / 255.0, 20, 0x3809C, vec![27, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_28, 28, 0, 1, 2, 42.0 / 255.0, 20, 0x380A2, vec![28, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_29, 29, 0, 1, 2, 42.0 / 255.0, 20, 0x380A8, vec![29, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_30, 30, 0, 1, 2, 42.0 / 255.0, 20, 0x380AE, vec![30, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_31, 31, 0, 1, 2, 42.0 / 255.0, 20, 0x380B4, vec![31, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_32, 32, 0, 1, 2, 42.0 / 255.0, 20, 0x380BA, vec![32, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_33, 33, 0, 1, 2, 42.0 / 255.0, 20, 0x380C0, vec![33, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_34, 34, 0, 1, 2, 42.0 / 255.0, 20, 0x380C6, vec![34, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_35, 35, 0, 1, 2, 42.0 / 255.0, 20, 0x380CC, vec![35, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_36, 36, 0, 1, 2, 42.0 / 255.0, 20, 0x380D2, vec![36, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_37, 37, 0, 1, 2, 42.0 / 255.0, 20, 0x380D8, vec![37, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_38, 38, 0, 1, 2, 42.0 / 255.0, 20, 0x380DE, vec![38, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_39, 39, 0, 1, 2, 42.0 / 255.0, 20, 0x380E4, vec![39, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_40, 40, 0, 1, 2, 42.0 / 255.0, 20, 0x380EA, vec![40, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_41, 41, 0, 1, 2, 42.0 / 255.0, 20, 0x380F0, vec![41, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_42, 42, 0, 1, 2, 42.0 / 255.0, 20, 0x380F6, vec![42, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_43, 43, 0, 1, 2, 42.0 / 255.0, 20, 0x380FC, vec![43, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_44, 44, 0, 1, 2, 42.0 / 255.0, 20, 0x38102, vec![44, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_45, 45, 0, 1, 2, 42.0 / 255.0, 20, 0x38108, vec![45, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_46, 46, 0, 1, 2, 42.0 / 255.0, 20, 0x3810E, vec![46, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_47, 47, 0, 1, 2, 42.0 / 255.0, 20, 0x38114, vec![47, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_48, 48, 0, 1, 2, 42.0 / 255.0, 20, 0x3811A, vec![48, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_49, 49, 0, 1, 2, 42.0 / 255.0, 20, 0x38120, vec![49, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_50, 50, 0, 1, 2, 42.0 / 255.0, 20, 0x38126, vec![50, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_51, 51, 0, 1, 2, 42.0 / 255.0, 20, 0x3812C, vec![51, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_52, 52, 0, 1, 2, 42.0 / 255.0, 20, 0x38132, vec![52, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_53, 53, 0, 1, 2, 42.0 / 255.0, 20, 0x38138, vec![53, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_54, 54, 0, 1, 2, 42.0 / 255.0, 20, 0x3813E, vec![54, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_55, 55, 0, 1, 2, 42.0 / 255.0, 20, 0x38144, vec![55, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_56, 56, 0, 1, 2, 42.0 / 255.0, 20, 0x3814A, vec![56, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_57, 57, 0, 1, 2, 42.0 / 255.0, 20, 0x38150, vec![57, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_58, 58, 0, 1, 2, 42.0 / 255.0, 20, 0x38156, vec![58, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_59, 59, 0, 1, 2, 42.0 / 255.0, 20, 0x3815C, vec![59, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_60, 60, 0, 1, 2, 42.0 / 255.0, 20, 0x38162, vec![60, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_61, 61, 0, 1, 2, 42.0 / 255.0, 20, 0x38168, vec![61, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_62, 62, 0, 1, 2, 42.0 / 255.0, 20, 0x3816E, vec![62, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_63, 63, 0, 1, 2, 42.0 / 255.0, 20, 0x38174, vec![63, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_64, 64, 0, 1, 2, 42.0 / 255.0, 20, 0x3817A, vec![64, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_65, 65, 0, 1, 2, 42.0 / 255.0, 20, 0x38180, vec![65, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_66, 66, 0, 1, 2, 42.0 / 255.0, 20, 0x38186, vec![66, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_67, 67, 0, 1, 2, 42.0 / 255.0, 20, 0x3818C, vec![67, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_68, 68, 0, 1, 2, 42.0 / 255.0, 20, 0x38192, vec![68, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_69, 69, 0, 1, 2, 42.0 / 255.0, 20, 0x38198, vec![69, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_70, 70, 0, 1, 2, 42.0 / 255.0, 20, 0x3819E, vec![70, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_71, 71, 0, 1, 2, 42.0 / 255.0, 20, 0x381A4, vec![71, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_72, 72, 0, 1, 2, 42.0 / 255.0, 20, 0x381AA, vec![72, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_73, 73, 0, 1, 2, 42.0 / 255.0, 20, 0x381B0, vec![73, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_74, 74, 0, 1, 2, 42.0 / 255.0, 20, 0x381B6, vec![74, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_75, 75, 0, 1, 2, 42.0 / 255.0, 20, 0x381BC, vec![75, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_76, 76, 0, 1, 2, 42.0 / 255.0, 20, 0x381C2, vec![76, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_77, 77, 0, 1, 2, 42.0 / 255.0, 20, 0x381C8, vec![77, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_78, 78, 0, 1, 2, 42.0 / 255.0, 20, 0x381CE, vec![78, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_79, 79, 0, 1, 2, 42.0 / 255.0, 20, 0x381D4, vec![79, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_80, 80, 0, 1, 2, 42.0 / 255.0, 20, 0x381DA, vec![80, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_81, 81, 0, 1, 2, 42.0 / 255.0, 20, 0x381E0, vec![81, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_82, 82, 0, 1, 2, 42.0 / 255.0, 20, 0x381E6, vec![82, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_83, 83, 0, 1, 2, 42.0 / 255.0, 20, 0x381EC, vec![83, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_84, 84, 0, 1, 2, 42.0 / 255.0, 20, 0x381F2, vec![84, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_85, 85, 0, 1, 2, 42.0 / 255.0, 20, 0x381F8, vec![85, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_86, 86, 0, 1, 2, 42.0 / 255.0, 20, 0x381FE, vec![86, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_87, 87, 0, 1, 2, 42.0 / 255.0, 20, 0x38204, vec![87, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_88, 88, 0, 1, 2, 42.0 / 255.0, 20, 0x3820A, vec![88, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_89, 89, 0, 1, 2, 42.0 / 255.0, 20, 0x38210, vec![89, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_90, 90, 0, 1, 2, 42.0 / 255.0, 20, 0x38216, vec![90, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_91, 91, 0, 1, 2, 42.0 / 255.0, 20, 0x3821C, vec![91, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_92, 92, 0, 1, 2, 42.0 / 255.0, 20, 0x38222, vec![92, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_93, 93, 0, 1, 2, 42.0 / 255.0, 20, 0x38228, vec![93, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_94, 94, 0, 1, 2, 42.0 / 255.0, 20, 0x3822E, vec![94, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_95, 95, 0, 1, 2, 42.0 / 255.0, 20, 0x38234, vec![95, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_96, 96, 0, 1, 2, 42.0 / 255.0, 20, 0x3823A, vec![96, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_97, 97, 0, 1, 2, 42.0 / 255.0, 20, 0x38240, vec![97, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_98, 98, 0, 1, 2, 42.0 / 255.0, 20, 0x38246, vec![98, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_99, 99, 0, 1, 2, 42.0 / 255.0, 20, 0x3824C, vec![99, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_100, 100, 0, 1, 2, 42.0 / 255.0, 20, 0x38252, vec![100, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_101, 101, 0, 1, 2, 42.0 / 255.0, 20, 0x38258, vec![101, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_102, 102, 0, 1, 2, 42.0 / 255.0, 20, 0x3825E, vec![102, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_103, 103, 0, 1, 2, 42.0 / 255.0, 20, 0x38264, vec![103, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_104, 104, 0, 1, 2, 42.0 / 255.0, 20, 0x3826A, vec![104, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_105, 105, 0, 1, 2, 42.0 / 255.0, 20, 0x38270, vec![105, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_106, 106, 0, 1, 2, 42.0 / 255.0, 20, 0x38276, vec![106, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_107, 107, 0, 1, 2, 42.0 / 255.0, 20, 0x3827C, vec![107, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_108, 108, 0, 1, 2, 42.0 / 255.0, 20, 0x38282, vec![108, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_109, 109, 0, 1, 2, 42.0 / 255.0, 20, 0x38288, vec![109, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_110, 110, 0, 1, 2, 42.0 / 255.0, 20, 0x3828E, vec![110, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_111, 111, 0, 1, 2, 42.0 / 255.0, 20, 0x38294, vec![111, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_112, 112, 0, 1, 2, 42.0 / 255.0, 20, 0x3829A, vec![112, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_113, 113, 0, 1, 2, 42.0 / 255.0, 20, 0x382A0, vec![113, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_114, 114, 0, 1, 2, 42.0 / 255.0, 20, 0x382A6, vec![114, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_115, 115, 0, 1, 2, 42.0 / 255.0, 20, 0x382AC, vec![115, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_116, 116, 0, 1, 2, 42.0 / 255.0, 20, 0x382B2, vec![116, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_117, 117, 0, 1, 2, 42.0 / 255.0, 20, 0x382B8, vec![117, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_118, 118, 0, 1, 2, 42.0 / 255.0, 20, 0x382BE, vec![118, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_119, 119, 0, 1, 2, 42.0 / 255.0, 20, 0x382C4, vec![119, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_120, 120, 0, 1, 2, 42.0 / 255.0, 20, 0x382CA, vec![120, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_121, 121, 0, 1, 2, 42.0 / 255.0, 20, 0x382D0, vec![121, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_122, 122, 0, 1, 2, 42.0 / 255.0, 20, 0x382D6, vec![122, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_123, 123, 0, 1, 2, 42.0 / 255.0, 20, 0x382DC, vec![123, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_124, 124, 0, 1, 2, 42.0 / 255.0, 20, 0x382E2, vec![124, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_125, 125, 0, 1, 2, 42.0 / 255.0, 20, 0x382E8, vec![125, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_126, 126, 0, 1, 2, 42.0 / 255.0, 20, 0x382EE, vec![126, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_127, 127, 0, 1, 2, 42.0 / 255.0, 20, 0x382F4, vec![127, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_128, 128, 0, 1, 2, 42.0 / 255.0, 20, 0x382FA, vec![128, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_129, 129, 0, 1, 2, 42.0 / 255.0, 20, 0x38300, vec![129, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_130, 130, 0, 1, 2, 42.0 / 255.0, 20, 0x38306, vec![130, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_131, 131, 0, 1, 2, 42.0 / 255.0, 20, 0x3830C, vec![131, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_132, 132, 0, 1, 2, 42.0 / 255.0, 20, 0x38312, vec![132, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_133, 133, 0, 1, 2, 42.0 / 255.0, 20, 0x38318, vec![133, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_134, 134, 0, 1, 2, 42.0 / 255.0, 20, 0x3831E, vec![134, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_135, 135, 0, 1, 2, 42.0 / 255.0, 20, 0x38324, vec![135, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_136, 136, 0, 1, 2, 42.0 / 255.0, 20, 0x3832A, vec![136, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_137, 137, 0, 1, 2, 42.0 / 255.0, 20, 0x38330, vec![137, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_138, 138, 0, 1, 2, 42.0 / 255.0, 20, 0x38336, vec![138, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_139, 139, 0, 1, 2, 42.0 / 255.0, 20, 0x3833C, vec![139, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_140, 140, 0, 1, 2, 42.0 / 255.0, 20, 0x38342, vec![140, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_141, 141, 0, 1, 2, 42.0 / 255.0, 20, 0x38348, vec![141, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_142, 142, 0, 1, 2, 42.0 / 255.0, 20, 0x3834E, vec![142, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_143, 143, 0, 1, 2, 42.0 / 255.0, 20, 0x38354, vec![143, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_144, 144, 0, 1, 2, 42.0 / 255.0, 20, 0x3835A, vec![144, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_145, 145, 0, 1, 2, 42.0 / 255.0, 20, 0x38360, vec![145, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_146, 146, 0, 1, 2, 42.0 / 255.0, 20, 0x38366, vec![146, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_147, 147, 0, 1, 2, 42.0 / 255.0, 20, 0x3836C, vec![147, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_148, 148, 0, 1, 2, 42.0 / 255.0, 20, 0x38372, vec![148, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_149, 149, 0, 1, 2, 42.0 / 255.0, 20, 0x38378, vec![149, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_150, 150, 0, 1, 2, 42.0 / 255.0, 20, 0x3837E, vec![150, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_151, 151, 0, 1, 2, 42.0 / 255.0, 20, 0x38384, vec![151, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_152, 152, 0, 1, 2, 42.0 / 255.0, 20, 0x3838A, vec![152, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_153, 153, 0, 1, 2, 42.0 / 255.0, 20, 0x38390, vec![153, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_154, 154, 0, 1, 2, 42.0 / 255.0, 20, 0x38396, vec![154, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_155, 155, 0, 1, 2, 42.0 / 255.0, 20, 0x3839C, vec![155, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_156, 156, 0, 1, 2, 42.0 / 255.0, 20, 0x383A2, vec![156, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_157, 157, 0, 1, 2, 42.0 / 255.0, 20, 0x383A8, vec![157, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_158, 158, 0, 1, 2, 42.0 / 255.0, 20, 0x383AE, vec![158, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_159, 159, 0, 1, 2, 42.0 / 255.0, 20, 0x383B4, vec![159, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_160, 160, 0, 1, 2, 42.0 / 255.0, 20, 0x383BA, vec![160, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_161, 161, 0, 1, 2, 42.0 / 255.0, 20, 0x383C0, vec![161, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_162, 162, 0, 1, 2, 42.0 / 255.0, 20, 0x383C6, vec![162, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_163, 163, 0, 1, 2, 42.0 / 255.0, 20, 0x383CC, vec![163, 0, 1, 2, 42, 20]);
    set_move_stats_test!(set_move_stats_164, 164, 0, 1, 2, 42.0 / 255.0, 20, 0x383D2, vec![164, 0, 1, 2, 42, 20]);
}
