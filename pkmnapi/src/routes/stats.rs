use pkmnapi_db::types::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::stats::*;
use crate::responses::errors::*;
use crate::responses::stats::*;
use crate::utils;

#[get("/stats/<pokedex_id>")]
pub fn get_stats(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<StatsResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let db = utils::get_db_with_applied_patches(sql, &access_token)?;

    let stats = match db.get_stats(pokedex_id) {
        Ok(stats) => stats,
        Err(e) => return Err(StatsResponseError::new(&e.to_string())),
    };

    let type_names: Result<Vec<TypeName>, _> = stats
        .type_ids
        .iter()
        .map(|type_id| match db.get_type_name(type_id.value()) {
            Ok(type_name) => Ok(type_name),
            Err(e) => return Err(StatsResponseError::new(&e.to_string())),
        })
        .collect();

    let type_names = match type_names {
        Ok(type_names) => type_names,
        Err(e) => return Err(e),
    };

    let response = StatsResponse::new(&pokedex_id, &stats, type_names);

    Ok(Json(response))
}

#[post("/stats/<pokedex_id>", format = "application/json", data = "<data>")]
pub fn post_stats(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<StatsRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(StatsResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(StatsResponseErrorInvalid::new(
                &"An unknown error occurred".to_string(),
            ));
        }
    };

    let connection = sql.get_connection().unwrap();
    let rom_data_sql = match sql.select_user_rom_data_by_access_token(&connection, &access_token) {
        Ok(Some(rom_sql)) => rom_sql,
        _ => return Err(RomResponseErrorNoRom::new()),
    };

    let db = match PkmnapiDB::new(&rom_data_sql.data) {
        Ok(db) => db,
        Err(_) => return Err(RomResponseErrorInvalidRom::new()),
    };

    let stats = Stats {
        pokedex_id: PokedexID::from(pokedex_id),
        base_hp: data.get_base_hp(),
        base_attack: data.get_base_attack(),
        base_defence: data.get_base_defence(),
        base_speed: data.get_base_speed(),
        base_special: data.get_base_special(),
        type_ids: data
            .get_type_ids()
            .into_iter()
            .map(|type_id| TypeID::from(type_id.parse::<u8>().unwrap()))
            .collect(),
        catch_rate: data.get_catch_rate(),
        base_exp_yield: data.get_base_exp_yield(),
    };

    let patch = match db.set_stats(pokedex_id, stats) {
        Ok(patch) => patch,
        Err(e) => return Err(StatsResponseError::new(&e.to_string())),
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    match sql.insert_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => {}
        Err(e) => return Err(StatsResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
