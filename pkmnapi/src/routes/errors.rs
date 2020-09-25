use rocket::http::{ContentType, Status};
use rocket::response::Response;
use rocket::Request;
use std::io::Cursor;

use crate::responses::errors::*;

#[catch(404)]
pub fn not_found(_req: &Request) -> Result<ResponseError, ResponseError> {
    Ok(NotFoundError::new())
}

#[catch(500)]
pub fn internal_server_error<'a>(_req: &Request) -> Result<Response<'a>, ResponseError> {
    let response = InternalServerError::new();
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .status(Status::InternalServerError)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
}
