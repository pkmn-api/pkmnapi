use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StatsRequest {
    pub data: StatsRequestData,
}

impl StatsRequest {
    pub fn get_base_hp(&self) -> u8 {
        self.data.attributes.base_hp
    }

    pub fn get_base_attack(&self) -> u8 {
        self.data.attributes.base_attack
    }

    pub fn get_base_defence(&self) -> u8 {
        self.data.attributes.base_defence
    }

    pub fn get_base_speed(&self) -> u8 {
        self.data.attributes.base_speed
    }

    pub fn get_base_special(&self) -> u8 {
        self.data.attributes.base_special
    }

    pub fn get_type_ids(&self) -> Vec<&String> {
        self.data
            .attributes
            .types
            .iter()
            .map(|_type| &_type.id)
            .collect()
    }

    pub fn get_catch_rate(&self) -> u8 {
        self.data.attributes.catch_rate
    }

    pub fn get_base_exp_yield(&self) -> u8 {
        self.data.attributes.base_exp_yield
    }
}

#[derive(Debug, Deserialize)]
pub struct StatsRequestData {
    #[serde(rename = "type")]
    pub _type: StatsRequestDataType,
    pub attributes: StatsRequestDataAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum StatsRequestDataType {
    stats,
}

#[derive(Debug, Deserialize)]
pub struct StatsRequestDataAttributes {
    pub base_hp: u8,
    pub base_attack: u8,
    pub base_defence: u8,
    pub base_speed: u8,
    pub base_special: u8,
    pub types: Vec<StatsRequestDataAttributesType>,
    pub catch_rate: u8,
    pub base_exp_yield: u8,
}

#[derive(Debug, Deserialize)]
pub struct StatsRequestDataAttributesType {
    id: String,
}
