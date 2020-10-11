use pkmnapi_db::string::*;
use pkmnapi_db::types::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::guards::*;
use crate::requests::sav_player_names::*;
use crate::responses::errors::*;
use crate::responses::sav_player_names::*;
use crate::utils;

#[get("/savs/player_names")]
pub fn get_sav_player_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<Json<SavPlayerNameResponse>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let (db, _) = utils::get_db_with_applied_patches(&sql, &access_token)?;

    let sav = match db.sav {
        Some(sav) => sav,
        None => return Err(SavErrorNoSav::new()),
    };

    let player_id = sav.get_player_id().unwrap_or(0x00);

    let player_name = match sav.get_player_name() {
        Ok(player_name) => player_name,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_sav_player_names,
                Some(e.to_string()),
            ))
        }
    };

    let response = SavPlayerNameResponse::new(&player_id, &player_name);

    Ok(Json(response))
}

#[post("/savs/player_names", format = "application/json", data = "<data>")]
pub fn post_sav_player_name(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    access_token: Result<AccessToken, AccessTokenError>,
    patch_description: Result<PatchDescription, PatchDescriptionError>,
    data: Result<Json<SavPlayerNameRequest>, JsonError>,
) -> Result<status::Accepted<JsonValue>, ResponseError> {
    let access_token = utils::get_access_token(access_token)?;
    let data = utils::get_data(data, BaseErrorResponseId::error_sav_player_names_invalid)?;
    let (db, connection) = utils::get_db(&sql, &access_token)?;

    let sav = match db.sav {
        Some(sav) => sav,
        None => return Err(SavErrorNoSav::new()),
    };

    let player_name = SavePlayerName {
        name: ROMString::from(data.get_name()),
    };

    let patch = match sav.set_player_name(&player_name) {
        Ok(patch) => patch,
        Err(e) => {
            return Err(NotFoundError::new(
                BaseErrorResponseId::error_sav_player_names,
                Some(e.to_string()),
            ))
        }
    };

    let patch_description = utils::get_patch_description(patch_description);

    if let Err(e) = sql.insert_sav_patch(
        &connection,
        &access_token,
        &patch.to_raw(),
        patch_description,
    ) {
        return Err(NotFoundError::new(
            BaseErrorResponseId::error_sav_player_names,
            Some(e.to_string()),
        ));
    }

    Ok(status::Accepted(Some(json!({}))))
}
