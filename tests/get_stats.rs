use pkmnapi::db::types::*;

mod common;

macro_rules! get_stats_test {
    ($test_name: ident, $pokedex_id: expr, $base_hp: expr, $base_attack: expr, $base_defence: expr, $base_speed: expr, $base_special: expr, $type_ids: expr, $catch_rate: expr, $base_exp_yield: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_stats($pokedex_id) {
                Ok(stats) => assert_eq!(
                    stats,
                    Stats {
                        pokedex_id: PokedexID::from($pokedex_id),
                        base_hp: $base_hp,
                        base_attack: $base_attack,
                        base_defence: $base_defence,
                        base_speed: $base_speed,
                        base_special: $base_special,
                        type_ids: $type_ids
                            .iter()
                            .map(|type_id| { TypeID::from(*type_id as u8) })
                            .collect(),
                        catch_rate: $catch_rate,
                        base_exp_yield: $base_exp_yield
                    },
                    "Searched for Pokédex ID: {}",
                    $pokedex_id
                ),
                Err(_) => panic!(format!("Could not find Pokédex ID: {}", $pokedex_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_stats_test)]

get_stats_test!(get_stats_1, 1, 45, 49, 49, 45, 65, vec![22, 3], 45, 64);
get_stats_test!(get_stats_2, 2, 60, 62, 63, 60, 80, vec![22, 3], 45, 141);
get_stats_test!(get_stats_3, 3, 80, 82, 83, 80, 100, vec![22, 3], 45, 208);
get_stats_test!(get_stats_4, 4, 39, 52, 43, 65, 50, vec![20, 20], 45, 65);
get_stats_test!(get_stats_5, 5, 58, 64, 58, 80, 65, vec![20, 20], 45, 142);
get_stats_test!(get_stats_6, 6, 78, 84, 78, 100, 85, vec![20, 2], 45, 209);
get_stats_test!(get_stats_7, 7, 44, 48, 65, 43, 50, vec![21, 21], 45, 66);
get_stats_test!(get_stats_8, 8, 59, 63, 80, 58, 65, vec![21, 21], 45, 143);
get_stats_test!(get_stats_9, 9, 79, 83, 100, 78, 85, vec![21, 21], 45, 210);
get_stats_test!(get_stats_10, 10, 45, 30, 35, 45, 20, vec![7, 7], 255, 53);
get_stats_test!(get_stats_11, 11, 50, 20, 55, 30, 25, vec![7, 7], 120, 72);
get_stats_test!(get_stats_12, 12, 60, 45, 50, 70, 80, vec![7, 2], 45, 160);
get_stats_test!(get_stats_13, 13, 40, 35, 30, 50, 20, vec![7, 3], 255, 52);
get_stats_test!(get_stats_14, 14, 45, 25, 50, 35, 25, vec![7, 3], 120, 71);
get_stats_test!(get_stats_15, 15, 65, 80, 40, 75, 45, vec![7, 3], 45, 159);
get_stats_test!(get_stats_16, 16, 40, 45, 40, 56, 35, vec![0, 2], 255, 55);
get_stats_test!(get_stats_17, 17, 63, 60, 55, 71, 50, vec![0, 2], 120, 113);
get_stats_test!(get_stats_18, 18, 83, 80, 75, 91, 70, vec![0, 2], 45, 172);
get_stats_test!(get_stats_19, 19, 30, 56, 35, 72, 25, vec![0, 0], 255, 57);
get_stats_test!(get_stats_20, 20, 55, 81, 60, 97, 50, vec![0, 0], 90, 116);
get_stats_test!(get_stats_21, 21, 40, 60, 30, 70, 31, vec![0, 2], 255, 58);
get_stats_test!(get_stats_22, 22, 65, 90, 65, 100, 61, vec![0, 2], 90, 162);
get_stats_test!(get_stats_23, 23, 35, 60, 44, 55, 40, vec![3, 3], 255, 62);
get_stats_test!(get_stats_24, 24, 60, 85, 69, 80, 65, vec![3, 3], 90, 147);
get_stats_test!(get_stats_25, 25, 35, 55, 30, 90, 50, vec![23, 23], 190, 82);
get_stats_test!(get_stats_26, 26, 60, 90, 55, 100, 90, vec![23, 23], 75, 122);
get_stats_test!(get_stats_27, 27, 50, 75, 85, 40, 30, vec![4, 4], 255, 93);
get_stats_test!(get_stats_28, 28, 75, 100, 110, 65, 55, vec![4, 4], 90, 163);
get_stats_test!(get_stats_29, 29, 55, 47, 52, 41, 40, vec![3, 3], 235, 59);
get_stats_test!(get_stats_30, 30, 70, 62, 67, 56, 55, vec![3, 3], 120, 117);
get_stats_test!(get_stats_31, 31, 90, 82, 87, 76, 75, vec![3, 4], 45, 194);
get_stats_test!(get_stats_32, 32, 46, 57, 40, 50, 40, vec![3, 3], 235, 60);
get_stats_test!(get_stats_33, 33, 61, 72, 57, 65, 55, vec![3, 3], 120, 118);
get_stats_test!(get_stats_34, 34, 81, 92, 77, 85, 75, vec![3, 4], 45, 195);
get_stats_test!(get_stats_35, 35, 70, 45, 48, 35, 60, vec![0, 0], 150, 68);
get_stats_test!(get_stats_36, 36, 95, 70, 73, 60, 85, vec![0, 0], 25, 129);
get_stats_test!(get_stats_37, 37, 38, 41, 40, 65, 65, vec![20, 20], 190, 63);
get_stats_test!(
    get_stats_38,
    38,
    73,
    76,
    75,
    100,
    100,
    vec![20, 20],
    75,
    178
);
get_stats_test!(get_stats_39, 39, 115, 45, 20, 20, 25, vec![0, 0], 170, 76);
get_stats_test!(get_stats_40, 40, 140, 70, 45, 45, 50, vec![0, 0], 50, 109);
get_stats_test!(get_stats_41, 41, 40, 45, 35, 55, 40, vec![3, 2], 255, 54);
get_stats_test!(get_stats_42, 42, 75, 80, 70, 90, 75, vec![3, 2], 90, 171);
get_stats_test!(get_stats_43, 43, 45, 50, 55, 30, 75, vec![22, 3], 255, 78);
get_stats_test!(get_stats_44, 44, 60, 65, 70, 40, 85, vec![22, 3], 120, 132);
get_stats_test!(get_stats_45, 45, 75, 80, 85, 50, 100, vec![22, 3], 45, 184);
get_stats_test!(get_stats_46, 46, 35, 70, 55, 25, 55, vec![7, 22], 190, 70);
get_stats_test!(get_stats_47, 47, 60, 95, 80, 30, 80, vec![7, 22], 75, 128);
get_stats_test!(get_stats_48, 48, 60, 55, 50, 45, 40, vec![7, 3], 190, 75);
get_stats_test!(get_stats_49, 49, 70, 65, 60, 90, 90, vec![7, 3], 75, 138);
get_stats_test!(get_stats_50, 50, 10, 55, 25, 95, 45, vec![4, 4], 255, 81);
get_stats_test!(get_stats_51, 51, 35, 80, 50, 120, 70, vec![4, 4], 50, 153);
get_stats_test!(get_stats_52, 52, 40, 45, 35, 90, 40, vec![0, 0], 255, 69);
get_stats_test!(get_stats_53, 53, 65, 70, 60, 115, 65, vec![0, 0], 90, 148);
get_stats_test!(get_stats_54, 54, 50, 52, 48, 55, 50, vec![21, 21], 190, 80);
get_stats_test!(get_stats_55, 55, 80, 82, 78, 85, 80, vec![21, 21], 75, 174);
get_stats_test!(get_stats_56, 56, 40, 80, 35, 70, 35, vec![1, 1], 190, 74);
get_stats_test!(get_stats_57, 57, 65, 105, 60, 95, 60, vec![1, 1], 75, 149);
get_stats_test!(get_stats_58, 58, 55, 70, 45, 60, 50, vec![20, 20], 190, 91);
get_stats_test!(get_stats_59, 59, 90, 110, 80, 95, 80, vec![20, 20], 75, 213);
get_stats_test!(get_stats_60, 60, 40, 50, 40, 90, 40, vec![21, 21], 255, 77);
get_stats_test!(get_stats_61, 61, 65, 65, 65, 90, 50, vec![21, 21], 120, 131);
get_stats_test!(get_stats_62, 62, 90, 85, 95, 70, 70, vec![21, 1], 45, 185);
get_stats_test!(get_stats_63, 63, 25, 20, 15, 90, 105, vec![24, 24], 200, 73);
get_stats_test!(
    get_stats_64,
    64,
    40,
    35,
    30,
    105,
    120,
    vec![24, 24],
    100,
    145
);
get_stats_test!(
    get_stats_65,
    65,
    55,
    50,
    45,
    120,
    135,
    vec![24, 24],
    50,
    186
);
get_stats_test!(get_stats_66, 66, 70, 80, 50, 35, 35, vec![1, 1], 180, 88);
get_stats_test!(get_stats_67, 67, 80, 100, 70, 45, 50, vec![1, 1], 90, 146);
get_stats_test!(get_stats_68, 68, 90, 130, 80, 55, 65, vec![1, 1], 45, 193);
get_stats_test!(get_stats_69, 69, 50, 75, 35, 40, 70, vec![22, 3], 255, 84);
get_stats_test!(get_stats_70, 70, 65, 90, 50, 55, 85, vec![22, 3], 120, 151);
get_stats_test!(get_stats_71, 71, 80, 105, 65, 70, 100, vec![22, 3], 45, 191);
get_stats_test!(get_stats_72, 72, 40, 40, 35, 70, 100, vec![21, 3], 190, 105);
get_stats_test!(get_stats_73, 73, 80, 70, 65, 100, 120, vec![21, 3], 60, 205);
get_stats_test!(get_stats_74, 74, 40, 80, 100, 20, 30, vec![5, 4], 255, 86);
get_stats_test!(get_stats_75, 75, 55, 95, 115, 35, 45, vec![5, 4], 120, 134);
get_stats_test!(get_stats_76, 76, 80, 110, 130, 45, 55, vec![5, 4], 45, 177);
get_stats_test!(get_stats_77, 77, 50, 85, 55, 90, 65, vec![20, 20], 190, 152);
get_stats_test!(
    get_stats_78,
    78,
    65,
    100,
    70,
    105,
    80,
    vec![20, 20],
    60,
    192
);
get_stats_test!(get_stats_79, 79, 90, 65, 65, 15, 40, vec![21, 24], 190, 99);
get_stats_test!(get_stats_80, 80, 95, 75, 110, 30, 80, vec![21, 24], 75, 164);
get_stats_test!(get_stats_81, 81, 25, 35, 70, 45, 95, vec![23, 23], 190, 89);
get_stats_test!(get_stats_82, 82, 50, 60, 95, 70, 120, vec![23, 23], 60, 161);
get_stats_test!(get_stats_83, 83, 52, 65, 55, 60, 58, vec![0, 2], 45, 94);
get_stats_test!(get_stats_84, 84, 35, 85, 45, 75, 35, vec![0, 2], 190, 96);
get_stats_test!(get_stats_85, 85, 60, 110, 70, 100, 60, vec![0, 2], 45, 158);
get_stats_test!(get_stats_86, 86, 65, 45, 55, 45, 70, vec![21, 21], 190, 100);
get_stats_test!(get_stats_87, 87, 90, 70, 80, 70, 95, vec![21, 25], 75, 176);
get_stats_test!(get_stats_88, 88, 80, 80, 50, 25, 40, vec![3, 3], 190, 90);
get_stats_test!(get_stats_89, 89, 105, 105, 75, 50, 65, vec![3, 3], 75, 157);
get_stats_test!(get_stats_90, 90, 30, 65, 100, 40, 45, vec![21, 21], 190, 97);
get_stats_test!(get_stats_91, 91, 50, 95, 180, 70, 85, vec![21, 25], 60, 203);
get_stats_test!(get_stats_92, 92, 30, 35, 30, 80, 100, vec![8, 3], 190, 95);
get_stats_test!(get_stats_93, 93, 45, 50, 45, 95, 115, vec![8, 3], 90, 126);
get_stats_test!(get_stats_94, 94, 60, 65, 60, 110, 130, vec![8, 3], 45, 190);
get_stats_test!(get_stats_95, 95, 35, 45, 160, 70, 30, vec![5, 4], 45, 108);
get_stats_test!(get_stats_96, 96, 60, 48, 45, 42, 90, vec![24, 24], 190, 102);
get_stats_test!(get_stats_97, 97, 85, 73, 70, 67, 115, vec![24, 24], 75, 165);
get_stats_test!(
    get_stats_98,
    98,
    30,
    105,
    90,
    50,
    25,
    vec![21, 21],
    225,
    115
);
get_stats_test!(
    get_stats_99,
    99,
    55,
    130,
    115,
    75,
    50,
    vec![21, 21],
    60,
    206
);
get_stats_test!(
    get_stats_100,
    100,
    40,
    30,
    50,
    100,
    55,
    vec![23, 23],
    190,
    103
);
get_stats_test!(
    get_stats_101,
    101,
    60,
    50,
    70,
    140,
    80,
    vec![23, 23],
    60,
    150
);
get_stats_test!(get_stats_102, 102, 60, 40, 80, 40, 60, vec![22, 24], 90, 98);
get_stats_test!(
    get_stats_103,
    103,
    95,
    95,
    85,
    55,
    125,
    vec![22, 24],
    45,
    212
);
get_stats_test!(get_stats_104, 104, 50, 50, 95, 35, 40, vec![4, 4], 190, 87);
get_stats_test!(get_stats_105, 105, 60, 80, 110, 45, 50, vec![4, 4], 75, 124);
get_stats_test!(get_stats_106, 106, 50, 120, 53, 87, 35, vec![1, 1], 45, 139);
get_stats_test!(get_stats_107, 107, 50, 105, 79, 76, 35, vec![1, 1], 45, 140);
get_stats_test!(get_stats_108, 108, 90, 55, 75, 30, 60, vec![0, 0], 45, 127);
get_stats_test!(get_stats_109, 109, 40, 65, 95, 35, 60, vec![3, 3], 190, 114);
get_stats_test!(get_stats_110, 110, 65, 90, 120, 60, 85, vec![3, 3], 60, 173);
get_stats_test!(get_stats_111, 111, 80, 85, 95, 25, 30, vec![4, 5], 120, 135);
get_stats_test!(
    get_stats_112,
    112,
    105,
    130,
    120,
    40,
    45,
    vec![4, 5],
    60,
    204
);
get_stats_test!(get_stats_113, 113, 250, 5, 5, 50, 105, vec![0, 0], 30, 255);
get_stats_test!(
    get_stats_114,
    114,
    65,
    55,
    115,
    60,
    100,
    vec![22, 22],
    45,
    166
);
get_stats_test!(get_stats_115, 115, 105, 95, 80, 90, 40, vec![0, 0], 45, 175);
get_stats_test!(
    get_stats_116,
    116,
    30,
    40,
    70,
    60,
    70,
    vec![21, 21],
    225,
    83
);
get_stats_test!(
    get_stats_117,
    117,
    55,
    65,
    95,
    85,
    95,
    vec![21, 21],
    75,
    155
);
get_stats_test!(
    get_stats_118,
    118,
    45,
    67,
    60,
    63,
    50,
    vec![21, 21],
    225,
    111
);
get_stats_test!(
    get_stats_119,
    119,
    80,
    92,
    65,
    68,
    80,
    vec![21, 21],
    60,
    170
);
get_stats_test!(
    get_stats_120,
    120,
    30,
    45,
    55,
    85,
    70,
    vec![21, 21],
    225,
    106
);
get_stats_test!(
    get_stats_121,
    121,
    60,
    75,
    85,
    115,
    100,
    vec![21, 24],
    60,
    207
);
get_stats_test!(
    get_stats_122,
    122,
    40,
    45,
    65,
    90,
    100,
    vec![24, 24],
    45,
    136
);
get_stats_test!(
    get_stats_123,
    123,
    70,
    110,
    80,
    105,
    55,
    vec![7, 2],
    45,
    187
);
get_stats_test!(
    get_stats_124,
    124,
    65,
    50,
    35,
    95,
    95,
    vec![25, 24],
    45,
    137
);
get_stats_test!(
    get_stats_125,
    125,
    65,
    83,
    57,
    105,
    85,
    vec![23, 23],
    45,
    156
);
get_stats_test!(
    get_stats_126,
    126,
    65,
    95,
    57,
    93,
    85,
    vec![20, 20],
    45,
    167
);
get_stats_test!(
    get_stats_127,
    127,
    65,
    125,
    100,
    85,
    55,
    vec![7, 7],
    45,
    200
);
get_stats_test!(
    get_stats_128,
    128,
    75,
    100,
    95,
    110,
    70,
    vec![0, 0],
    45,
    211
);
get_stats_test!(
    get_stats_129,
    129,
    20,
    10,
    55,
    80,
    20,
    vec![21, 21],
    255,
    20
);
get_stats_test!(
    get_stats_130,
    130,
    95,
    125,
    79,
    81,
    100,
    vec![21, 2],
    45,
    214
);
get_stats_test!(
    get_stats_131,
    131,
    130,
    85,
    80,
    60,
    95,
    vec![21, 25],
    45,
    219
);
get_stats_test!(get_stats_132, 132, 48, 48, 48, 48, 48, vec![0, 0], 35, 61);
get_stats_test!(get_stats_133, 133, 55, 55, 50, 55, 65, vec![0, 0], 45, 92);
get_stats_test!(
    get_stats_134,
    134,
    130,
    65,
    60,
    65,
    110,
    vec![21, 21],
    45,
    196
);
get_stats_test!(
    get_stats_135,
    135,
    65,
    65,
    60,
    130,
    110,
    vec![23, 23],
    45,
    197
);
get_stats_test!(
    get_stats_136,
    136,
    65,
    130,
    60,
    65,
    110,
    vec![20, 20],
    45,
    198
);
get_stats_test!(get_stats_137, 137, 65, 60, 70, 40, 75, vec![0, 0], 45, 130);
get_stats_test!(
    get_stats_138,
    138,
    35,
    40,
    100,
    35,
    90,
    vec![5, 21],
    45,
    120
);
get_stats_test!(
    get_stats_139,
    139,
    70,
    60,
    125,
    55,
    115,
    vec![5, 21],
    45,
    199
);
get_stats_test!(get_stats_140, 140, 30, 80, 90, 55, 45, vec![5, 21], 45, 119);
get_stats_test!(
    get_stats_141,
    141,
    60,
    115,
    105,
    80,
    70,
    vec![5, 21],
    45,
    201
);
get_stats_test!(
    get_stats_142,
    142,
    80,
    105,
    65,
    130,
    60,
    vec![5, 2],
    45,
    202
);
get_stats_test!(
    get_stats_143,
    143,
    160,
    110,
    65,
    30,
    65,
    vec![0, 0],
    25,
    154
);
get_stats_test!(
    get_stats_144,
    144,
    90,
    85,
    100,
    85,
    125,
    vec![25, 2],
    3,
    215
);
get_stats_test!(
    get_stats_145,
    145,
    90,
    90,
    85,
    100,
    125,
    vec![23, 2],
    3,
    216
);
get_stats_test!(
    get_stats_146,
    146,
    90,
    100,
    90,
    90,
    125,
    vec![20, 2],
    3,
    217
);
get_stats_test!(get_stats_147, 147, 41, 64, 45, 50, 50, vec![26, 26], 45, 67);
get_stats_test!(
    get_stats_148,
    148,
    61,
    84,
    65,
    70,
    70,
    vec![26, 26],
    45,
    144
);
get_stats_test!(
    get_stats_149,
    149,
    91,
    134,
    95,
    80,
    100,
    vec![26, 2],
    45,
    218
);
get_stats_test!(
    get_stats_150,
    150,
    106,
    110,
    90,
    130,
    154,
    vec![24, 24],
    3,
    220
);
get_stats_test!(
    get_stats_151,
    151,
    100,
    100,
    100,
    100,
    100,
    vec![24, 24],
    45,
    64
);
