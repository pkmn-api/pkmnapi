use pkmnapi_db::TypeName;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TypeNameResponse = BaseResponse<TypeNameResponseAttributes>;
pub type TypeNameResponseData = BaseResponseData<TypeNameResponseAttributes>;

impl TypeNameResponse {
    /// Create a new `TypeNameResponse`
    pub fn new(type_id: &u8, type_name: &TypeName) -> TypeNameResponse {
        TypeNameResponse {
            data: TypeNameResponseData::new(type_id, type_name),
            links: Links {
                _self: utils::generate_url("types/names", Some(&type_id.to_string())),
            },
        }
    }
}

impl TypeNameResponseData {
    pub fn new(type_id: &u8, type_name: &TypeName) -> TypeNameResponseData {
        BaseResponseData {
            id: type_id.to_string(),
            _type: BaseResponseType::type_names,
            attributes: TypeNameResponseAttributes {
                name: type_name.name.to_string(),
            },
            links: Links {
                _self: utils::generate_url("types/names", Some(&type_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TypeNameResponseAttributes {
    pub name: String,
}
