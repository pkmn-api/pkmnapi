use pkmnapi_db::string::*;
use pkmnapi_db::*;

mod common;

macro_rules! get_tm_name_test {
    ($test_name:ident, $tm_id:expr, $tm_name:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_tm_name(&$tm_id) {
                Ok(tm_name) => assert_eq!(
                    tm_name,
                    TMName {
                        name: ROMString::from($tm_name)
                    },
                    "Searched for TM ID: {}",
                    $tm_id
                ),
                Err(_) => panic!(format!("Could not find TM ID: {}", $tm_id)),
            };
        }
    };
}

get_tm_name_test!(get_tm_name_1, 1, "TM01");
get_tm_name_test!(get_tm_name_2, 2, "TM02");
get_tm_name_test!(get_tm_name_3, 3, "TM03");
get_tm_name_test!(get_tm_name_4, 4, "TM04");
get_tm_name_test!(get_tm_name_5, 5, "TM05");
