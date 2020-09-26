use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TrainerPartiesResponseErrorInvalid =
    BaseErrorResponse<TrainerPartiesResponseErrorInvalidAttributes>;

impl TrainerPartiesResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerPartiesResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_parties_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerPartiesResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerPartiesResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerPartiesResponseErrorInvalidAttributes {
    pub message: String,
}
