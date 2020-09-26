use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TrainerNameResponseError = BaseErrorResponse<TrainerNameResponseErrorAttributes>;

impl TrainerNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_names,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerNameResponseErrorAttributes {
    pub message: String,
}
