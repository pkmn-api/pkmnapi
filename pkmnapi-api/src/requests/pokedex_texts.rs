use rocket_okapi::JsonSchema;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokedexTextRequest = BaseRequest<PokedexTextRequestType, PokedexTextRequestAttributes>;

impl PokedexTextRequest {
    pub fn get_text(&self) -> &String {
        &self.data.attributes.text
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
#[allow(non_camel_case_types)]
pub enum PokedexTextRequestType {
    pokedex_texts,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct PokedexTextRequestAttributes {
    pub text: String,
}
