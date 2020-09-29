use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokedexTextResponseError = BaseErrorResponse<PokedexTextResponseErrorAttributes>;

impl PokedexTextResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokedexTextResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokedex_texts,
                _type: BaseErrorResponseType::errors,
                attributes: PokedexTextResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokedexTextResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokedexTextResponseErrorAttributes {
    pub message: String,
}
