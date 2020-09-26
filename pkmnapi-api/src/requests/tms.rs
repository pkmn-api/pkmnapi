use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TMRequest = BaseRequest<TMRequestType, TMRequestAttributes>;

impl TMRequest {
    pub fn get_move_id(&self) -> &String {
        &self.data.attributes._move.id
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TMRequestType {
    tms,
}

#[derive(Debug, Deserialize)]
pub struct TMRequestAttributes {
    #[serde(rename = "move")]
    pub _move: TMRequestAttributesMove,
}

#[derive(Debug, Deserialize)]
pub struct TMRequestAttributesMove {
    pub id: String,
}
