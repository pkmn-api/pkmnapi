use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TrainerPicResponseError = BaseErrorResponse<TrainerPicResponseErrorAttributes>;

impl TrainerPicResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerPicResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_pics,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerPicResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerPicResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerPicResponseErrorAttributes {
    pub message: String,
}
