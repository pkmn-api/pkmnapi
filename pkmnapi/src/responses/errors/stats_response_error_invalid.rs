use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type StatsResponseErrorInvalid = BaseErrorResponse<StatsResponseErrorInvalidAttributes>;

impl StatsResponseErrorInvalid {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseErrorInvalid {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_stats_invalid,
                _type: BaseErrorResponseType::errors,
                attributes: StatsResponseErrorInvalidAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseErrorInvalid(status::BadRequest(Some(Json(response))))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorInvalidAttributes {
    pub message: String,
}
