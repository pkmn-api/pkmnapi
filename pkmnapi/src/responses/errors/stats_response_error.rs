use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type StatsResponseError = BaseErrorResponse<StatsResponseErrorAttributes>;

impl StatsResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = StatsResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_stats,
                _type: BaseErrorResponseType::errors,
                attributes: StatsResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::StatsResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct StatsResponseErrorAttributes {
    pub message: String,
}
