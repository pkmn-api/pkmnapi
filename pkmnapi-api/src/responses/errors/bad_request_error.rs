use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type BadRequestError = BaseErrorResponse<BadRequestErrorAttributes>;

impl BadRequestError {
    pub fn new(id: BaseErrorResponseId, message: Option<String>) -> ResponseError {
        let response = BadRequestError {
            data: BaseErrorResponseData {
                id,
                _type: BaseErrorResponseType::errors,
                attributes: BadRequestErrorAttributes {
                    message: message.unwrap_or("Invalid request".to_owned()),
                },
            },
        };

        ResponseError::BadRequestError(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct BadRequestErrorAttributes {
    pub message: String,
}
