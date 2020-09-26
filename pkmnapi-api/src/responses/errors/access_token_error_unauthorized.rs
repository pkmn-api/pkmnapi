use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type AccessTokenErrorUnauthorized = BaseErrorResponse<AccessTokenErrorUnauthorizedAttributes>;

impl AccessTokenErrorUnauthorized {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorUnauthorized {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_unauthorized,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorUnauthorizedAttributes {
                    message: "Authorization header must be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorUnauthorized(status::Unauthorized(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorUnauthorizedAttributes {
    pub message: String,
}
