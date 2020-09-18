use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TypeEffectResponseError = BaseErrorResponse<TypeEffectResponseErrorAttributes>;

impl TypeEffectResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_effects,
                _type: BaseErrorResponseType::errors,
                attributes: TypeEffectResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorAttributes {
    pub message: String,
}
