use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokemonNameRequest = BaseRequest<PokemonNameRequestType, PokemonNameRequestAttributes>;

impl PokemonNameRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum PokemonNameRequestType {
    pokemon_names,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PokemonNameRequestAttributes {
    pub name: String,
}
