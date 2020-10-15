use pkmnapi_db::patch::*;

mod common;

macro_rules! set_trainer_reward_test {
    (
        $test_name:ident,
        $trainer_id:expr,
        $trainer_reward:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_trainer_reward(&$trainer_id, &$trainer_reward) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for trainer ID: {}",
                    $trainer_id
                ),
                Err(_) => panic!(format!("Could not find trainer ID: {}", $trainer_id)),
            };
        }
    };
}

set_trainer_reward_test!(
    set_trainer_reward_1,
    1,
    1337,
    0x39916,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_2,
    2,
    1337,
    0x3991B,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_3,
    3,
    1337,
    0x39920,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_4,
    4,
    1337,
    0x39925,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_5,
    5,
    1337,
    0x3992A,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_6,
    6,
    1337,
    0x3992F,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_7,
    7,
    1337,
    0x39934,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_8,
    8,
    1337,
    0x39939,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_9,
    9,
    1337,
    0x3993E,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_10,
    10,
    1337,
    0x39943,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_11,
    11,
    1337,
    0x39948,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_12,
    12,
    1337,
    0x3994D,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_13,
    13,
    1337,
    0x39952,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_14,
    14,
    1337,
    0x39957,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_15,
    15,
    1337,
    0x3995C,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_16,
    16,
    1337,
    0x39961,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_17,
    17,
    1337,
    0x39966,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_18,
    18,
    1337,
    0x3996B,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_19,
    19,
    1337,
    0x39970,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_20,
    20,
    1337,
    0x39975,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_21,
    21,
    1337,
    0x3997A,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_22,
    22,
    1337,
    0x3997F,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_23,
    23,
    1337,
    0x39984,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_24,
    24,
    1337,
    0x39989,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_25,
    25,
    1337,
    0x3998E,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_26,
    26,
    1337,
    0x39993,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_27,
    27,
    1337,
    0x39998,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_28,
    28,
    1337,
    0x3999D,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_29,
    29,
    1337,
    0x399A2,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_30,
    30,
    1337,
    0x399A7,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_31,
    31,
    1337,
    0x399AC,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_32,
    32,
    1337,
    0x399B1,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_33,
    33,
    1337,
    0x399B6,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_34,
    34,
    1337,
    0x399BB,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_35,
    35,
    1337,
    0x399C0,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_36,
    36,
    1337,
    0x399C5,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_37,
    37,
    1337,
    0x399CA,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_38,
    38,
    1337,
    0x399CF,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_39,
    39,
    1337,
    0x399D4,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_40,
    40,
    1337,
    0x399D9,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_41,
    41,
    1337,
    0x399DE,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_42,
    42,
    1337,
    0x399E3,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_43,
    43,
    1337,
    0x399E8,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_44,
    44,
    1337,
    0x399ED,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_45,
    45,
    1337,
    0x399F2,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_46,
    46,
    1337,
    0x399F7,
    vec![0x00, 0x13, 0x37]
);
set_trainer_reward_test!(
    set_trainer_reward_47,
    47,
    1337,
    0x399FC,
    vec![0x00, 0x13, 0x37]
);
