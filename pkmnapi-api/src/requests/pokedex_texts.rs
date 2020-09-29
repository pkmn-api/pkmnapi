use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokedexTextRequest = BaseRequest<PokedexTextRequestType, PokedexTextRequestAttributes>;

impl PokedexTextRequest {
    pub fn get_text(&self) -> &String {
        &self.data.attributes.text
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokedexTextRequestType {
    pokedex_texts,
}

#[derive(Debug, Deserialize)]
pub struct PokedexTextRequestAttributes {
    pub text: String,
}
