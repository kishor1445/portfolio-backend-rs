use actix_web::{FromRequest, HttpMessage};
use futures::future::{Ready, ready};

use crate::{auth::middleware::Claims, error::APIError};

#[derive(Debug)]
pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(claims) = req.extensions().get::<Claims>() {
            return ready(Ok(AuthenticatedUser(claims.clone())));
        }

        ready(Err(APIError::Unauthorized.into()))
    }
}
