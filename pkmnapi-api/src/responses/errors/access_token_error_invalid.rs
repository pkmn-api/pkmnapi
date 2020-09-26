use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type AccessTokenErrorInvalid = BaseErrorResponse<AccessTokenErrorInvalidAttributes>;

impl AccessTokenErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalidAttributes {
    pub message: String,
}
