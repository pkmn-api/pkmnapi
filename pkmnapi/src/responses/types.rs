use pkmnapi_db::types::TypeName;
use serde::Serialize;

use crate::responses::links::Links;
use crate::utils;

#[derive(Debug, Serialize)]
pub struct TypeResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: TypeResponseType,
    pub attributes: TypeResponseAttributes,
    pub links: Links,
}

impl TypeResponse {
    /// Create a new `TypeResponse`
    pub fn new(type_id: &u8, type_name: &TypeName) -> TypeResponse {
        TypeResponse {
            id: type_id.to_string(),
            _type: TypeResponseType::types,
            attributes: TypeResponseAttributes {
                name: type_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("types", Some(&type_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum TypeResponseType {
    types,
}

#[derive(Debug, Serialize)]
pub struct TypeResponseAttributes {
    pub name: String,
}
