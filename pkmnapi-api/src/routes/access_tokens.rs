use pkmnapi_email::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use validator;

use crate::guards::*;
use crate::requests::access_tokens::*;
use crate::responses::errors::*;
use crate::utils;

#[post("/access_tokens", format = "application/json", data = "<data>")]
pub fn post_access_token(
    sql: State<PkmnapiSQL>,
    data: Result<Json<AccessTokenRequest>, JsonError>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<status::Created<JsonValue>, ResponseError> {
    if access_token.is_ok() {
        return Err(AccessTokenErrorForbidden::new());
    }

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(AccessTokenErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(AccessTokenErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ));
        }
    };

    let email_address = data.get_email_address();
    let valid_email_address = validator::validate_email(email_address);

    if !valid_email_address {
        return Err(AccessTokenErrorInvalid::new(&format!(
            "Invalid email address: {}",
            email_address
        )));
    }

    let connection = sql.get_connection().unwrap();

    match sql.select_user_by_id(&connection, &email_address) {
        Ok(Some(user)) => {
            let seconds = user.seconds_to_expiration();

            if seconds >= 0 {
                return Err(AccessTokenErrorTimeout::new(&format!(
                    "Please try again in {} seconds",
                    seconds
                )));
            }
        }
        _ => {}
    };

    let (_, access_token) = sql.insert_user(&connection, &email_address).unwrap();

    let response = if cfg!(debug_assertions) {
        json!(access_token)
    } else {
        let email = PkmnapiEmail::new(
            &email_address,
            PkmnapiEmailTemplate::AccessToken(access_token.clone()),
        );

        match email.send() {
            Ok(_) => {}
            Err(e) => return Err(AccessTokenErrorEmail::new(&e)),
        };

        json!({})
    };

    Ok(status::Created(
        utils::generate_url("access_tokens", None),
        Some(response),
    ))
}
