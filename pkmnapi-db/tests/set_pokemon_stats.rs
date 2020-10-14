use pkmnapi_db::patch::*;
use pkmnapi_db::*;

mod common;

macro_rules! set_pokemon_stats_test {
    (
        $test_name:ident,
        $pokedex_id:expr,
        $base_hp:expr,
        $base_attack:expr,
        $base_defence:expr,
        $base_speed:expr,
        $base_special:expr,
        $type_ids:expr,
        $catch_rate:expr,
        $base_exp_yield:expr,
        $patch_offset:expr,
        $patch_data:expr
    ) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.set_pokemon_stats(
                &$pokedex_id,
                &PokemonStats {
                    pokedex_id: $pokedex_id,
                    base_hp: $base_hp,
                    base_attack: $base_attack,
                    base_defence: $base_defence,
                    base_speed: $base_speed,
                    base_special: $base_special,
                    type_ids: $type_ids,
                    catch_rate: $catch_rate,
                    base_exp_yield: $base_exp_yield,
                },
            ) {
                Ok(patch) => assert_eq!(
                    patch,
                    Patch {
                        offset: $patch_offset,
                        length: $patch_data.len(),
                        data: $patch_data
                    },
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

set_pokemon_stats_test!(
    set_states_1,
    1,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x383DE,
    vec![0x01, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_2,
    2,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x383FA,
    vec![0x02, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_3,
    3,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38416,
    vec![0x03, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_4,
    4,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38432,
    vec![0x04, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_5,
    5,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3844E,
    vec![0x05, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_6,
    6,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3846A,
    vec![0x06, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_7,
    7,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38486,
    vec![0x07, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_8,
    8,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x384A2,
    vec![0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_9,
    9,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x384BE,
    vec![0x09, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_10,
    10,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x384DA,
    vec![0x0A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_11,
    11,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x384F6,
    vec![0x0B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_12,
    12,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38512,
    vec![0x0C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_13,
    13,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3852E,
    vec![0x0D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_14,
    14,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3854A,
    vec![0x0E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_15,
    15,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38566,
    vec![0x0F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_16,
    16,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38582,
    vec![0x10, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_17,
    17,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3859E,
    vec![0x11, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_18,
    18,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x385BA,
    vec![0x12, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_19,
    19,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x385D6,
    vec![0x13, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_20,
    20,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x385F2,
    vec![0x14, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_21,
    21,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3860E,
    vec![0x15, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_22,
    22,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3862A,
    vec![0x16, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_23,
    23,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38646,
    vec![0x17, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_24,
    24,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38662,
    vec![0x18, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_25,
    25,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3867E,
    vec![0x19, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_26,
    26,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3869A,
    vec![0x1A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_27,
    27,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x386B6,
    vec![0x1B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_28,
    28,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x386D2,
    vec![0x1C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_29,
    29,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x386EE,
    vec![0x1D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_30,
    30,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3870A,
    vec![0x1E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_31,
    31,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38726,
    vec![0x1F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_32,
    32,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38742,
    vec![0x20, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_33,
    33,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3875E,
    vec![0x21, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_34,
    34,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3877A,
    vec![0x22, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_35,
    35,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38796,
    vec![0x23, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_36,
    36,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x387B2,
    vec![0x24, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_37,
    37,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x387CE,
    vec![0x25, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_38,
    38,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x387EA,
    vec![0x26, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_39,
    39,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38806,
    vec![0x27, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_40,
    40,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38822,
    vec![0x28, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_41,
    41,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3883E,
    vec![0x29, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_42,
    42,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3885A,
    vec![0x2A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_43,
    43,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38876,
    vec![0x2B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_44,
    44,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38892,
    vec![0x2C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_45,
    45,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x388AE,
    vec![0x2D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_46,
    46,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x388CA,
    vec![0x2E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_47,
    47,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x388E6,
    vec![0x2F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_48,
    48,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38902,
    vec![0x30, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_49,
    49,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3891E,
    vec![0x31, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_50,
    50,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3893A,
    vec![0x32, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_51,
    51,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38956,
    vec![0x33, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_52,
    52,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38972,
    vec![0x34, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_53,
    53,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3898E,
    vec![0x35, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_54,
    54,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x389AA,
    vec![0x36, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_55,
    55,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x389C6,
    vec![0x37, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_56,
    56,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x389E2,
    vec![0x38, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_57,
    57,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x389FE,
    vec![0x39, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_58,
    58,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38A1A,
    vec![0x3A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_59,
    59,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38A36,
    vec![0x3B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_60,
    60,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38A52,
    vec![0x3C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_61,
    61,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38A6E,
    vec![0x3D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_62,
    62,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38A8A,
    vec![0x3E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_63,
    63,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38AA6,
    vec![0x3F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_64,
    64,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38AC2,
    vec![0x40, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_65,
    65,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38ADE,
    vec![0x41, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_66,
    66,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38AFA,
    vec![0x42, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_67,
    67,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38B16,
    vec![0x43, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_68,
    68,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38B32,
    vec![0x44, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_69,
    69,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38B4E,
    vec![0x45, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_70,
    70,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38B6A,
    vec![0x46, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_71,
    71,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38B86,
    vec![0x47, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_72,
    72,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38BA2,
    vec![0x48, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_73,
    73,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38BBE,
    vec![0x49, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_74,
    74,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38BDA,
    vec![0x4A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_75,
    75,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38BF6,
    vec![0x4B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_76,
    76,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38C12,
    vec![0x4C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_77,
    77,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38C2E,
    vec![0x4D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_78,
    78,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38C4A,
    vec![0x4E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_79,
    79,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38C66,
    vec![0x4F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_80,
    80,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38C82,
    vec![0x50, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_81,
    81,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38C9E,
    vec![0x51, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_82,
    82,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38CBA,
    vec![0x52, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_83,
    83,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38CD6,
    vec![0x53, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_84,
    84,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38CF2,
    vec![0x54, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_85,
    85,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38D0E,
    vec![0x55, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_86,
    86,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38D2A,
    vec![0x56, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_87,
    87,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38D46,
    vec![0x57, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_88,
    88,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38D62,
    vec![0x58, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_89,
    89,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38D7E,
    vec![0x59, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_90,
    90,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38D9A,
    vec![0x5A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_91,
    91,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38DB6,
    vec![0x5B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_92,
    92,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38DD2,
    vec![0x5C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_93,
    93,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38DEE,
    vec![0x5D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_94,
    94,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38E0A,
    vec![0x5E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_95,
    95,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38E26,
    vec![0x5F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_96,
    96,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38E42,
    vec![0x60, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_97,
    97,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38E5E,
    vec![0x61, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_98,
    98,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38E7A,
    vec![0x62, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_99,
    99,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38E96,
    vec![0x63, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_100,
    100,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38EB2,
    vec![0x64, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_101,
    101,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38ECE,
    vec![0x65, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_102,
    102,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38EEA,
    vec![0x66, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_103,
    103,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38F06,
    vec![0x67, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_104,
    104,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38F22,
    vec![0x68, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_105,
    105,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38F3E,
    vec![0x69, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_106,
    106,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38F5A,
    vec![0x6A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_107,
    107,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38F76,
    vec![0x6B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_108,
    108,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38F92,
    vec![0x6C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_109,
    109,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38FAE,
    vec![0x6D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_110,
    110,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38FCA,
    vec![0x6E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_111,
    111,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x38FE6,
    vec![0x6F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_112,
    112,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39002,
    vec![0x70, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_113,
    113,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3901E,
    vec![0x71, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_114,
    114,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3903A,
    vec![0x72, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_115,
    115,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39056,
    vec![0x73, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_116,
    116,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39072,
    vec![0x74, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_117,
    117,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3908E,
    vec![0x75, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_118,
    118,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x390AA,
    vec![0x76, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_119,
    119,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x390C6,
    vec![0x77, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_120,
    120,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x390E2,
    vec![0x78, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_121,
    121,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x390FE,
    vec![0x79, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_122,
    122,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3911A,
    vec![0x7A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_123,
    123,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39136,
    vec![0x7B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_124,
    124,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39152,
    vec![0x7C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_125,
    125,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3916E,
    vec![0x7D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_126,
    126,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3918A,
    vec![0x7E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_127,
    127,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x391A6,
    vec![0x7F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_128,
    128,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x391C2,
    vec![0x80, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_129,
    129,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x391DE,
    vec![0x81, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_130,
    130,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x391FA,
    vec![0x82, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_131,
    131,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39216,
    vec![0x83, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_132,
    132,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39232,
    vec![0x84, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_133,
    133,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3924E,
    vec![0x85, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_134,
    134,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3926A,
    vec![0x86, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_135,
    135,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39286,
    vec![0x87, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_136,
    136,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x392A2,
    vec![0x88, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_137,
    137,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x392BE,
    vec![0x89, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_138,
    138,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x392DA,
    vec![0x8A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_139,
    139,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x392F6,
    vec![0x8B, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_140,
    140,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39312,
    vec![0x8C, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_141,
    141,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3932E,
    vec![0x8D, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_142,
    142,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3934A,
    vec![0x8E, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_143,
    143,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39366,
    vec![0x8F, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_144,
    144,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39382,
    vec![0x90, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_145,
    145,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3939E,
    vec![0x91, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_146,
    146,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x393BA,
    vec![0x92, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_147,
    147,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x393D6,
    vec![0x93, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_148,
    148,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x393F2,
    vec![0x94, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_149,
    149,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3940E,
    vec![0x95, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_150,
    150,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x3942A,
    vec![0x96, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
set_pokemon_stats_test!(
    set_states_151,
    151,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    vec![0x06, 0x07],
    0x08,
    0x09,
    0x39446,
    vec![0x97, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09]
);
