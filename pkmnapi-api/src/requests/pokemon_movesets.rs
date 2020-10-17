use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokemonMovesetRequest =
    BaseRequest<PokemonMovesetRequestType, PokemonMovesetRequestAttributes>;

impl PokemonMovesetRequest {
    pub fn get_moveset(&self) -> Vec<u8> {
        self.data
            .attributes
            .moveset
            .iter()
            .map(|moveset| moveset._move.id)
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonMovesetRequestType {
    pokemon_movesets,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMovesetRequestAttributes {
    pub moveset: Vec<PokemonMovesetRequestAttributesMoveset>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMovesetRequestAttributesMoveset {
    #[serde(rename = "move")]
    pub _move: PokemonMovesetRequestAttributesMovesetMove,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMovesetRequestAttributesMovesetMove {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
