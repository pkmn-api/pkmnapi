use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TMResponseErrorInvalid = BaseErrorResponse<TMResponseErrorInvalidAttributes>;

impl TMResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TMResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_tms_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TMResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TMResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TMResponseErrorInvalidAttributes {
    pub message: String,
}
