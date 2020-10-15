mod common;

macro_rules! get_trainer_reward_test {
    ($test_name:ident, $trainer_id:expr, $trainer_reward:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_trainer_reward(&$trainer_id) {
                Ok(trainer_reward) => assert_eq!(
                    trainer_reward, $trainer_reward,
                    "Searched for trainer ID: {}",
                    $trainer_id
                ),
                Err(_) => panic!(format!("Could not find trainer ID: {}", $trainer_id)),
            };
        }
    };
}

get_trainer_reward_test!(get_trainer_reward_1, 1, 1500);
get_trainer_reward_test!(get_trainer_reward_2, 2, 1000);
get_trainer_reward_test!(get_trainer_reward_3, 3, 1500);
get_trainer_reward_test!(get_trainer_reward_4, 4, 3000);
get_trainer_reward_test!(get_trainer_reward_5, 5, 2000);
get_trainer_reward_test!(get_trainer_reward_6, 6, 2000);
get_trainer_reward_test!(get_trainer_reward_7, 7, 5000);
get_trainer_reward_test!(get_trainer_reward_8, 8, 2500);
get_trainer_reward_test!(get_trainer_reward_9, 9, 3500);
get_trainer_reward_test!(get_trainer_reward_10, 10, 2000);
get_trainer_reward_test!(get_trainer_reward_11, 11, 9000);
get_trainer_reward_test!(get_trainer_reward_12, 12, 5000);
get_trainer_reward_test!(get_trainer_reward_13, 13, 3500);
get_trainer_reward_test!(get_trainer_reward_14, 14, 3500);
get_trainer_reward_test!(get_trainer_reward_15, 15, 500);
get_trainer_reward_test!(get_trainer_reward_16, 16, 2500);
get_trainer_reward_test!(get_trainer_reward_17, 17, 7000);
get_trainer_reward_test!(get_trainer_reward_18, 18, 7000);
get_trainer_reward_test!(get_trainer_reward_19, 19, 1000);
get_trainer_reward_test!(get_trainer_reward_20, 20, 2500);
get_trainer_reward_test!(get_trainer_reward_21, 21, 3500);
get_trainer_reward_test!(get_trainer_reward_22, 22, 4000);
get_trainer_reward_test!(get_trainer_reward_23, 23, 2500);
get_trainer_reward_test!(get_trainer_reward_24, 24, 2500);
get_trainer_reward_test!(get_trainer_reward_25, 25, 3500);
get_trainer_reward_test!(get_trainer_reward_26, 26, 9900);
get_trainer_reward_test!(get_trainer_reward_27, 27, 3000);
get_trainer_reward_test!(get_trainer_reward_28, 28, 5000);
get_trainer_reward_test!(get_trainer_reward_29, 29, 9900);
get_trainer_reward_test!(get_trainer_reward_30, 30, 3000);
get_trainer_reward_test!(get_trainer_reward_31, 31, 3500);
get_trainer_reward_test!(get_trainer_reward_32, 32, 3500);
get_trainer_reward_test!(get_trainer_reward_33, 33, 9900);
get_trainer_reward_test!(get_trainer_reward_34, 34, 9900);
get_trainer_reward_test!(get_trainer_reward_35, 35, 9900);
get_trainer_reward_test!(get_trainer_reward_36, 36, 9900);
get_trainer_reward_test!(get_trainer_reward_37, 37, 9900);
get_trainer_reward_test!(get_trainer_reward_38, 38, 9900);
get_trainer_reward_test!(get_trainer_reward_39, 39, 9900);
get_trainer_reward_test!(get_trainer_reward_40, 40, 9900);
get_trainer_reward_test!(get_trainer_reward_41, 41, 7000);
get_trainer_reward_test!(get_trainer_reward_42, 42, 6500);
get_trainer_reward_test!(get_trainer_reward_43, 43, 9900);
get_trainer_reward_test!(get_trainer_reward_44, 44, 9900);
get_trainer_reward_test!(get_trainer_reward_45, 45, 3000);
get_trainer_reward_test!(get_trainer_reward_46, 46, 9900);
get_trainer_reward_test!(get_trainer_reward_47, 47, 9900);
