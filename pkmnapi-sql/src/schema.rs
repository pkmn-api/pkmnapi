table! {
    rom_data (id) {
        id -> Text,
        name -> Text,
        data -> Binary,
    }
}

table! {
    rom_patches (id) {
        id -> Text,
        date_create -> Text,
        user_id -> Text,
        data -> Binary,
        description -> Nullable<Text>,
    }
}

table! {
    roms (id) {
        id -> Text,
        date_create -> Text,
        name -> Text,
        rom_data_id -> Text,
    }
}

table! {
    sav_patches (id) {
        id -> Text,
        date_create -> Text,
        user_id -> Text,
        data -> Binary,
        description -> Nullable<Text>,
    }
}

table! {
    savs (id) {
        id -> Text,
        date_create -> Text,
        data -> Binary,
    }
}

table! {
    users (id) {
        id -> Text,
        date_create -> Text,
        date_expire -> Text,
        access_token_hash -> Text,
        rom_id -> Nullable<Text>,
        sav_id -> Nullable<Text>,
    }
}

joinable!(rom_patches -> users (user_id));
joinable!(roms -> rom_data (rom_data_id));
joinable!(sav_patches -> users (user_id));
joinable!(users -> roms (rom_id));
joinable!(users -> savs (sav_id));

allow_tables_to_appear_in_same_query!(rom_data, rom_patches, roms, sav_patches, savs, users,);
