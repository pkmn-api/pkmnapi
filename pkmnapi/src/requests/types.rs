use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TypeRequest {
    pub data: TypeRequestData,
}

impl TypeRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize)]
pub struct TypeRequestData {
    #[serde(rename = "type")]
    pub _type: TypeRequestDataType,
    pub attributes: TypeRequestDataAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TypeRequestDataType {
    types,
}

#[derive(Debug, Deserialize)]
pub struct TypeRequestDataAttributes {
    pub name: String,
}
