use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type MoveNameResponseError = BaseErrorResponse<MoveNameResponseErrorAttributes>;

impl MoveNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_move_names,
                _type: BaseErrorResponseType::errors,
                attributes: MoveNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveNameResponseErrorAttributes {
    pub message: String,
}
