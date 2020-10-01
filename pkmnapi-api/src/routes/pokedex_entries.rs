use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::pokedex_entries::*;
use crate::responses::errors::*;
use crate::responses::pokedex_entries::*;
use crate::utils;

#[get("/pokedex/entries/<pokedex_id>")]
pub fn get_pokedex_entry(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    pokedex_id: u8,
) -> Result<Json<PokedexEntryResponse>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let pokedex_entry = match db.get_pokedex_entry(&pokedex_id) {
        Ok(pokedex_entry) => pokedex_entry,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokedex_entries,
                Some(e.to_string()),
            ))
        }
    };
    let response = PokedexEntryResponse::new(&pokedex_id, &pokedex_entry);

    Ok(Json(response))
}

#[post(
    "/pokedex/entries/<pokedex_id>",
    format = "application/json",
    data = "<data>"
)]
pub fn post_pokedex_entry(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PokedexEntryRequest>, JsonError>,
    pokedex_id: u8,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = match access_token {
        Ok(access_token) => access_token.into_inner(),
        Err(_) => return Err(AccessTokenErrorUnauthorized::new()),
    };

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_pokedex_entries_invalid,
                Some(e.to_string()),
            ));
        }
        _ => {
            return Err(BadRequestError::new(
                BaseErrorResponseId::error_pokedex_entries_invalid,
                Some("An unknown error occurred".to_owned()),
            ));
        }
    };

    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let pokedex_entry = PokedexEntry {
        species: ROMString::from(data.get_species()),
        height: data.get_height(),
        weight: data.get_weight(),
    };

    let patch = match db.set_pokedex_entry(&pokedex_id, &pokedex_entry) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_pokedex_entries,
                Some(e.to_string()),
            ))
        }
    };

    let patch_description = match patch_description {
        Ok(patch_description) => patch_description.into_inner(),
        Err(_) => None,
    };

    if let Err(e) = sql.insert_rom_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        return Err(NotFoundError::new(
            BaseErrorResponseId::error_pokedex_entries,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}
