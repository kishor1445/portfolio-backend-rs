use std::rc::Rc;

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::{LocalBoxFuture, Ready, ok};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub email: String,
    pub name: String,
    pub picture: String,
}

pub struct Auth;

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .map(|t| t.to_string());


        let key = std::env::var("JWT_SECRET").expect("JWT_SECRET not set");

        if let Some(token) = token {
            match decode::<Claims>(
                &token,
                &DecodingKey::from_secret(key.as_bytes()),
                &Validation::default(),
            ) {
                Ok(data) => {
                    req.extensions_mut().insert(data.claims);
                    return Box::pin(self.service.call(req));
                }
                Err(_) => {}
            }
        }

        Box::pin(async { Err(actix_web::error::ErrorUnauthorized("Unauthorized")) })
    }
}
