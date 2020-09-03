use pkmnapi_sql::*;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::{Json, JsonError, JsonValue};

use crate::requests::access_tokens::AccessTokenRequest;
use crate::responses::access_tokens::AccessTokenInvalid;
use crate::utils::HostHeader;

#[post("/access_tokens", data = "<data>")]
pub fn post_access_token(
    sql: State<PkmnapiSQL>,
    host: HostHeader,
    data: Result<Json<AccessTokenRequest>, JsonError>,
) -> Result<status::Created<JsonValue>, status::BadRequest<Json<AccessTokenInvalid>>> {
    let data = match data {
        Ok(data) => data,
        Err(JsonError::Parse(_, e)) => {
            let message = format!("{}", e);
            let response = AccessTokenInvalid::new(&message);

            return Err(status::BadRequest(Some(Json(response))));
        }
        _ => return Err(status::BadRequest(None)),
    };

    let data = data.into_inner();
    let email_address = data.get_email_address();

    let (user, access_token) = sql.insert_user(&email_address).unwrap();

    println!("SEND EMAIL: {}", access_token);
    println!("{}", host);

    Ok(status::Created(String::from("foo"), Some(json!({}))))
}
