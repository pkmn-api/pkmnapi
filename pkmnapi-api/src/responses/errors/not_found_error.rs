use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type NotFoundError = BaseErrorResponse<NotFoundErrorAttributes>;

impl NotFoundError {
    pub fn new() -> ResponseError {
        let response = NotFoundError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_not_found,
                _type: BaseErrorResponseType::errors,
                attributes: NotFoundErrorAttributes {
                    message: "The requested resource could not be found".to_owned(),
                },
            },
        };

        ResponseError::NotFoundError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct NotFoundErrorAttributes {
    pub message: String,
}
