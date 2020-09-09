use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! get_trainer_name_test {
    ($test_name: ident, $trainer_id: expr, $trainer_name: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_trainer_name(&$trainer_id) {
                Ok(trainer_name) => assert_eq!(
                    trainer_name,
                    TrainerName {
                        name: ROMString::from($trainer_name)
                    },
                    "Searched for trainer ID: {}",
                    $trainer_id
                ),
                Err(_) => panic!(format!("Could not find trainer ID: {}", $trainer_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_trainer_name_test)]

get_trainer_name_test!(get_trainer_name_0, 0, "YOUNGSTER");
get_trainer_name_test!(get_trainer_name_1, 1, "BUG CATCHER");
get_trainer_name_test!(get_trainer_name_2, 2, "LASS");
get_trainer_name_test!(get_trainer_name_3, 3, "SAILOR");
get_trainer_name_test!(get_trainer_name_4, 4, "JR.TRAINER♂");
get_trainer_name_test!(get_trainer_name_5, 5, "JR.TRAINER♀");
get_trainer_name_test!(get_trainer_name_6, 6, "POKéMANIAC");
get_trainer_name_test!(get_trainer_name_7, 7, "SUPER NERD");
get_trainer_name_test!(get_trainer_name_8, 8, "HIKER");
get_trainer_name_test!(get_trainer_name_9, 9, "BIKER");
get_trainer_name_test!(get_trainer_name_10, 10, "BURGLAR");
get_trainer_name_test!(get_trainer_name_11, 11, "ENGINEER");
get_trainer_name_test!(get_trainer_name_12, 12, "JUGGLER");
get_trainer_name_test!(get_trainer_name_13, 13, "FISHERMAN");
get_trainer_name_test!(get_trainer_name_14, 14, "SWIMMER");
get_trainer_name_test!(get_trainer_name_15, 15, "CUE BALL");
get_trainer_name_test!(get_trainer_name_16, 16, "GAMBLER");
get_trainer_name_test!(get_trainer_name_17, 17, "BEAUTY");
get_trainer_name_test!(get_trainer_name_18, 18, "PSYCHIC");
get_trainer_name_test!(get_trainer_name_19, 19, "ROCKER");
get_trainer_name_test!(get_trainer_name_20, 20, "JUGGLER");
get_trainer_name_test!(get_trainer_name_21, 21, "TAMER");
get_trainer_name_test!(get_trainer_name_22, 22, "BIRD KEEPER");
get_trainer_name_test!(get_trainer_name_23, 23, "BLACKBELT");
get_trainer_name_test!(get_trainer_name_24, 24, "RIVAL1");
get_trainer_name_test!(get_trainer_name_25, 25, "PROF.OAK");
get_trainer_name_test!(get_trainer_name_26, 26, "CHIEF");
get_trainer_name_test!(get_trainer_name_27, 27, "SCIENTIST");
get_trainer_name_test!(get_trainer_name_28, 28, "GIOVANNI");
get_trainer_name_test!(get_trainer_name_29, 29, "ROCKET");
get_trainer_name_test!(get_trainer_name_30, 30, "COOLTRAINER♂");
get_trainer_name_test!(get_trainer_name_31, 31, "COOLTRAINER♀");
get_trainer_name_test!(get_trainer_name_32, 32, "BRUNO");
get_trainer_name_test!(get_trainer_name_33, 33, "BROCK");
get_trainer_name_test!(get_trainer_name_34, 34, "MISTY");
get_trainer_name_test!(get_trainer_name_35, 35, "LT.SURGE");
get_trainer_name_test!(get_trainer_name_36, 36, "ERIKA");
get_trainer_name_test!(get_trainer_name_37, 37, "KOGA");
get_trainer_name_test!(get_trainer_name_38, 38, "BLAINE");
get_trainer_name_test!(get_trainer_name_39, 39, "SABRINA");
get_trainer_name_test!(get_trainer_name_40, 40, "GENTLEMAN");
get_trainer_name_test!(get_trainer_name_41, 41, "RIVAL2");
get_trainer_name_test!(get_trainer_name_42, 42, "RIVAL3");
get_trainer_name_test!(get_trainer_name_43, 43, "LORELEI");
get_trainer_name_test!(get_trainer_name_44, 44, "CHANNELER");
get_trainer_name_test!(get_trainer_name_45, 45, "AGATHA");
get_trainer_name_test!(get_trainer_name_46, 46, "LANCE");
