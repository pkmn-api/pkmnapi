use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TypeNameResponseErrorInvalid = BaseErrorResponse<TypeNameResponseErrorInvalidAttributes>;

impl TypeNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TypeNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeNameResponseErrorInvalidAttributes {
    pub message: String,
}
