use pkmnapi_db::*;

mod common;

macro_rules! get_trade_test {
    ($test_name:ident, $trade_id:expr, $give_pokedex_id:expr, $get_pokedex_id:expr, $nickname:expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_trade(&$trade_id) {
                Ok(trade) => assert_eq!(
                    trade,
                    Trade::new($give_pokedex_id, $get_pokedex_id, $nickname),
                    "Searched for trade ID: {}",
                    $trade_id
                ),
                Err(_) => panic!(format!("Could not find trade ID: {}", $trade_id)),
            };
        }
    };
}

get_trade_test!(get_trade_0, 0, 33, 30, "TERRY");
get_trade_test!(get_trade_1, 1, 63, 122, "MARCEL");
get_trade_test!(get_trade_2, 2, 12, 15, "CHIKUCHIKU");
get_trade_test!(get_trade_3, 3, 77, 86, "SAILOR");
get_trade_test!(get_trade_4, 4, 21, 83, "DUX");
get_trade_test!(get_trade_5, 5, 80, 108, "MARC");
get_trade_test!(get_trade_6, 6, 61, 124, "LOLA");
get_trade_test!(get_trade_7, 7, 26, 101, "DORIS");
get_trade_test!(get_trade_8, 8, 48, 114, "CRINKLES");
get_trade_test!(get_trade_9, 9, 32, 29, "SPOT");
