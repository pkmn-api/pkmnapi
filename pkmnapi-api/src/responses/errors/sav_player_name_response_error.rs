use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavPlayerNameResponseError = BaseErrorResponse<SavPlayerNameResponseErrorAttributes>;

impl SavPlayerNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = SavPlayerNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_sav_player_names,
                _type: BaseErrorResponseType::errors,
                attributes: SavPlayerNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::SavPlayerNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct SavPlayerNameResponseErrorAttributes {
    pub message: String,
}
