use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavResponseErrorNoSav = BaseErrorResponse<SavResponseErrorNoSavAttributes>;

impl SavResponseErrorNoSav {
    pub fn new() -> ResponseError {
        let response = SavResponseErrorNoSav {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_no_sav,
                _type: BaseErrorResponseType::errors,
                attributes: SavResponseErrorNoSavAttributes {
                    message: "No SAV uploaded".to_owned(),
                },
            },
        };

        ResponseError::SavResponseErrorNoSav(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavResponseErrorNoSavAttributes {
    pub message: String,
}
