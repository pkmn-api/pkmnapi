use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type HMResponseErrorInvalid = BaseErrorResponse<HMResponseErrorInvalidAttributes>;

impl HMResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = HMResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_hms_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: HMResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::HMResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct HMResponseErrorInvalidAttributes {
    pub message: String,
}
