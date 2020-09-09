use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TMRequest {
    pub data: TMRequestData,
}

impl TMRequest {
    pub fn get_move_id(&self) -> &String {
        &self.data.attributes._move.id
    }
}

#[derive(Debug, Deserialize)]
pub struct TMRequestData {
    #[serde(rename = "type")]
    pub _type: TMRequestDataType,
    pub attributes: TMRequestDataAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TMRequestDataType {
    tms,
}

#[derive(Debug, Deserialize)]
pub struct TMRequestDataAttributes {
    #[serde(rename = "move")]
    pub _move: TMRequestDataAttributesMove,
}

#[derive(Debug, Deserialize)]
pub struct TMRequestDataAttributesMove {
    pub id: String,
}
