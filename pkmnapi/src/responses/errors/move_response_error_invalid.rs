use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type MoveResponseErrorInvalid = BaseErrorResponse<MoveResponseErrorInvalidAttributes>;

impl MoveResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_moves_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: MoveResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveResponseErrorInvalidAttributes {
    pub message: String,
}
