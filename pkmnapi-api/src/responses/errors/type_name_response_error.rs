use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TypeNameResponseError = BaseErrorResponse<TypeNameResponseErrorAttributes>;

impl TypeNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_names,
                _type: BaseErrorResponseType::errors,
                attributes: TypeNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeNameResponseErrorAttributes {
    pub message: String,
}
