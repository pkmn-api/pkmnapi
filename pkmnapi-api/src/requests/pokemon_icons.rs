use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokemonIconRequest = BaseRequest<PokemonIconRequestType, PokemonIconRequestAttributes>;

impl PokemonIconRequest {
    pub fn get_icon_id(&self) -> u8 {
        self.data.attributes.icon.id
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonIconRequestType {
    pokemon_icons,
}

#[derive(Debug, Deserialize)]
pub struct PokemonIconRequestAttributes {
    pub icon: PokemonIconRequestAttributesIcon,
}

#[derive(Debug, Deserialize)]
pub struct PokemonIconRequestAttributesIcon {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
