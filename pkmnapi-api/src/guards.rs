use governor::clock::{Clock, DefaultClock, QuantaClock};
use governor::state::keyed::HashMapStateStore;
use governor::RateLimiter;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;

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
            None => return Outcome::Failure((Status::Unauthorized, AccessTokenError::Missing)),
        };

        if access_token.find("Bearer ") != Some(0) {
            return Outcome::Failure((Status::Unauthorized, AccessTokenError::Invalid));
        }

        let access_token = access_token.replace("Bearer ", "");
        let access_token = AccessToken(access_token);

        return Outcome::Success(access_token);
    }
}

#[derive(Debug, PartialEq)]
pub struct IfMatch(String);

impl IfMatch {
    pub fn into_inner(self) -> String {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum IfMatchError {
    Missing,
}

impl<'a, 'r> FromRequest<'a, 'r> for IfMatch {
    type Error = IfMatchError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let if_matches: Vec<&'a str> = request.headers().get("if-match").collect();

        let if_match = match if_matches.get(0) {
            Some(if_match) => if_match.to_string(),
            None => return Outcome::Failure((Status::Unauthorized, IfMatchError::Missing)),
        };

        let if_match = IfMatch(if_match);

        return Outcome::Success(if_match);
    }
}

#[derive(Debug, PartialEq)]
pub struct PatchDescription(Option<String>);

impl PatchDescription {
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

#[derive(Debug, PartialEq)]
pub enum PatchDescriptionError {
    Missing,
}

impl<'a, 'r> FromRequest<'a, 'r> for PatchDescription {
    type Error = PatchDescriptionError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let patch_descriptions: Vec<&'a str> =
            request.headers().get("x-patch-description").collect();

        let patch_description = match patch_descriptions.get(0) {
            Some(patch_description) => Some(patch_description.to_string()),
            None => None,
        };

        let patch_description = PatchDescription(patch_description);

        return Outcome::Success(patch_description);
    }
}

#[derive(Debug, PartialEq)]
pub struct RateLimit;

#[derive(Debug, PartialEq)]
pub enum RateLimitError {
    Unknown,
    TooManyRequests(u64),
}

impl<'a, 'r> FromRequest<'a, 'r> for RateLimit {
    type Error = RateLimitError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<RateLimit, Self::Error> {
        let lim = match request
            .guard::<State<RateLimiter<String, HashMapStateStore<String>, DefaultClock>>>()
        {
            Outcome::Success(lim) => lim,
            _ => return Outcome::Failure((Status::InternalServerError, RateLimitError::Unknown)),
        };
        let ip = match request.client_ip() {
            Some(ip) => ip.to_string(),
            None => "unknown".to_owned(),
        };

        if let Err(e) = lim.check_key(&ip) {
            let clock = QuantaClock::default();
            let wait_time = e.wait_time_from(clock.now()).as_secs();

            Outcome::Failure((
                Status::TooManyRequests,
                RateLimitError::TooManyRequests(wait_time),
            ))
        } else {
            Outcome::Success(RateLimit)
        }
    }
}
