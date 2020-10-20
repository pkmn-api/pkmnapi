use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type MoveStatsRequest = BaseRequest<MoveStatsRequestType, MoveStatsRequestAttributes>;

impl MoveStatsRequest {
    pub fn get_effect(&self) -> u8 {
        self.data.attributes.effect
    }

    pub fn get_power(&self) -> u8 {
        self.data.attributes.power
    }

    pub fn get_type_id(&self) -> u8 {
        self.data.attributes._type.id
    }

    pub fn get_accuracy(&self) -> f32 {
        self.data.attributes.accuracy
    }

    pub fn get_pp(&self) -> u8 {
        self.data.attributes.pp
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MoveStatsRequestType {
    move_stats,
}

#[derive(Debug, Deserialize)]
pub struct MoveStatsRequestAttributes {
    pub effect: u8,
    pub power: u8,

    #[serde(rename = "type")]
    pub _type: MoveStatsRequestAttributesType,
    pub accuracy: f32,
    pub pp: u8,
}

#[derive(Debug, Deserialize)]
pub struct MoveStatsRequestAttributesType {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    pub id: u8,
}
