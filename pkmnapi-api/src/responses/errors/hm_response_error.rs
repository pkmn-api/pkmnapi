use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type HMResponseError = BaseErrorResponse<HMResponseErrorAttributes>;

impl HMResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = HMResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_hms,
                _type: BaseErrorResponseType::errors,
                attributes: HMResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::HMResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct HMResponseErrorAttributes {
    pub message: String,
}
