use pkmnapi_db::types::TypeName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TypeResponse = BaseResponse<TypeResponseAttributes>;

impl TypeResponse {
    /// Create a new `TypeResponse`
    pub fn new(type_id: &u8, type_name: &TypeName) -> TypeResponse {
        TypeResponse {
            id: type_id.to_string(),
            _type: BaseResponseType::types,
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
pub struct TypeResponseAttributes {
    pub name: String,
}
