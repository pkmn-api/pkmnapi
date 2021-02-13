use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokedexEntryRequest = BaseRequest<PokedexEntryRequestType, PokedexEntryRequestAttributes>;

impl PokedexEntryRequest {
    pub fn get_species(&self) -> &String {
        &self.data.attributes.species
    }

    pub fn get_height(&self) -> u32 {
        self.data.attributes.height
    }

    pub fn get_weight(&self) -> u32 {
        self.data.attributes.weight
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum PokedexEntryRequestType {
    pokedex_entries,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PokedexEntryRequestAttributes {
    pub species: String,
    pub height: u32,
    pub weight: u32,
}
