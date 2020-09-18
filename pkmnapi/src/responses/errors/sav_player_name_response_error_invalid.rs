use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavPlayerNameResponseErrorInvalid =
    BaseErrorResponse<SavPlayerNameResponseErrorInvalidAttributes>;

impl SavPlayerNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = SavPlayerNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_sav_player_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: SavPlayerNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::SavPlayerNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavPlayerNameResponseErrorInvalidAttributes {
    pub message: String,
}
