use pkmnapi_db::types::{TypeEffect, TypeName};
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseType};
use crate::responses::links::Links;
use crate::responses::types::TypeResponse;
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
            id: type_effect_id.to_string(),
            _type: BaseResponseType::type_effects,
            attributes: TypeEffectResponseAttributes {
                attacking_type: TypeResponse::new(&type_effect.attacking_type_id, type_names[0]),
                defending_type: TypeResponse::new(&type_effect.defending_type_id, type_names[1]),
                multiplier: type_effect.multiplier,
            },
            links: Links {
                _self: utils::generate_url("type_effects", Some(&type_effect_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseAttributes {
    pub attacking_type: TypeResponse,
    pub defending_type: TypeResponse,
    pub multiplier: f32,
}
