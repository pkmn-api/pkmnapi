use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type SavErrorNoSav = BaseErrorResponse<SavErrorNoSavAttributes>;

impl SavErrorNoSav {
    pub fn new() -> ResponseError {
        let response = SavErrorNoSav {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_savs_no_sav,
                _type: BaseErrorResponseType::errors,
                attributes: SavErrorNoSavAttributes {
                    message: "No SAV uploaded".to_owned(),
                },
            },
        };

        ResponseError::SavErrorNoSav(status::Forbidden(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct SavErrorNoSavAttributes {
    pub message: String,
}
