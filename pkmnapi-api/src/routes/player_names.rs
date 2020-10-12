use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::player_names::*;
use crate::responses::errors::*;
use crate::responses::player_names::*;
use crate::utils;

#[get("/player_names")]
pub fn get_player_names(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<PlayerNamesResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let player_names = match db.get_player_names() {
        Ok(player_names) => player_names,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_player_names,
                Some(e.to_string()),
            ))
        }
    };

    let response = PlayerNamesResponse::new(&player_names);

    Ok(Json(response))
}

#[post("/player_names", format = "application/json", data = "<data>")]
pub fn post_player_names(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<PlayerNamesRequest>, JsonError>,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_player_names_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let player_names = PlayerNames {
        player: data
            .get_player_names()
            .iter()
            .map(|name| ROMString::from(name))
            .collect(),
        rival: data
            .get_rival_names()
            .iter()
            .map(|name| ROMString::from(name))
            .collect(),
    };

    let patch = match db.set_player_names(&player_names) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_player_names,
                Some(e.to_string()),
            ))
        }
    };

    utils::insert_rom_patch(
        sql,
        connection,
        access_token,
        patch,
        patch_description,
        BaseErrorResponseId::error_player_names,
    )?;

    Ok(status::Accepted(Some(json!({}))))
}
