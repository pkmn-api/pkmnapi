use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TypeResponseErrorInvalid = BaseErrorResponse<TypeResponseErrorInvalidAttributes>;

impl TypeResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_types_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TypeResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeResponseErrorInvalidAttributes {
    pub message: String,
}
