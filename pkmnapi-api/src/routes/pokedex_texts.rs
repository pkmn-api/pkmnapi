use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokedex_texts::*;
use crate::responses::errors::*;
use crate::responses::pokedex_texts::*;
use crate::utils;

#[get("/pokedex/texts/<pokedex_id>")]
pub fn get_pokedex_text(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokedexTextResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokedex_text = match db.get_pokedex_text(&pokedex_id) {
        Ok(pokedex_text) => pokedex_text,
        Err(e) => return Err(PokedexTextResponseError::new(&e.to_string())),
    };
    let response = PokedexTextResponse::new(&pokedex_id, &pokedex_text);

    Ok(Json(response))
}

#[post(
    "/pokedex/texts/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokedex_text(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokedexTextRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(PokedexTextResponseErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(PokedexTextResponseErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokedex_text = PokedexText {
        text: ROMString::from(data.get_text()),
    };

    let patch = match db.set_pokedex_text(&pokedex_id, &pokedex_text) {
        Ok(patch) => patch,
        Err(e) => return Err(PokedexTextResponseError::new(&e.to_string())),
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    match sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        Ok(_) => {}
        Err(e) => return Err(PokedexTextResponseError::new(&e.to_string())),
    };

    Ok(status::Accepted(Some(json!({}))))
}
