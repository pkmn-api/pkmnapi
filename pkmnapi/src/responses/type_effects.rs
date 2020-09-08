use pkmnapi_db::types::{TypeEffect, TypeName};
use serde::Serialize;

use crate::responses::links::Links;
use crate::responses::types::TypeResponse;
use crate::utils;

#[derive(Debug, Serialize)]
pub struct TypeEffectResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: TypeEffectResponseType,
    pub attributes: TypeEffectResponseAttributes,
    pub links: Links,
}

impl TypeEffectResponse {
    /// Create a new `TypeEffectResponse`
    pub fn new(
        type_effect_id: &u8,
        type_effect: &TypeEffect,
        attacking_type_name: &TypeName,
        defending_type_name: &TypeName,
    ) -> TypeEffectResponse {
        TypeEffectResponse {
            id: type_effect_id.to_string(),
            _type: TypeEffectResponseType::type_effects,
            attributes: TypeEffectResponseAttributes {
                attacking_type: TypeResponse::new(
                    &type_effect.attacking_type_id.value(),
                    attacking_type_name,
                ),
                defending_type: TypeResponse::new(
                    &type_effect.defending_type_id.value(),
                    defending_type_name,
                ),
                multiplier: type_effect.multiplier,
            },
            links: Links {
                _self: utils::generate_url("type_effects", Some(&type_effect_id.to_string())),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum TypeEffectResponseType {
    type_effects,
}

#[derive(Debug, Serialize)]
pub struct TypeEffectResponseAttributes {
    pub attacking_type: TypeResponse,
    pub defending_type: TypeResponse,
    pub multiplier: f32,
}
