use pkmnapi_db::TypeName;
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type TypeNameResponse = BaseResponse<TypeNameResponseAttributes>;
pub type TypeNameResponseData = BaseResponseData<TypeNameResponseAttributes>;
pub type TypeNameResponseAll = BaseResponseAll<TypeNameResponseData>;

impl TypeNameResponseAll {
    pub fn new(type_ids: &Vec<u8>, type_names: &HashMap<u8, TypeName>) -> TypeNameResponseAll {
        TypeNameResponseAll {
            data: type_ids
                .iter()
                .map(|type_id| TypeNameResponseData::new(type_id, type_names.get(type_id).unwrap()))
                .collect(),
            links: Links {
                _self: utils::generate_url("types/names", None),
            },
        }
    }
}

impl TypeNameResponse {
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

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypeNameResponseAttributes {
    pub name: String,
}
