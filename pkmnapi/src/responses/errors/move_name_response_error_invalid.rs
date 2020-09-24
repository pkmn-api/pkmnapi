use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type MoveNameResponseErrorInvalid = BaseErrorResponse<MoveNameResponseErrorInvalidAttributes>;

impl MoveNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = MoveNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_move_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: MoveNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MoveNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct MoveNameResponseErrorInvalidAttributes {
    pub message: String,
}
