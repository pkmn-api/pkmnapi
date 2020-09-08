use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::State;

use crate::responses::errors::*;

pub fn get_db_with_applied_patches(
    sql: State<PkmnapiSQL>,
    access_token: &String,
) -> Result<PkmnapiDB, ResponseError> {
    let connection = sql.get_connection().unwrap();
    let rom_data_sql = match sql.select_user_rom_data_by_access_token(&connection, &access_token) {
        Ok(Some(rom_sql)) => rom_sql,
        _ => return Err(RomResponseErrorNoRom::new()),
    };

    let patches = match sql.select_patches_by_access_token(&connection, &access_token) {
        Ok(patches) => patches,
        Err(_) => vec![],
    };

    let mut db = match PkmnapiDB::new(&rom_data_sql.data) {
        Ok(db) => db,
        Err(_) => return Err(RomResponseErrorInvalidRom::new()),
    };

    for patch in patches {
        db.apply_patch(patch.data);
    }

    Ok(db)
}
