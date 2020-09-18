use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TypeResponseError = BaseErrorResponse<TypeResponseErrorAttributes>;

impl TypeResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_types,
                _type: BaseErrorResponseType::errors,
                attributes: TypeResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorAttributes {
    pub message: String,
}
