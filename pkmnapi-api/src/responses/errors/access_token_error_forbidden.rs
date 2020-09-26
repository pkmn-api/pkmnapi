use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type AccessTokenErrorForbidden = BaseErrorResponse<AccessTokenErrorForbiddenAttributes>;

impl AccessTokenErrorForbidden {
    pub fn new() -> ResponseError {
        let response = AccessTokenErrorForbidden {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_access_tokens_forbidden,
                _type: BaseErrorResponseType::errors,
                attributes: AccessTokenErrorForbiddenAttributes {
                    message: "Authorization header must not be set".to_owned(),
                },
            },
        };

        ResponseError::AccessTokenErrorForbidden(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorForbiddenAttributes {
    pub message: String,
}
