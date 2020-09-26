use rocket::response::status;
use rocket_contrib::json::Json;
use serde::Serialize;

use crate::responses::errors::*;

pub type MapPicResponseError = BaseErrorResponse<MapPicResponseErrorAttributes>;

impl MapPicResponseError {
    pub fn new(message: &String) -> ResponseError {
        let response = MapPicResponseError {
            data: BaseErrorResponseData {
                id: BaseErrorResponseId::error_map_pics,
                _type: BaseErrorResponseType::errors,
                attributes: MapPicResponseErrorAttributes {
                    message: message.to_owned(),
                },
            },
        };

        ResponseError::MapPicResponseError(status::NotFound(Json(response)))
    }
}

#[derive(Debug, Serialize)]
pub struct MapPicResponseErrorAttributes {
    pub message: String,
}
