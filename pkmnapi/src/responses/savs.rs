use pkmnapi_sql::models::Sav;
use serde::Serialize;

use crate::responses::base::{BaseResponse, BaseResponseData, BaseResponseType};
use crate::responses::links::Links;
use crate::utils;

pub type SavResponse = BaseResponse<SavResponseAttributes>;

impl SavResponse {
    /// Create a new `SavResponse`
    pub fn new(sav: &Sav) -> SavResponse {
        SavResponse {
            data: BaseResponseData {
                id: sav.id.to_owned(),
                _type: BaseResponseType::savs,
                attributes: SavResponseAttributes {},
                links: Links {
                    _self: utils::generate_url("savs", Some(&sav.id)),
                },
            },
            links: Links {
                _self: utils::generate_url("savs", Some(&sav.id)),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SavResponseAttributes {}
