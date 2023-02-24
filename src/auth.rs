use crate::get_env;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;

/*
Code Summary:
This module handles the authorization of requests by verifying that the API key in the "x-auth" field of the header is valid. If the key is missing, invalid, or the environment variable containing the key is not set, the request is rejected with an appropriate status code.

Function:

FromRequest: trait that defines how a request is authorized by checking if the x-auth header contains the correct API key. If the key is valid, returns an instance of the ApiKey struct.

Struct:

ApiKey: contains the valid API key as a String.

Enum:

ApiKeyError: contains error types for the FromRequest function.
*/

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount, // too many keys
    Missing,  // no keys
    Invalid,  // invalid key
    NotSet,   // environment variable not set
}

pub struct ApiKey(String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    // Check the x-auth header and compare to the API key in the environment
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-auth").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)), // No API key found in header
            1 => match get_env("API_KEY") {
                Ok(key) if keys[0] == key => Outcome::Success(ApiKey(keys[0].to_string())), // Valid API key
                Ok(_) => Outcome::Failure((Status::Unauthorized, ApiKeyError::Invalid)), // Invalid API key
                Err(_) => Outcome::Failure((Status::InternalServerError, ApiKeyError::NotSet)), // Environment variable not set
            },
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)), // Multiple API keys found in header
        }
    }
}
