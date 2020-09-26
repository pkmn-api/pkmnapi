use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokemonStatsResponseErrorInvalid =
    BaseErrorResponse<PokemonStatsResponseErrorInvalidAttributes>;

impl PokemonStatsResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonStatsResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_stats_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonStatsResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonStatsResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonStatsResponseErrorInvalidAttributes {
    pub message: String,
}
