use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_db::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokemon_names::*;
use crate::responses::errors::*;
use crate::responses::pokemon_names::*;
use crate::utils;

#[get("/pokemon/names/<pokedex_id>")]
pub fn get_pokemon_name(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokemonNameResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let db = utils::get_db_with_applied_patches(sql, &access_token)?;

    let pokemon_name = match db.get_pokemon_name(&pokedex_id) {
        Ok(pokemon_name) => pokemon_name,
        Err(e) => return Err(PokemonNameResponseError::new(&e.to_string())),
    };
    let response = PokemonNameResponse::new(&pokedex_id, &pokemon_name);

    Ok(Json(response))
}

#[post(
    "/pokemon/names/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokemon_name(
    sql: State<PkmnapiSQL>,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokemonNameRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(PokemonNameResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(PokemonNameResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
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

    let pokemon_name = PokemonName {
        name: ROMString::from(data.get_name()),
    };

    let patch = match db.set_pokemon_name(&pokedex_id, &pokemon_name) {
        Ok(patch) => patch,
        Err(e) => return Err(PokemonNameResponseError::new(&e.to_string())),
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
        Err(e) => return Err(PokemonNameResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
