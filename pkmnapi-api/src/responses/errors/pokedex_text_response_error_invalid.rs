use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokedexTextResponseErrorInvalid =
    BaseErrorResponse<PokedexTextResponseErrorInvalidAttributes>;

impl PokedexTextResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = PokedexTextResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokedex_texts_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: PokedexTextResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokedexTextResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct PokedexTextResponseErrorInvalidAttributes {
    pub message: String,
}
