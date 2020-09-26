use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokemonStatsResponseError = BaseErrorResponse<PokemonStatsResponseErrorAttributes>;

impl PokemonStatsResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonStatsResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_stats,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonStatsResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonStatsResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonStatsResponseErrorAttributes {
    pub message: String,
}
