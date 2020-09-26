use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokemonNameResponseErrorInvalid =
    BaseErrorResponse<PokemonNameResponseErrorInvalidAttributes>;

impl PokemonNameResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = PokemonNameResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokemon_names_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: PokemonNameResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokemonNameResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonNameResponseErrorInvalidAttributes {
    pub message: String,
}
