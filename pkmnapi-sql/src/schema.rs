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
        patch -> Nullable<Binary>,
        rom_id -> Nullable<Text>,
    }
}

joinable!(roms -> rom_data (rom_data_id));
joinable!(users -> roms (rom_id));

allow_tables_to_appear_in_same_query!(rom_data, roms, users,);
