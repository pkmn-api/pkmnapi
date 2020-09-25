use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type ETagErrorMissing = BaseErrorResponse<ETagErrorMissingAttributes>;

impl ETagErrorMissing {
    pub fn new() -> ResponseError {
        let response = ETagErrorMissing {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_etag_missing,
                _type: BaseErrorResponseType::errors,
                attributes: ETagErrorMissingAttributes {
                    message: "If-Match header must be set".to_owned(),
                },
            },
        };

        ResponseError::ETagErrorMissing(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct ETagErrorMissingAttributes {
    pub message: String,
}
