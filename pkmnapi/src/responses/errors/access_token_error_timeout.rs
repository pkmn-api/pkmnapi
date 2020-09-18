use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type AccessTokenErrorTimeout = BaseErrorResponse<AccessTokenErrorTimeoutAttributes>;

impl AccessTokenErrorTimeout {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorTimeout {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_timeout,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorTimeoutAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorTimeout(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorTimeoutAttributes {
    pub message: String,
}
