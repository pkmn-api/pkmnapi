use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TypeEffectRequest = BaseRequest<TypeEffectRequestType, TypeEffectRequestAttributes>;

impl TypeEffectRequest {
    pub fn get_attacking_type_id(&self) -> u8 {
        self.data.attributes.attacking_type.id
    }

    pub fn get_defending_type_id(&self) -> u8 {
        self.data.attributes.defending_type.id
    }

    pub fn get_multiplier(&self) -> f32 {
        self.data.attributes.multiplier
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TypeEffectRequestType {
    type_effects,
}

#[derive(Debug, Deserialize)]
pub struct TypeEffectRequestAttributes {
    pub attacking_type: TypeEffectRequestAttributesType,
    pub defending_type: TypeEffectRequestAttributesType,
    pub multiplier: f32,
}

#[derive(Debug, Deserialize)]
pub struct TypeEffectRequestAttributesType {
    #[serde(deserialize_with = "crate::utils::from_numeric_str")]
    id: u8,
}
