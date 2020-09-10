use pkmnapi_db::types::PokemonName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokemonNameResponse = BaseResponse<PokemonNameResponseAttributes>;

impl PokemonNameResponse {
    /// Create a new `PokemonNameResponse`
    pub fn new(pokedex_id: &u8, pokemon_name: &PokemonName) -> PokemonNameResponse {
        PokemonNameResponse {
            data: BaseResponseData {
                id: pokedex_id.to_string(),
                _type: BaseResponseType::pokemon_names,
                attributes: PokemonNameResponseAttributes {
                    name: pokemon_name.name.to_string(),
                },
                links: Links {
                    _self: utils::generate_url("pokemon/names", Some(&pokedex_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("pokemon/names", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonNameResponseAttributes {
    pub name: String,
}
