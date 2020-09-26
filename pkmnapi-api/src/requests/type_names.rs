use serde::Deserialize;

use crate::requests::base::BaseRequest;

pub type TypeNameRequest = BaseRequest<TypeNameRequestType, TypeNameRequestAttributes>;

impl TypeNameRequest {
    pub fn get_name(&self) -> &String {
        &self.data.attributes.name
    }
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum TypeNameRequestType {
    type_names,
}

#[derive(Debug, Deserialize)]
pub struct TypeNameRequestAttributes {
    pub name: String,
}
