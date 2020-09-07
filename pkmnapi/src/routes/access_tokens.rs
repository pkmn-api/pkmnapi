use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};
use validator;

use crate::guards::access_tokens::*;
use crate::requests::access_tokens::*;
use crate::responses::errors::*;

#[post("/access_tokens", format = "application/json", data = "<data>")]
pub fn post_access_token(
    sql: State<PkmnapiSQL>,
    data: Result<Json<AccessTokenRequest>, JsonError>,
    access_token: Result<AccessToken, AccessTokenError>,
) -> Result<status::Created<JsonValue>, ResponseError> {
    if let Ok(_) = access_token {
        return Err(AccessTokenErrorForbidden::new());
    }

    let data = match data {
        Ok(data) => data.into_inner(),
        Err(JsonError::Parse(_, e)) => {
            return Err(AccessTokenErrorInvalid::new(&e.to_string()));
        }
        _ => {
            return Err(AccessTokenErrorInvalid::new(
                &"An unknown error occurred".to_string(),
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
    let (_, access_token) = sql.insert_user(&connection, &email_address).unwrap();

    // TODO: send email

    #[cfg(debug_assertions)]
    let response = json!(access_token);

    #[cfg(not(debug_assertions))]
    let response = json!({});

    Ok(status::Created(String::from("foo"), Some(response)))
}
