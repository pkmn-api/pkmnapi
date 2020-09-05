use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

#[derive(Debug, PartialEq)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum AccessTokenError {
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for AccessToken {
    type Error = AccessTokenError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let access_tokens: Vec<&'a str> = request.headers().get("authorization").collect();

        let access_token = match access_tokens.get(0) {
            Some(access_token) => access_token.to_string(),
            None => return Outcome::Failure((Status::BadRequest, AccessTokenError::Missing)),
        };

        if access_token.find("Bearer ") != Some(0) {
            return Outcome::Failure((Status::BadRequest, AccessTokenError::Invalid));
        }

        let access_token = access_token.replace("Bearer ", "");
        let access_token = AccessToken(access_token);

        return Outcome::Success(access_token);
    }
}
