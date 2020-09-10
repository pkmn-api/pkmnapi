use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TypeRequest = BaseRequest<TypeRequestType, TypeRequestAttributes>;

impl TypeRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TypeRequestType {
    types,
}

#[derive(Debug, Deserialize)]
pub struct TypeRequestAttributes {
    pub name: String,
}
