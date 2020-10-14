use pkmnapi_db::{Party, PartyPokemon};
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TrainerPartiesRequest =
    BaseRequest<TrainerPartiesRequestType, TrainerPartiesRequestAttributes>;

impl TrainerPartiesRequest {
    pub fn get_parties(&self) -> Vec<Party> {
        self.data
            .attributes
            .parties
            .iter()
            .map(|party| {
                let pokemon = party
                    .party
                    .iter()
                    .map(|party_pokemon| {
                        PartyPokemon::new(party_pokemon.level, party_pokemon.pokemon.id)
                    })
                    .collect();

                Party::new(&pokemon)
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TrainerPartiesRequestType {
    trainer_parties,
}

#[derive(Debug, Deserialize)]
pub struct TrainerPartiesRequestAttributes {
    pub parties: Vec<TrainerPartiesRequestAttributesParty>,
}

#[derive(Debug, Deserialize)]
pub struct TrainerPartiesRequestAttributesParty {
    pub party: Vec<TrainerPartiesRequestAttributesPartyPokemon>,
}

#[derive(Debug, Deserialize)]
pub struct TrainerPartiesRequestAttributesPartyPokemon {
    pub level: u8,
    pub pokemon: TrainerPartiesRequestAttributesPartyPokemonAttributes,
}

#[derive(Debug, Deserialize)]
pub struct TrainerPartiesRequestAttributesPartyPokemonAttributes {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
