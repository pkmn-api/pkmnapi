use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokemonEvolutionsResponseErrorInvalid =
    BaseErrorResponse<PokemonEvolutionsResponseErrorInvalidAttributes>;

impl PokemonEvolutionsResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonEvolutionsResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_evolutions_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonEvolutionsResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonEvolutionsResponseErrorInvalid(status::BadRequest(Some(Json(
            response,
        ))))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonEvolutionsResponseErrorInvalidAttributes {
    pub message: String,
}
