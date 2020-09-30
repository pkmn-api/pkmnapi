use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokemonEvolutionsResponseError =
    BaseErrorResponse<PokemonEvolutionsResponseErrorAttributes>;

impl PokemonEvolutionsResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonEvolutionsResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_evolutions,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonEvolutionsResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonEvolutionsResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonEvolutionsResponseErrorAttributes {
    pub message: String,
}
