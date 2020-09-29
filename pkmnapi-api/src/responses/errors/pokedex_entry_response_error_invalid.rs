use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokedexEntryResponseErrorInvalid =
    BaseErrorResponse<PokedexEntryResponseErrorInvalidAttributes>;

impl PokedexEntryResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = PokedexEntryResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokedex_entries_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: PokedexEntryResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokedexEntryResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct PokedexEntryResponseErrorInvalidAttributes {
    pub message: String,
}
