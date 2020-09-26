use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TMResponseError = BaseErrorResponse<TMResponseErrorAttributes>;

impl TMResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TMResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_tms,
                _type: BaseErrorResponseType::errors,
                attributes: TMResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TMResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TMResponseErrorAttributes {
    pub message: String,
}
