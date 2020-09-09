use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MoveRequest {
    pub data: MoveRequestData,
}

impl MoveRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize)]
pub struct MoveRequestData {
    #[serde(rename = "type")]
    pub _type: MoveRequestDataMove,
    pub attributes: MoveRequestDataAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum MoveRequestDataMove {
    moves,
}

#[derive(Debug, Deserialize)]
pub struct MoveRequestDataAttributes {
    pub name: String,
}
