use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! get_move_name_test {
    ($test_name: ident, $move_id: expr, $move_name: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_move_name(&$move_id) {
                Ok(move_name) => assert_eq!(
                    move_name,
                    MoveName {
                        name: ROMString::from($move_name)
                    },
                    "Searched for move ID: {}",
                    $move_id
                ),
                Err(_) => panic!(format!("Could not find move ID: {}", $move_id)),
            };
        }
    };
}

#[rustfmt::skip::macros(get_move_name_test)]

get_move_name_test!(get_move_name_1, 1, "POUND");
get_move_name_test!(get_move_name_2, 2, "KARATE CHOP");
get_move_name_test!(get_move_name_3, 3, "DOUBLESLAP");
get_move_name_test!(get_move_name_4, 4, "COMET PUNCH");
get_move_name_test!(get_move_name_5, 5, "MEGA PUNCH");
get_move_name_test!(get_move_name_6, 6, "PAY DAY");
get_move_name_test!(get_move_name_7, 7, "FIRE PUNCH");
get_move_name_test!(get_move_name_8, 8, "ICE PUNCH");
get_move_name_test!(get_move_name_9, 9, "THUNDERPUNCH");
get_move_name_test!(get_move_name_10, 10, "SCRATCH");
get_move_name_test!(get_move_name_11, 11, "VICEGRIP");
get_move_name_test!(get_move_name_12, 12, "GUILLOTINE");
get_move_name_test!(get_move_name_13, 13, "RAZOR WIND");
get_move_name_test!(get_move_name_14, 14, "SWORDS DANCE");
get_move_name_test!(get_move_name_15, 15, "CUT");
get_move_name_test!(get_move_name_16, 16, "GUST");
get_move_name_test!(get_move_name_17, 17, "WING ATTACK");
get_move_name_test!(get_move_name_18, 18, "WHIRLWIND");
get_move_name_test!(get_move_name_19, 19, "FLY");
get_move_name_test!(get_move_name_20, 20, "BIND");
get_move_name_test!(get_move_name_21, 21, "SLAM");
get_move_name_test!(get_move_name_22, 22, "VINE WHIP");
get_move_name_test!(get_move_name_23, 23, "STOMP");
get_move_name_test!(get_move_name_24, 24, "DOUBLE KICK");
get_move_name_test!(get_move_name_25, 25, "MEGA KICK");
get_move_name_test!(get_move_name_26, 26, "JUMP KICK");
get_move_name_test!(get_move_name_27, 27, "ROLLING KICK");
get_move_name_test!(get_move_name_28, 28, "SAND-ATTACK");
get_move_name_test!(get_move_name_29, 29, "HEADBUTT");
get_move_name_test!(get_move_name_30, 30, "HORN ATTACK");
get_move_name_test!(get_move_name_31, 31, "FURY ATTACK");
get_move_name_test!(get_move_name_32, 32, "HORN DRILL");
get_move_name_test!(get_move_name_33, 33, "TACKLE");
get_move_name_test!(get_move_name_34, 34, "BODY SLAM");
get_move_name_test!(get_move_name_35, 35, "WRAP");
get_move_name_test!(get_move_name_36, 36, "TAKE DOWN");
get_move_name_test!(get_move_name_37, 37, "THRASH");
get_move_name_test!(get_move_name_38, 38, "DOUBLE-EDGE");
get_move_name_test!(get_move_name_39, 39, "TAIL WHIP");
get_move_name_test!(get_move_name_40, 40, "POISON STING");
get_move_name_test!(get_move_name_41, 41, "TWINEEDLE");
get_move_name_test!(get_move_name_42, 42, "PIN MISSILE");
get_move_name_test!(get_move_name_43, 43, "LEER");
get_move_name_test!(get_move_name_44, 44, "BITE");
get_move_name_test!(get_move_name_45, 45, "GROWL");
get_move_name_test!(get_move_name_46, 46, "ROAR");
get_move_name_test!(get_move_name_47, 47, "SING");
get_move_name_test!(get_move_name_48, 48, "SUPERSONIC");
get_move_name_test!(get_move_name_49, 49, "SONICBOOM");
get_move_name_test!(get_move_name_50, 50, "DISABLE");
get_move_name_test!(get_move_name_51, 51, "ACID");
get_move_name_test!(get_move_name_52, 52, "EMBER");
get_move_name_test!(get_move_name_53, 53, "FLAMETHROWER");
get_move_name_test!(get_move_name_54, 54, "MIST");
get_move_name_test!(get_move_name_55, 55, "WATER GUN");
get_move_name_test!(get_move_name_56, 56, "HYDRO PUMP");
get_move_name_test!(get_move_name_57, 57, "SURF");
get_move_name_test!(get_move_name_58, 58, "ICE BEAM");
get_move_name_test!(get_move_name_59, 59, "BLIZZARD");
get_move_name_test!(get_move_name_60, 60, "PSYBEAM");
get_move_name_test!(get_move_name_61, 61, "BUBBLEBEAM");
get_move_name_test!(get_move_name_62, 62, "AURORA BEAM");
get_move_name_test!(get_move_name_63, 63, "HYPER BEAM");
get_move_name_test!(get_move_name_64, 64, "PECK");
get_move_name_test!(get_move_name_65, 65, "DRILL PECK");
get_move_name_test!(get_move_name_66, 66, "SUBMISSION");
get_move_name_test!(get_move_name_67, 67, "LOW KICK");
get_move_name_test!(get_move_name_68, 68, "COUNTER");
get_move_name_test!(get_move_name_69, 69, "SEISMIC TOSS");
get_move_name_test!(get_move_name_70, 70, "STRENGTH");
get_move_name_test!(get_move_name_71, 71, "ABSORB");
get_move_name_test!(get_move_name_72, 72, "MEGA DRAIN");
get_move_name_test!(get_move_name_73, 73, "LEECH SEED");
get_move_name_test!(get_move_name_74, 74, "GROWTH");
get_move_name_test!(get_move_name_75, 75, "RAZOR LEAF");
get_move_name_test!(get_move_name_76, 76, "SOLARBEAM");
get_move_name_test!(get_move_name_77, 77, "POISONPOWDER");
get_move_name_test!(get_move_name_78, 78, "STUN SPORE");
get_move_name_test!(get_move_name_79, 79, "SLEEP POWDER");
get_move_name_test!(get_move_name_80, 80, "PETAL DANCE");
get_move_name_test!(get_move_name_81, 81, "STRING SHOT");
get_move_name_test!(get_move_name_82, 82, "DRAGON RAGE");
get_move_name_test!(get_move_name_83, 83, "FIRE SPIN");
get_move_name_test!(get_move_name_84, 84, "THUNDERSHOCK");
get_move_name_test!(get_move_name_85, 85, "THUNDERBOLT");
get_move_name_test!(get_move_name_86, 86, "THUNDER WAVE");
get_move_name_test!(get_move_name_87, 87, "THUNDER");
get_move_name_test!(get_move_name_88, 88, "ROCK THROW");
get_move_name_test!(get_move_name_89, 89, "EARTHQUAKE");
get_move_name_test!(get_move_name_90, 90, "FISSURE");
get_move_name_test!(get_move_name_91, 91, "DIG");
get_move_name_test!(get_move_name_92, 92, "TOXIC");
get_move_name_test!(get_move_name_93, 93, "CONFUSION");
get_move_name_test!(get_move_name_94, 94, "PSYCHIC");
get_move_name_test!(get_move_name_95, 95, "HYPNOSIS");
get_move_name_test!(get_move_name_96, 96, "MEDITATE");
get_move_name_test!(get_move_name_97, 97, "AGILITY");
get_move_name_test!(get_move_name_98, 98, "QUICK ATTACK");
get_move_name_test!(get_move_name_99, 99, "RAGE");
get_move_name_test!(get_move_name_100, 100, "TELEPORT");
get_move_name_test!(get_move_name_101, 101, "NIGHT SHADE");
get_move_name_test!(get_move_name_102, 102, "MIMIC");
get_move_name_test!(get_move_name_103, 103, "SCREECH");
get_move_name_test!(get_move_name_104, 104, "DOUBLE TEAM");
get_move_name_test!(get_move_name_105, 105, "RECOVER");
get_move_name_test!(get_move_name_106, 106, "HARDEN");
get_move_name_test!(get_move_name_107, 107, "MINIMIZE");
get_move_name_test!(get_move_name_108, 108, "SMOKESCREEN");
get_move_name_test!(get_move_name_109, 109, "CONFUSE RAY");
get_move_name_test!(get_move_name_110, 110, "WITHDRAW");
get_move_name_test!(get_move_name_111, 111, "DEFENSE CURL");
get_move_name_test!(get_move_name_112, 112, "BARRIER");
get_move_name_test!(get_move_name_113, 113, "LIGHT SCREEN");
get_move_name_test!(get_move_name_114, 114, "HAZE");
get_move_name_test!(get_move_name_115, 115, "REFLECT");
get_move_name_test!(get_move_name_116, 116, "FOCUS ENERGY");
get_move_name_test!(get_move_name_117, 117, "BIDE");
get_move_name_test!(get_move_name_118, 118, "METRONOME");
get_move_name_test!(get_move_name_119, 119, "MIRROR MOVE");
get_move_name_test!(get_move_name_120, 120, "SELFDESTRUCT");
get_move_name_test!(get_move_name_121, 121, "EGG BOMB");
get_move_name_test!(get_move_name_122, 122, "LICK");
get_move_name_test!(get_move_name_123, 123, "SMOG");
get_move_name_test!(get_move_name_124, 124, "SLUDGE");
get_move_name_test!(get_move_name_125, 125, "BONE CLUB");
get_move_name_test!(get_move_name_126, 126, "FIRE BLAST");
get_move_name_test!(get_move_name_127, 127, "WATERFALL");
get_move_name_test!(get_move_name_128, 128, "CLAMP");
get_move_name_test!(get_move_name_129, 129, "SWIFT");
get_move_name_test!(get_move_name_130, 130, "SKULL BASH");
get_move_name_test!(get_move_name_131, 131, "SPIKE CANNON");
get_move_name_test!(get_move_name_132, 132, "CONSTRICT");
get_move_name_test!(get_move_name_133, 133, "AMNESIA");
get_move_name_test!(get_move_name_134, 134, "KINESIS");
get_move_name_test!(get_move_name_135, 135, "SOFTBOILED");
get_move_name_test!(get_move_name_136, 136, "HI JUMP KICK");
get_move_name_test!(get_move_name_137, 137, "GLARE");
get_move_name_test!(get_move_name_138, 138, "DREAM EATER");
get_move_name_test!(get_move_name_139, 139, "POISON GAS");
get_move_name_test!(get_move_name_140, 140, "BARRAGE");
get_move_name_test!(get_move_name_141, 141, "LEECH LIFE");
get_move_name_test!(get_move_name_142, 142, "LOVELY KISS");
get_move_name_test!(get_move_name_143, 143, "SKY ATTACK");
get_move_name_test!(get_move_name_144, 144, "TRANSFORM");
get_move_name_test!(get_move_name_145, 145, "BUBBLE");
get_move_name_test!(get_move_name_146, 146, "DIZZY PUNCH");
get_move_name_test!(get_move_name_147, 147, "SPORE");
get_move_name_test!(get_move_name_148, 148, "FLASH");
get_move_name_test!(get_move_name_149, 149, "PSYWAVE");
get_move_name_test!(get_move_name_150, 150, "SPLASH");
get_move_name_test!(get_move_name_151, 151, "ACID ARMOR");
get_move_name_test!(get_move_name_152, 152, "CRABHAMMER");
get_move_name_test!(get_move_name_153, 153, "EXPLOSION");
get_move_name_test!(get_move_name_154, 154, "FURY SWIPES");
get_move_name_test!(get_move_name_155, 155, "BONEMERANG");
get_move_name_test!(get_move_name_156, 156, "REST");
get_move_name_test!(get_move_name_157, 157, "ROCK SLIDE");
get_move_name_test!(get_move_name_158, 158, "HYPER FANG");
get_move_name_test!(get_move_name_159, 159, "SHARPEN");
get_move_name_test!(get_move_name_160, 160, "CONVERSION");
get_move_name_test!(get_move_name_161, 161, "TRI ATTACK");
get_move_name_test!(get_move_name_162, 162, "SUPER FANG");
get_move_name_test!(get_move_name_163, 163, "SLASH");
get_move_name_test!(get_move_name_164, 164, "SUBSTITUTE");
get_move_name_test!(get_move_name_165, 165, "STRUGGLE");
