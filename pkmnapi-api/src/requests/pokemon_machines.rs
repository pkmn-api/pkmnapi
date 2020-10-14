use pkmnapi_db::PokemonMachine;
use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type PokemonMachinesRequest =
    BaseRequest<PokemonMachinesRequestType, PokemonMachinesRequestAttributes>;

impl PokemonMachinesRequest {
    pub fn get_machines(&self) -> Vec<PokemonMachine> {
        self.data
            .attributes
            .machines
            .iter()
            .map(|machine| match machine._type {
                PokemonMachinesRequestAttributesMachineType::tm_moves => {
                    PokemonMachine::TM(machine.id)
                }
                PokemonMachinesRequestAttributesMachineType::hm_moves => {
                    PokemonMachine::HM(machine.id)
                }
            })
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonMachinesRequestType {
    pokemon_machines,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMachinesRequestAttributes {
    machines: Vec<PokemonMachinesRequestAttributesMachine>,
}

#[derive(Debug, Deserialize)]
pub struct PokemonMachinesRequestAttributesMachine {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,

    #[serde(rename = "type")]
    pub _type: PokemonMachinesRequestAttributesMachineType,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum PokemonMachinesRequestAttributesMachineType {
    tm_moves,
    hm_moves,
}
