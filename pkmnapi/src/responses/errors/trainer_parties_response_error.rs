use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TrainerPartiesResponseError = BaseErrorResponse<TrainerPartiesResponseErrorAttributes>;

impl TrainerPartiesResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerPartiesResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_parties,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerPartiesResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerPartiesResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerPartiesResponseErrorAttributes {
    pub message: String,
}
