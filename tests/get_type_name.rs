macro_rules! get_type_name_test {
    ($test_name: ident, $type_id: expr, $type_name: expr) => {
        #[test]
        #[ignore]
        #[allow(non_snake_case)]
        fn $test_name() {
            let db = common::load_rom();

            match db.get_type_name($type_id) {
                Ok(type_name) => assert_eq!(
                    type_name,
                    PkmnapiDBTypeName {
                        name: PkmnapiDBString::from($type_name)
                    },
                    "Searched for type ID: {}",
                    $type_id
                ),
                Err(_) => panic!(format!("Could not find type ID: {}", $type_id)),
            };
        }
    };
}

#[cfg(test)]
#[rustfmt::skip::macros(get_type_name_test)]
mod tests {
    use pkmnapi::db::string::*;
    use pkmnapi::db::types::*;

    mod common;

    get_type_name_test!(get_type_name_0, 0, "NORMAL");
    get_type_name_test!(get_type_name_1, 1, "FIGHTING");
    get_type_name_test!(get_type_name_2, 2, "FLYING");
    get_type_name_test!(get_type_name_3, 3, "POISON");
    get_type_name_test!(get_type_name_4, 4, "GROUND");
    get_type_name_test!(get_type_name_5, 5, "ROCK");
    get_type_name_test!(get_type_name_6, 6, "BIRD");
    get_type_name_test!(get_type_name_7, 7, "BUG");
    get_type_name_test!(get_type_name_8, 8, "GHOST");
    get_type_name_test!(get_type_name_9, 9, "NORMAL");
    get_type_name_test!(get_type_name_10, 10, "NORMAL");
    get_type_name_test!(get_type_name_11, 11, "NORMAL");
    get_type_name_test!(get_type_name_12, 12, "NORMAL");
    get_type_name_test!(get_type_name_13, 13, "NORMAL");
    get_type_name_test!(get_type_name_14, 14, "NORMAL");
    get_type_name_test!(get_type_name_15, 15, "NORMAL");
    get_type_name_test!(get_type_name_16, 16, "NORMAL");
    get_type_name_test!(get_type_name_17, 17, "NORMAL");
    get_type_name_test!(get_type_name_18, 18, "NORMAL");
    get_type_name_test!(get_type_name_19, 19, "NORMAL");
    get_type_name_test!(get_type_name_20, 20, "FIRE");
    get_type_name_test!(get_type_name_21, 21, "WATER");
    get_type_name_test!(get_type_name_22, 22, "GRASS");
    get_type_name_test!(get_type_name_23, 23, "ELECTRIC");
    get_type_name_test!(get_type_name_24, 24, "PSYCHIC");
    get_type_name_test!(get_type_name_25, 25, "ICE");
    get_type_name_test!(get_type_name_26, 26, "DRAGON");
}
