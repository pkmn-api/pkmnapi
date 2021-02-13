use pkmnapi_email::*;
use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use rocket_okapi::openapi;
use validator;

use crate::guards::*;
use crate::requests::access_tokens::*;
use crate::responses::errors::*;
use crate::utils;

#[openapi]
#[post("/access_tokens", format = "application/json", data = "<data>")]
pub fn post_access_token(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
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

        if let Err(e) = email.send() {
            return Err(AccessTokenErrorEmail::new(&e));
        }

        json!({})
    };

    Ok(status::Created(
        utils::generate_url("access_tokens", None),
        Some(response),
    ))
}

#[openapi]
#[post("/access_tokens/delete", format = "application/json", data = "<data>")]
pub fn post_access_token_delete(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
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

    let delete_code = match sql.update_user_delete_code_by_id(&connection, &email_address) {
        Ok(Some(user)) => user.delete_code.unwrap_or("".to_owned()),
        _ => "".to_owned(),
    };

    let response = if cfg!(debug_assertions) {
        json!(delete_code)
    } else {
        if !delete_code.is_empty() {
            let email = PkmnapiEmail::new(
                &email_address,
                PkmnapiEmailTemplate::DeleteCode(email_address.clone(), delete_code.clone()),
            );

            if let Err(e) = email.send() {
                return Err(AccessTokenErrorEmail::new(&e));
            }
        }

        json!({})
    };

    Ok(status::Created(
        utils::generate_url("access_tokens", None),
        Some(response),
    ))
}

#[openapi]
#[delete("/access_tokens", format = "application/json", data = "<data>")]
pub fn delete_access_token(
    sql: State<PkmnapiSQL>,
    _rate_limit: RateLimit,
    data: Result<Json<AccessTokenDeleteRequest>, JsonError>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<status::NoContent, ResponseError> {
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

    let code = data.get_code();
    let email_address = data.get_email_address();
    let valid_email_address = validator::validate_email(email_address);

    if !valid_email_address {
        return Err(AccessTokenErrorInvalid::new(&format!(
            "Invalid email address: {}",
            email_address
        )));
    }

    let connection = sql.get_connection().unwrap();

    match sql.delete_user_by_id(&connection, &email_address, &code) {
        Ok(_) => {}
        _ => {
            return Err(AccessTokenErrorInvalid::new(
                &"An unknown error occurred".to_owned(),
            ))
        }
    };

    Ok(status::NoContent)
}
