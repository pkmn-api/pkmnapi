use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TrainerNameResponseErrorInvalid =
    BaseErrorResponse<TrainerNameResponseErrorInvalidAttributes>;

impl TrainerNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TrainerNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_trainer_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TrainerNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TrainerNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TrainerNameResponseErrorInvalidAttributes {
    pub message: String,
}
