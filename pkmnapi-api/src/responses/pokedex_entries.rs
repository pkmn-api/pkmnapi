use pkmnapi_db::types::PokedexEntry;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type PokedexEntryResponse = BaseResponse<PokedexEntryResponseAttributes>;

impl PokedexEntryResponse {
    /// Create a new `PokedexEntryResponse`
    pub fn new(pokedex_id: &u8, pokedex_entry: &PokedexEntry) -> PokedexEntryResponse {
        PokedexEntryResponse {
            data: BaseResponseData {
                id: pokedex_id.to_string(),
                _type: BaseResponseType::pokedex_entries,
                attributes: PokedexEntryResponseAttributes {
                    species: pokedex_entry.species.to_string(),
                    height: pokedex_entry.height,
                    weight: pokedex_entry.weight,
                },
                links: Links {
                    _self: utils::generate_url("pokedex/entries", Some(&pokedex_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("pokedex/entries", Some(&pokedex_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PokedexEntryResponseAttributes {
    pub species: String,
    pub height: u32,
    pub weight: u32,
}
