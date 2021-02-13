use pkmnapi_db::{TypeEffect, TypeName};
use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::responses::base::{BaseResponse, BaseResponseAll, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::type_names::TypeNameResponseData;
use crate::utils;

pub type TypeEffectResponse = BaseResponse<TypeEffectResponseAttributes>;
pub type TypeEffectResponseData = BaseResponseData<TypeEffectResponseAttributes>;
pub type TypeEffectResponseAll = BaseResponseAll<TypeEffectResponseData>;

impl TypeEffectResponseAll {
    pub fn new(
        type_effect_ids: &Vec<u8>,
        type_effects: &HashMap<u8, TypeEffect>,
        type_names: &HashMap<u8, TypeName>,
    ) -> TypeEffectResponseAll {
        TypeEffectResponseAll {
            data: type_effect_ids
                .iter()
                .map(|type_effect_id| {
                    TypeEffectResponseData::new(
                        type_effect_id,
                        type_effects.get(type_effect_id).unwrap(),
                        type_names,
                    )
                })
                .collect(),
            links: Links {
                _self: utils::generate_url("types/effects", None),
            },
        }
    }
}

impl TypeEffectResponse {
    pub fn new(
        type_effect_id: &u8,
        type_effect: &TypeEffect,
        type_names: &HashMap<u8, TypeName>,
    ) -> TypeEffectResponse {
        TypeEffectResponse {
            data: TypeEffectResponseData::new(type_effect_id, type_effect, type_names),
            links: Links {
                _self: utils::generate_url("types/effects", Some(&type_effect_id.to_string())),
            },
        }
    }
}

impl TypeEffectResponseData {
    pub fn new(
        type_effect_id: &u8,
        type_effect: &TypeEffect,
        type_names: &HashMap<u8, TypeName>,
    ) -> TypeEffectResponseData {
        BaseResponseData {
            id: type_effect_id.to_string(),
            _type: BaseResponseType::type_effects,
            attributes: TypeEffectResponseAttributes {
                attacking_type: TypeNameResponseData::new(
                    &type_effect.attacking_type_id,
                    &type_names.get(&type_effect.attacking_type_id).unwrap(),
                ),
                defending_type: TypeNameResponseData::new(
                    &type_effect.defending_type_id,
                    &type_names.get(&type_effect.defending_type_id).unwrap(),
                ),
                multiplier: type_effect.multiplier,
            },
            links: Links {
                _self: utils::generate_url("types/effects", Some(&type_effect_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypeEffectResponseAttributes {
    pub attacking_type: TypeNameResponseData,
    pub defending_type: TypeNameResponseData,
    pub multiplier: f32,
}
