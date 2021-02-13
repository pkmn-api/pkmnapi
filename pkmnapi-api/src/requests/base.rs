use rocket_okapi::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BaseRequest<T, U> {
    pub data: BaseRequestData<T, U>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BaseRequestData<T, U> {
    #[serde(rename = "type")]
    pub _type: T,
    pub attributes: U,
}
