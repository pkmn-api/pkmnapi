use std::fs;

mod common;

macro_rules! get_icon_test {
    ($test_name:ident, $icon_id:expr, $icon_filename:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();
            let icon_data = fs::read($icon_filename).unwrap();

            match db.get_icon(&$icon_id) {
                Ok(icon) => assert_eq!(
                    icon.to_gif(26).unwrap(),
                    icon_data,
                    "Searched for icon ID: {}",
                    $icon_id
                ),
                Err(_) => panic!(format!("Could not find icon ID: {}", $icon_id)),
            };
        }
    };
}

get_icon_test!(get_icon_0, 0, "../secrets/data/icon/0.gif");
get_icon_test!(get_icon_1, 1, "../secrets/data/icon/1.gif");
get_icon_test!(get_icon_2, 2, "../secrets/data/icon/2.gif");
get_icon_test!(get_icon_3, 3, "../secrets/data/icon/3.gif");
get_icon_test!(get_icon_4, 4, "../secrets/data/icon/4.gif");
get_icon_test!(get_icon_5, 5, "../secrets/data/icon/5.gif");
get_icon_test!(get_icon_6, 6, "../secrets/data/icon/6.gif");
get_icon_test!(get_icon_7, 7, "../secrets/data/icon/7.gif");
get_icon_test!(get_icon_8, 8, "../secrets/data/icon/8.gif");
get_icon_test!(get_icon_9, 9, "../secrets/data/icon/9.gif");
