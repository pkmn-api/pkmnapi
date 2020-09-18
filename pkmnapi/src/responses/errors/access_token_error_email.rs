use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type AccessTokenErrorEmail = BaseErrorResponse<AccessTokenErrorEmailAttributes>;

impl AccessTokenErrorEmail {
    pub fn new(message: &String) -> ResponseError {
        let response = AccessTokenErrorEmail {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_email,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorEmailAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorEmail(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorEmailAttributes {
    pub message: String,
}
