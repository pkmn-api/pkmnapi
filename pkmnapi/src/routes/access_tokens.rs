use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::requests::access_tokens::AccessTokenRequest;
use crate::responses::errors::*;
use crate::utils::HostHeader;

#[post("/access_tokens", data = "<data>")]
pub fn post_access_token(
    sql: State<PkmnapiSQL>,
    host: HostHeader,
    data: Result<Json<AccessTokenRequest>, JsonError>,
) -> Result<status::Created<JsonValue>, ResponseError> {
    let data = match data {
        Ok(data) => data,
        Err(JsonError::Parse(_, e)) => {
            let message = format!("{}", e);
            let response = AccessTokenErrorInvalid::new(&message);

            return Err(ResponseError::AccessTokenErrorInvalid(status::BadRequest(
                Some(Json(response)),
            )));
        }
        _ => {
            let response = AccessTokenErrorInvalid::new(&String::from("an unknown error occurred"));

            return Err(ResponseError::AccessTokenErrorInvalid(status::BadRequest(
                Some(Json(response)),
            )));
        }
    };

    let data = data.into_inner();
    let email_address = data.get_email_address();

    let connection = sql.get_connection().unwrap();
    let (_user, access_token) = sql.insert_user(&connection, &email_address).unwrap();

    // TODO: Check if email is valid

    println!("SEND EMAIL: {}", access_token);
    println!("{}", host);

    Ok(status::Created(String::from("foo"), Some(json!({}))))
}
