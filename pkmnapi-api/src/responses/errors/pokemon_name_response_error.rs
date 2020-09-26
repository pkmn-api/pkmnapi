use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokemonNameResponseError = BaseErrorResponse<PokemonNameResponseErrorAttributes>;

impl PokemonNameResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonNameResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_names,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonNameResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonNameResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonNameResponseErrorAttributes {
    pub message: String,
}
