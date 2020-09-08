table! {
    patches (id) {
        id -> Text,
        user_id -> Text,
        data -> Binary,
        description -> Nullable<Text>,
    }
}

table! {
    rom_data (id) {
        id -> Text,
        name -> Text,
        data -> Binary,
    }
}

table! {
    roms (id) {
        id -> Text,
        name -> Text,
        rom_data_id -> Text,
    }
}

table! {
    users (id) {
        id -> Text,
        date_create -> Text,
        date_expire -> Text,
        access_token_hash -> Text,
        rom_id -> Nullable<Text>,
    }
}

joinable!(patches -> users (user_id));
joinable!(roms -> rom_data (rom_data_id));
joinable!(users -> roms (rom_id));

allow_tables_to_appear_in_same_query!(
    patches,
    rom_data,
    roms,
    users,
);
