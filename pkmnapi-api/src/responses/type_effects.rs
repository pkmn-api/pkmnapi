use pkmnapi_db::{TypeEffect, TypeName};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::type_names::TypeNameResponseData;
use crate::utils;

pub type TypeEffectResponse = BaseResponse<TypeEffectResponseAttributes>;

impl TypeEffectResponse {
    /// Create a new `TypeEffectResponse`
    pub fn new(
        type_effect_id: &u8,
        type_effect: &TypeEffect,
        type_names: Vec<&TypeName>,
    ) -> TypeEffectResponse {
        TypeEffectResponse {
            data: BaseResponseData {
                id: type_effect_id.to_string(),
                _type: BaseResponseType::type_effects,
                attributes: TypeEffectResponseAttributes {
                    attacking_type: TypeNameResponseData::new(
                        &type_effect.attacking_type_id,
                        type_names[0],
                    ),
                    defending_type: TypeNameResponseData::new(
                        &type_effect.defending_type_id,
                        type_names[1],
                    ),
                    multiplier: type_effect.multiplier,
                },
                links: Links {
                    _self: utils::generate_url("types/effects", Some(&type_effect_id.to_string())),
                },
            },
            links: Links {
                _self: utils::generate_url("types/effects", Some(&type_effect_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseAttributes {
    pub attacking_type: TypeNameResponseData,
    pub defending_type: TypeNameResponseData,
    pub multiplier: f32,
}
