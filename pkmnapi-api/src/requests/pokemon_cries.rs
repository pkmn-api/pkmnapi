use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokemonCryRequest = BaseRequest<PokemonCryRequestType, PokemonCryRequestAttributes>;

impl PokemonCryRequest {
    pub fn get_base(&self) -> u8 {
        self.data.attributes.base
    }

    pub fn get_pitch(&self) -> u8 {
        self.data.attributes.pitch
    }

    pub fn get_length(&self) -> u8 {
        self.data.attributes.length
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum PokemonCryRequestType {
    pokemon_cries,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PokemonCryRequestAttributes {
    pub base: u8,
    pub pitch: u8,
    pub length: u8,
}
