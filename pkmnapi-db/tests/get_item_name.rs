use pkmnapi_db::string::*;
use pkmnapi_db::types::*;

mod common;

macro_rules! get_item_name_test {
    ($test_name: ident, $item_id: expr, $item_name: expr) => {
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

#[rustfmt::skip::macros(get_item_name_test)]

get_item_name_test!(get_item_name_0, 0, "MASTER BALL");
get_item_name_test!(get_item_name_1, 1, "ULTRA BALL");
get_item_name_test!(get_item_name_2, 2, "GREAT BALL");
get_item_name_test!(get_item_name_3, 3, "POKé BALL");
get_item_name_test!(get_item_name_4, 4, "TOWN MAP");
get_item_name_test!(get_item_name_5, 5, "BICYCLE");
get_item_name_test!(get_item_name_6, 6, "?????");
get_item_name_test!(get_item_name_7, 7, "SAFARI BALL");
get_item_name_test!(get_item_name_8, 8, "POKéDEX");
get_item_name_test!(get_item_name_9, 9, "MOON STONE");
get_item_name_test!(get_item_name_10, 10, "ANTIDOTE");
get_item_name_test!(get_item_name_11, 11, "BURN HEAL");
get_item_name_test!(get_item_name_12, 12, "ICE HEAL");
get_item_name_test!(get_item_name_13, 13, "AWAKENING");
get_item_name_test!(get_item_name_14, 14, "PARLYZ HEAL");
get_item_name_test!(get_item_name_15, 15, "FULL RESTORE");
get_item_name_test!(get_item_name_16, 16, "MAX POTION");
get_item_name_test!(get_item_name_17, 17, "HYPER POTION");
get_item_name_test!(get_item_name_18, 18, "SUPER POTION");
get_item_name_test!(get_item_name_19, 19, "POTION");
get_item_name_test!(get_item_name_20, 20, "BOULDERBADGE");
get_item_name_test!(get_item_name_21, 21, "CASCADEBADGE");
get_item_name_test!(get_item_name_22, 22, "THUNDERBADGE");
get_item_name_test!(get_item_name_23, 23, "RAINBOWBADGE");
get_item_name_test!(get_item_name_24, 24, "SOULBADGE");
get_item_name_test!(get_item_name_25, 25, "MARSHBADGE");
get_item_name_test!(get_item_name_26, 26, "VOLCANOBADGE");
get_item_name_test!(get_item_name_27, 27, "EARTHBADGE");
get_item_name_test!(get_item_name_28, 28, "ESCAPE ROPE");
get_item_name_test!(get_item_name_29, 29, "REPEL");
get_item_name_test!(get_item_name_30, 30, "OLD AMBER");
get_item_name_test!(get_item_name_31, 31, "FIRE STONE");
get_item_name_test!(get_item_name_32, 32, "THUNDERSTONE");
get_item_name_test!(get_item_name_33, 33, "WATER STONE");
get_item_name_test!(get_item_name_34, 34, "HP UP");
get_item_name_test!(get_item_name_35, 35, "PROTEIN");
get_item_name_test!(get_item_name_36, 36, "IRON");
get_item_name_test!(get_item_name_37, 37, "CARBOS");
get_item_name_test!(get_item_name_38, 38, "CALCIUM");
get_item_name_test!(get_item_name_39, 39, "RARE CANDY");
get_item_name_test!(get_item_name_40, 40, "DOME FOSSIL");
get_item_name_test!(get_item_name_41, 41, "HELIX FOSSIL");
get_item_name_test!(get_item_name_42, 42, "SECRET KEY");
get_item_name_test!(get_item_name_43, 43, "?????");
get_item_name_test!(get_item_name_44, 44, "BIKE VOUCHER");
get_item_name_test!(get_item_name_45, 45, "X ACCURACY");
get_item_name_test!(get_item_name_46, 46, "LEAF STONE");
get_item_name_test!(get_item_name_47, 47, "CARD KEY");
get_item_name_test!(get_item_name_48, 48, "NUGGET");
get_item_name_test!(get_item_name_49, 49, "PP UP");
get_item_name_test!(get_item_name_50, 50, "POKé DOLL");
get_item_name_test!(get_item_name_51, 51, "FULL HEAL");
get_item_name_test!(get_item_name_52, 52, "REVIVE");
get_item_name_test!(get_item_name_53, 53, "MAX REVIVE");
get_item_name_test!(get_item_name_54, 54, "GUARD SPEC.");
get_item_name_test!(get_item_name_55, 55, "SUPER REPEL");
get_item_name_test!(get_item_name_56, 56, "MAX REPEL");
get_item_name_test!(get_item_name_57, 57, "DIRE HIT");
get_item_name_test!(get_item_name_58, 58, "COIN");
get_item_name_test!(get_item_name_59, 59, "FRESH WATER");
get_item_name_test!(get_item_name_60, 60, "SODA POP");
get_item_name_test!(get_item_name_61, 61, "LEMONADE");
get_item_name_test!(get_item_name_62, 62, "S.S.TICKET");
get_item_name_test!(get_item_name_63, 63, "GOLD TEETH");
get_item_name_test!(get_item_name_64, 64, "X ATTACK");
get_item_name_test!(get_item_name_65, 65, "X DEFEND");
get_item_name_test!(get_item_name_66, 66, "X SPEED");
get_item_name_test!(get_item_name_67, 67, "X SPECIAL");
get_item_name_test!(get_item_name_68, 68, "COIN CASE");
get_item_name_test!(get_item_name_69, 69, "OAK's PARCEL");
get_item_name_test!(get_item_name_70, 70, "ITEMFINDER");
get_item_name_test!(get_item_name_71, 71, "SILPH SCOPE");
get_item_name_test!(get_item_name_72, 72, "POKé FLUTE");
get_item_name_test!(get_item_name_73, 73, "LIFT KEY");
get_item_name_test!(get_item_name_74, 74, "EXP.ALL");
get_item_name_test!(get_item_name_75, 75, "OLD ROD");
get_item_name_test!(get_item_name_76, 76, "GOOD ROD");
get_item_name_test!(get_item_name_77, 77, "SUPER ROD");
get_item_name_test!(get_item_name_78, 78, "PP UP");
get_item_name_test!(get_item_name_79, 79, "ETHER");
get_item_name_test!(get_item_name_80, 80, "MAX ETHER");
get_item_name_test!(get_item_name_81, 81, "ELIXER");
get_item_name_test!(get_item_name_82, 82, "MAX ELIXER");
get_item_name_test!(get_item_name_83, 83, "B2F");
get_item_name_test!(get_item_name_84, 84, "B1F");
get_item_name_test!(get_item_name_85, 85, "1F");
get_item_name_test!(get_item_name_86, 86, "2F");
get_item_name_test!(get_item_name_87, 87, "3F");
get_item_name_test!(get_item_name_88, 88, "4F");
get_item_name_test!(get_item_name_89, 89, "5F");
get_item_name_test!(get_item_name_90, 90, "6F");
get_item_name_test!(get_item_name_91, 91, "7F");
get_item_name_test!(get_item_name_92, 92, "8F");
get_item_name_test!(get_item_name_93, 93, "9F");
get_item_name_test!(get_item_name_94, 94, "10F");
get_item_name_test!(get_item_name_95, 95, "11F");
get_item_name_test!(get_item_name_96, 96, "B4F");
