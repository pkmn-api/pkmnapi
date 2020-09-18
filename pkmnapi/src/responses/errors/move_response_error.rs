use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type MoveResponseError = BaseErrorResponse<MoveResponseErrorAttributes>;

impl MoveResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_moves,
                _type: BaseErrorResponseType::errors,
                attributes: MoveResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveResponseErrorAttributes {
    pub message: String,
}
