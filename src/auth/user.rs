use actix_web::FromRequest;
use futures::future::{Ready, ready};
use serde::Serialize;
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{auth::google::Claims, error::APIError};

#[derive(Debug, Serialize)]
pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Extract token from Authorization header
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|t| t.to_string());

        if let Some(token) = token {
            // Get JWT secret from environment
            let key = match std::env::var("JWT_SECRET") {
                Ok(secret) => secret,
                Err(_) => {
                    return ready(Err(APIError::InternalServerError.into()));
                }
            };

            // Validate and decode the JWT token
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(key.as_bytes()),
                &Validation::default(),
            ) {
                Ok(token_data) => {
                    return ready(Ok(AuthenticatedUser(token_data.claims)));
                }
                Err(_) => {
                    return ready(Err(APIError::Unauthorized.into()));
                }
            }
        }

        ready(Err(APIError::Unauthorized.into()))
    }
}
