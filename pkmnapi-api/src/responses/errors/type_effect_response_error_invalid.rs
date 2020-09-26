use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type TypeEffectResponseErrorInvalid =
    BaseErrorResponse<TypeEffectResponseErrorInvalidAttributes>;

impl TypeEffectResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = TypeEffectResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_type_effects_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: TypeEffectResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::TypeEffectResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseErrorInvalidAttributes {
    pub message: String,
}
