use pkmnapi_db::string::*;
use pkmnapi_db::*;

mod common;

macro_rules! get_hm_name_test {
    ($test_name:ident, $hm_id:expr, $hm_name:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_hm_name(&$hm_id) {
                Ok(hm_name) => assert_eq!(
                    hm_name,
                    HMName {
                        name: ROMString::from($hm_name)
                    },
                    "Searched for HM ID: {}",
                    $hm_id
                ),
                Err(_) => panic!(format!("Could not find HM ID: {}", $hm_id)),
            };
        }
    };
}

get_hm_name_test!(get_hm_name_1, 1, "HM01");
get_hm_name_test!(get_hm_name_2, 2, "HM02");
get_hm_name_test!(get_hm_name_3, 3, "HM03");
get_hm_name_test!(get_hm_name_4, 4, "HM04");
get_hm_name_test!(get_hm_name_5, 5, "HM05");
