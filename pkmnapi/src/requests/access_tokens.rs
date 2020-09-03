use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AccessTokenRequest {
    pub data: AccessTokenRequestData,
}

impl AccessTokenRequest {
    pub fn get_email_address(&self) -> &String {
        &self.data.attributes.email_address
    }
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenRequestData {
    #[serde(rename = "type")]
    pub _type: AccessTokenRequestDataType,
    pub attributes: AccessTokenRequestDataAttributes,
}

#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AccessTokenRequestDataType {
    access_tokens,
}

#[derive(Debug, Deserialize)]
pub struct AccessTokenRequestDataAttributes {
    pub email_address: String,
}
