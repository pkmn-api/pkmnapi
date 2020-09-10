use pkmnapi_db::types::TypeName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TypeResponse = BaseResponse<TypeResponseAttributes>;
pub type TypeResponseData = BaseResponseData<TypeResponseAttributes>;

impl TypeResponse {
    /// Create a new `TypeResponse`
    pub fn new(type_id: &u8, type_name: &TypeName) -> TypeResponse {
        TypeResponse {
            data: TypeResponseData::new(type_id, type_name),
            links: Links {
                _self: utils::generate_url("types", Some(&type_id.to_string())),
            },
        }
    }
}

impl TypeResponseData {
    pub fn new(type_id: &u8, type_name: &TypeName) -> TypeResponseData {
        BaseResponseData {
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
