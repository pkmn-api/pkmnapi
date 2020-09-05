use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalid {
    pub data: AccessTokenErrorInvalidData,
}

impl AccessTokenErrorInvalid {
    pub fn new(message: &String) -> Self {
        AccessTokenErrorInvalid {
            data: AccessTokenErrorInvalidData {
                id: String::from("error_access_tokens_invalid"),
                _type: AccessTokenRequestDataType::errors,
                attributes: AccessTokenErrorInvalidDataAttributes {
                    message: message.to_string(),
                },
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalidData {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: AccessTokenRequestDataType,
    pub attributes: AccessTokenErrorInvalidDataAttributes,
}

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum AccessTokenRequestDataType {
    errors,
}

#[derive(Debug, Serialize)]
pub struct AccessTokenErrorInvalidDataAttributes {
    pub message: String,
}
