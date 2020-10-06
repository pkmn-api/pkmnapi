use pkmnapi_db::cry::Cry;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokemonCryResponse = BaseResponse<PokemonCryResponseAttributes>;

impl PokemonCryResponse {
    /// Create a new `PokemonCryResponse`
    pub fn new(pokedex_id: &u8, pokemon_cry: &Cry) -> PokemonCryResponse {
        PokemonCryResponse {
            data: BaseResponseData {
                id: pokedex_id.to_string(),
                _type: BaseResponseType::pokemon_cries,
                attributes: PokemonCryResponseAttributes {
                    base: pokemon_cry.base,
                    pitch: pokemon_cry.pitch,
                    length: pokemon_cry.length,
                },
                links: Links {
                    _self: utils::generate_url("pokemon/cries", Some(&pokedex_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("pokemon/cries", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokemonCryResponseAttributes {
    pub base: u8,
    pub pitch: u8,
    pub length: u8,
}
