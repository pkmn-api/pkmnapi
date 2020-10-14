use std::fs;

mod common;

macro_rules! get_trainer_pic_test {
    ($test_name:ident, $trainer_id:expr, $pic_filename:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();
            let pic_data = fs::read($pic_filename).unwrap();

            match db.get_trainer_pic(&$trainer_id) {
                Ok(pic) => assert_eq!(
                    pic.to_png(false).unwrap(),
                    pic_data,
                    "Searched for trainer ID: {}",
                    $trainer_id
                ),
                Err(_) => panic!(format!("Could not find trainer ID: {}", $trainer_id)),
            };
        }
    };
}

get_trainer_pic_test!(get_trainer_pic_1, 1, "../secrets/data/trainer_pic/1.png");
get_trainer_pic_test!(get_trainer_pic_2, 2, "../secrets/data/trainer_pic/2.png");
get_trainer_pic_test!(get_trainer_pic_3, 3, "../secrets/data/trainer_pic/3.png");
get_trainer_pic_test!(get_trainer_pic_4, 4, "../secrets/data/trainer_pic/4.png");
get_trainer_pic_test!(get_trainer_pic_5, 5, "../secrets/data/trainer_pic/5.png");
get_trainer_pic_test!(get_trainer_pic_6, 6, "../secrets/data/trainer_pic/6.png");
get_trainer_pic_test!(get_trainer_pic_7, 7, "../secrets/data/trainer_pic/7.png");
get_trainer_pic_test!(get_trainer_pic_8, 8, "../secrets/data/trainer_pic/8.png");
get_trainer_pic_test!(get_trainer_pic_9, 9, "../secrets/data/trainer_pic/9.png");
get_trainer_pic_test!(get_trainer_pic_10, 10, "../secrets/data/trainer_pic/10.png");
get_trainer_pic_test!(get_trainer_pic_11, 11, "../secrets/data/trainer_pic/11.png");
get_trainer_pic_test!(get_trainer_pic_12, 12, "../secrets/data/trainer_pic/12.png");
get_trainer_pic_test!(get_trainer_pic_13, 13, "../secrets/data/trainer_pic/13.png");
get_trainer_pic_test!(get_trainer_pic_14, 14, "../secrets/data/trainer_pic/14.png");
get_trainer_pic_test!(get_trainer_pic_15, 15, "../secrets/data/trainer_pic/15.png");
get_trainer_pic_test!(get_trainer_pic_16, 16, "../secrets/data/trainer_pic/16.png");
get_trainer_pic_test!(get_trainer_pic_17, 17, "../secrets/data/trainer_pic/17.png");
get_trainer_pic_test!(get_trainer_pic_18, 18, "../secrets/data/trainer_pic/18.png");
get_trainer_pic_test!(get_trainer_pic_19, 19, "../secrets/data/trainer_pic/19.png");
get_trainer_pic_test!(get_trainer_pic_20, 20, "../secrets/data/trainer_pic/20.png");
get_trainer_pic_test!(get_trainer_pic_21, 21, "../secrets/data/trainer_pic/21.png");
get_trainer_pic_test!(get_trainer_pic_22, 22, "../secrets/data/trainer_pic/22.png");
get_trainer_pic_test!(get_trainer_pic_23, 23, "../secrets/data/trainer_pic/23.png");
get_trainer_pic_test!(get_trainer_pic_24, 24, "../secrets/data/trainer_pic/24.png");
get_trainer_pic_test!(get_trainer_pic_25, 25, "../secrets/data/trainer_pic/25.png");
get_trainer_pic_test!(get_trainer_pic_26, 26, "../secrets/data/trainer_pic/26.png");
get_trainer_pic_test!(get_trainer_pic_27, 27, "../secrets/data/trainer_pic/27.png");
get_trainer_pic_test!(get_trainer_pic_28, 28, "../secrets/data/trainer_pic/28.png");
get_trainer_pic_test!(get_trainer_pic_29, 29, "../secrets/data/trainer_pic/29.png");
get_trainer_pic_test!(get_trainer_pic_30, 30, "../secrets/data/trainer_pic/30.png");
get_trainer_pic_test!(get_trainer_pic_31, 31, "../secrets/data/trainer_pic/31.png");
get_trainer_pic_test!(get_trainer_pic_32, 32, "../secrets/data/trainer_pic/32.png");
get_trainer_pic_test!(get_trainer_pic_33, 33, "../secrets/data/trainer_pic/33.png");
get_trainer_pic_test!(get_trainer_pic_34, 34, "../secrets/data/trainer_pic/34.png");
get_trainer_pic_test!(get_trainer_pic_35, 35, "../secrets/data/trainer_pic/35.png");
get_trainer_pic_test!(get_trainer_pic_36, 36, "../secrets/data/trainer_pic/36.png");
get_trainer_pic_test!(get_trainer_pic_37, 37, "../secrets/data/trainer_pic/37.png");
get_trainer_pic_test!(get_trainer_pic_38, 38, "../secrets/data/trainer_pic/38.png");
get_trainer_pic_test!(get_trainer_pic_39, 39, "../secrets/data/trainer_pic/39.png");
get_trainer_pic_test!(get_trainer_pic_40, 40, "../secrets/data/trainer_pic/40.png");
get_trainer_pic_test!(get_trainer_pic_41, 41, "../secrets/data/trainer_pic/41.png");
get_trainer_pic_test!(get_trainer_pic_42, 42, "../secrets/data/trainer_pic/42.png");
get_trainer_pic_test!(get_trainer_pic_43, 43, "../secrets/data/trainer_pic/43.png");
get_trainer_pic_test!(get_trainer_pic_44, 44, "../secrets/data/trainer_pic/44.png");
get_trainer_pic_test!(get_trainer_pic_45, 45, "../secrets/data/trainer_pic/45.png");
get_trainer_pic_test!(get_trainer_pic_46, 46, "../secrets/data/trainer_pic/46.png");
get_trainer_pic_test!(get_trainer_pic_47, 47, "../secrets/data/trainer_pic/47.png");
