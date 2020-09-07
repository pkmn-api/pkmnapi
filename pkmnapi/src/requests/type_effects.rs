use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypeEffectRequest {
    pub data: TypeEffectRequestData,
}

impl TypeEffectRequest {
    pub fn get_attacking_type_id(&self) -> &String {
        &self.data.attributes.attacking_type.id
    }

    pub fn get_defending_type_id(&self) -> &String {
        &self.data.attributes.defending_type.id
    }

    pub fn get_multiplier(&self) -> f32 {
        self.data.attributes.multiplier
    }
}

#[derive(Debug, Deserialize)]
pub struct TypeEffectRequestData {
    #[serde(rename = "type")]
    pub _type: TypeEffectRequestDataType,
    pub attributes: TypeEffectRequestDataAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TypeEffectRequestDataType {
    type_effects,
}

#[derive(Debug, Deserialize)]
pub struct TypeEffectRequestDataAttributes {
    pub attacking_type: TypeEffectRequestDataAttributesType,
    pub defending_type: TypeEffectRequestDataAttributesType,
    pub multiplier: f32,
}

#[derive(Debug, Deserialize)]
pub struct TypeEffectRequestDataAttributesType {
    id: String,
}
