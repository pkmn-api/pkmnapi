table! {
    rom_data (id) {
        id -> Varchar,
        name -> Varchar,
        data -> Bytea,
    }
}

table! {
    rom_patches (id) {
        id -> Varchar,
        date_create -> Varchar,
        user_id -> Varchar,
        data -> Bytea,
        description -> Nullable<Varchar>,
        etag -> Varchar,
    }
}

table! {
    roms (id) {
        id -> Varchar,
        date_create -> Varchar,
        name -> Varchar,
        etag -> Varchar,
        rom_data_id -> Varchar,
    }
}

table! {
    sav_patches (id) {
        id -> Varchar,
        date_create -> Varchar,
        user_id -> Varchar,
        data -> Bytea,
        description -> Nullable<Varchar>,
        etag -> Varchar,
    }
}

table! {
    savs (id) {
        id -> Varchar,
        date_create -> Varchar,
        data -> Bytea,
        etag -> Varchar,
    }
}

table! {
    users (id) {
        id -> Varchar,
        date_create -> Varchar,
        date_expire -> Varchar,
        access_token_hash -> Varchar,
        rom_id -> Nullable<Varchar>,
        sav_id -> Nullable<Varchar>,
    }
}

joinable!(rom_patches -> users (user_id));
joinable!(roms -> rom_data (rom_data_id));
joinable!(sav_patches -> users (user_id));
joinable!(users -> roms (rom_id));
joinable!(users -> savs (sav_id));

allow_tables_to_appear_in_same_query!(rom_data, rom_patches, roms, sav_patches, savs, users,);
