use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type ETagErrorMismatch = BaseErrorResponse<ETagErrorMismatchAttributes>;

impl ETagErrorMismatch {
    pub fn new() -> ResponseError {
        let response = ETagErrorMismatch {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_etag_mismatch,
                _type: BaseErrorResponseType::errors,
                attributes: ETagErrorMismatchAttributes {
                    message: "ETag mismatch".to_owned(),
                },
            },
        };

        ResponseError::ETagErrorMismatch(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct ETagErrorMismatchAttributes {
    pub message: String,
}
