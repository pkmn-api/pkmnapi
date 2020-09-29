use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type PokedexEntryResponseError = BaseErrorResponse<PokedexEntryResponseErrorAttributes>;

impl PokedexEntryResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = PokedexEntryResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_pokedex_entries,
                _type: BaseErrorResponseType::errors,
                attributes: PokedexEntryResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::PokedexEntryResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct PokedexEntryResponseErrorAttributes {
    pub message: String,
}
