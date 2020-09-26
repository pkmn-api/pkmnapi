use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokemonPicResponseError = BaseErrorResponse<PokemonPicResponseErrorAttributes>;

impl PokemonPicResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonPicResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_pics,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonPicResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonPicResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonPicResponseErrorAttributes {
    pub message: String,
}
