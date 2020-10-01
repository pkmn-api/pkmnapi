use rocket::http::{ContentType, Status};
use rocket::response::Response;
use rocket::Outcome;
use rocket::Request;
use std::io::Cursor;

use crate::guards::*;
use crate::responses::errors::*;

#[catch(404)]
pub fn not_found(_req: &Request) -> Result<ResponseError, ResponseError> {
    Ok(NotFoundError::new(
        BaseErrorResponseId::error_not_found,
        None,
    ))
}

#[catch(429)]
pub fn too_many_requests<'a>(req: &Request) -> Result<Response<'a>, ResponseError> {
    let wait_time = match req.guard::<RateLimit>() {
        Outcome::Failure((_, RateLimitError::TooManyRequests(wait_time))) => wait_time,
        _ => 0,
    };

    let response = TooManyRequestsError::new(wait_time);
    let body = serde_json::to_string(&response).unwrap();

    let response = Response::build()
        .status(Status::TooManyRequests)
        .header(ContentType::JSON)
        .sized_body(Cursor::new(body))
        .finalize();

    Ok(response)
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
