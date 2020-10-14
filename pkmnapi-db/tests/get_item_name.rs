use pkmnapi_db::string::*;
use pkmnapi_db::*;

mod common;

macro_rules! get_item_name_test {
    ($test_name:ident, $item_id:expr, $item_name:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_item_name(&$item_id) {
                Ok(item_name) => assert_eq!(
                    item_name,
                    ItemName {
                        name: ROMString::from($item_name)
                    },
                    "Searched for item ID: {}",
                    $item_id
                ),
                Err(_) => panic!(format!("Could not find item ID: {}", $item_id)),
            };
        }
    };
}

get_item_name_test!(get_item_name_1, 1, "MASTER BALL");
get_item_name_test!(get_item_name_2, 2, "ULTRA BALL");
get_item_name_test!(get_item_name_3, 3, "GREAT BALL");
get_item_name_test!(get_item_name_4, 4, "POKé BALL");
get_item_name_test!(get_item_name_5, 5, "TOWN MAP");
get_item_name_test!(get_item_name_6, 6, "BICYCLE");
get_item_name_test!(get_item_name_7, 7, "?????");
get_item_name_test!(get_item_name_8, 8, "SAFARI BALL");
get_item_name_test!(get_item_name_9, 9, "POKéDEX");
get_item_name_test!(get_item_name_10, 10, "MOON STONE");
get_item_name_test!(get_item_name_11, 11, "ANTIDOTE");
get_item_name_test!(get_item_name_12, 12, "BURN HEAL");
get_item_name_test!(get_item_name_13, 13, "ICE HEAL");
get_item_name_test!(get_item_name_14, 14, "AWAKENING");
get_item_name_test!(get_item_name_15, 15, "PARLYZ HEAL");
get_item_name_test!(get_item_name_16, 16, "FULL RESTORE");
get_item_name_test!(get_item_name_17, 17, "MAX POTION");
get_item_name_test!(get_item_name_18, 18, "HYPER POTION");
get_item_name_test!(get_item_name_19, 19, "SUPER POTION");
get_item_name_test!(get_item_name_20, 20, "POTION");
get_item_name_test!(get_item_name_21, 21, "BOULDERBADGE");
get_item_name_test!(get_item_name_22, 22, "CASCADEBADGE");
get_item_name_test!(get_item_name_23, 23, "THUNDERBADGE");
get_item_name_test!(get_item_name_24, 24, "RAINBOWBADGE");
get_item_name_test!(get_item_name_25, 25, "SOULBADGE");
get_item_name_test!(get_item_name_26, 26, "MARSHBADGE");
get_item_name_test!(get_item_name_27, 27, "VOLCANOBADGE");
get_item_name_test!(get_item_name_28, 28, "EARTHBADGE");
get_item_name_test!(get_item_name_29, 29, "ESCAPE ROPE");
get_item_name_test!(get_item_name_30, 30, "REPEL");
get_item_name_test!(get_item_name_31, 31, "OLD AMBER");
get_item_name_test!(get_item_name_32, 32, "FIRE STONE");
get_item_name_test!(get_item_name_33, 33, "THUNDERSTONE");
get_item_name_test!(get_item_name_34, 34, "WATER STONE");
get_item_name_test!(get_item_name_35, 35, "HP UP");
get_item_name_test!(get_item_name_36, 36, "PROTEIN");
get_item_name_test!(get_item_name_37, 37, "IRON");
get_item_name_test!(get_item_name_38, 38, "CARBOS");
get_item_name_test!(get_item_name_39, 39, "CALCIUM");
get_item_name_test!(get_item_name_40, 40, "RARE CANDY");
get_item_name_test!(get_item_name_41, 41, "DOME FOSSIL");
get_item_name_test!(get_item_name_42, 42, "HELIX FOSSIL");
get_item_name_test!(get_item_name_43, 43, "SECRET KEY");
get_item_name_test!(get_item_name_44, 44, "?????");
get_item_name_test!(get_item_name_45, 45, "BIKE VOUCHER");
get_item_name_test!(get_item_name_46, 46, "X ACCURACY");
get_item_name_test!(get_item_name_47, 47, "LEAF STONE");
get_item_name_test!(get_item_name_48, 48, "CARD KEY");
get_item_name_test!(get_item_name_49, 49, "NUGGET");
get_item_name_test!(get_item_name_50, 50, "PP UP");
get_item_name_test!(get_item_name_51, 51, "POKé DOLL");
get_item_name_test!(get_item_name_52, 52, "FULL HEAL");
get_item_name_test!(get_item_name_53, 53, "REVIVE");
get_item_name_test!(get_item_name_54, 54, "MAX REVIVE");
get_item_name_test!(get_item_name_55, 55, "GUARD SPEC.");
get_item_name_test!(get_item_name_56, 56, "SUPER REPEL");
get_item_name_test!(get_item_name_57, 57, "MAX REPEL");
get_item_name_test!(get_item_name_58, 58, "DIRE HIT");
get_item_name_test!(get_item_name_59, 59, "COIN");
get_item_name_test!(get_item_name_60, 60, "FRESH WATER");
get_item_name_test!(get_item_name_61, 61, "SODA POP");
get_item_name_test!(get_item_name_62, 62, "LEMONADE");
get_item_name_test!(get_item_name_63, 63, "S.S.TICKET");
get_item_name_test!(get_item_name_64, 64, "GOLD TEETH");
get_item_name_test!(get_item_name_65, 65, "X ATTACK");
get_item_name_test!(get_item_name_66, 66, "X DEFEND");
get_item_name_test!(get_item_name_67, 67, "X SPEED");
get_item_name_test!(get_item_name_68, 68, "X SPECIAL");
get_item_name_test!(get_item_name_69, 69, "COIN CASE");
get_item_name_test!(get_item_name_70, 70, "OAK's PARCEL");
get_item_name_test!(get_item_name_71, 71, "ITEMFINDER");
get_item_name_test!(get_item_name_72, 72, "SILPH SCOPE");
get_item_name_test!(get_item_name_73, 73, "POKé FLUTE");
get_item_name_test!(get_item_name_74, 74, "LIFT KEY");
get_item_name_test!(get_item_name_75, 75, "EXP.ALL");
get_item_name_test!(get_item_name_76, 76, "OLD ROD");
get_item_name_test!(get_item_name_77, 77, "GOOD ROD");
get_item_name_test!(get_item_name_78, 78, "SUPER ROD");
get_item_name_test!(get_item_name_79, 79, "PP UP");
get_item_name_test!(get_item_name_80, 80, "ETHER");
get_item_name_test!(get_item_name_81, 81, "MAX ETHER");
get_item_name_test!(get_item_name_82, 82, "ELIXER");
get_item_name_test!(get_item_name_83, 83, "MAX ELIXER");
get_item_name_test!(get_item_name_84, 84, "B2F");
get_item_name_test!(get_item_name_85, 85, "B1F");
get_item_name_test!(get_item_name_86, 86, "1F");
get_item_name_test!(get_item_name_87, 87, "2F");
get_item_name_test!(get_item_name_88, 88, "3F");
get_item_name_test!(get_item_name_89, 89, "4F");
get_item_name_test!(get_item_name_90, 90, "5F");
get_item_name_test!(get_item_name_91, 91, "6F");
get_item_name_test!(get_item_name_92, 92, "7F");
get_item_name_test!(get_item_name_93, 93, "8F");
get_item_name_test!(get_item_name_94, 94, "9F");
get_item_name_test!(get_item_name_95, 95, "10F");
get_item_name_test!(get_item_name_96, 96, "11F");
get_item_name_test!(get_item_name_97, 97, "B4F");
