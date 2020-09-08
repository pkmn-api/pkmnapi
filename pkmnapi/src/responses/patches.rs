use pkmnapi_sql::models::Patch;
use serde::Serialize;

use crate::responses::links::Links;

#[derive(Debug, Serialize)]
pub struct PatchesResponse {
    data: Vec<PatchResponse>,
    links: Links,
}

impl PatchesResponse {
    /// Create a new `PatchesResponse`
    pub fn new(patches: &Vec<Patch>) -> PatchesResponse {
        PatchesResponse {
            data: patches
                .iter()
                .map(|patch| PatchResponse::new(patch))
                .collect(),
            links: Links {
                _self: "foo".to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PatchResponse {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: PatchResponseType,
    pub attributes: PatchResponseAttributes,
    pub links: Links,
}

impl PatchResponse {
    /// Create a new `PatchResponse`
    pub fn new(patch: &Patch) -> PatchResponse {
        PatchResponse {
            id: patch.id.to_owned(),
            _type: PatchResponseType::patches,
            attributes: PatchResponseAttributes {
                description: match &patch.description {
                    Some(description) => Some(description.to_owned()),
                    None => None,
                },
            },
            links: Links {
                _self: "foo".to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum PatchResponseType {
    patches,
}

#[derive(Debug, Serialize)]
pub struct PatchResponseAttributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}
