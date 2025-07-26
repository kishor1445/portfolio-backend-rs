use actix_web::{HttpResponse, Result, get, web};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use std::env;

use crate::error::APIError;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_USERINFO_URL: &str = "https://www.googleapis.com/oauth2/v2/userinfo";

#[derive(Deserialize)]
pub struct AuthRequest {
    pub code: String,
}

#[derive(Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(Deserialize, Serialize)]
struct GoogleUser {
    email: String,
    name: String,
    picture: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    pub email: String,
    pub name: String,
    pub picture: String,
    pub exp: usize,
}

#[derive(Serialize)]
struct JwtResponse {
    token: String,
}

#[get("/auth/google/login")]
pub async fn google_login() -> HttpResponse {
    let client_id = env::var("GOOGLE_CLIENT_ID").unwrap();
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").unwrap();

    let url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope=email%20profile",
        GOOGLE_AUTH_URL, client_id, redirect_uri
    );

    HttpResponse::Found()
        .append_header(("Location", url))
        .finish()
}

#[get("/auth/google/callback")]
pub async fn google_callback(
    web::Query(info): web::Query<AuthRequest>,
) -> Result<HttpResponse, APIError> {
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID not set");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET not set");
    let redirect_uri = env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI not set");

    let params = [
        ("client_id", client_id.as_str()),
        ("client_secret", client_secret.as_str()),
        ("code", &info.code),
        ("grant_type", "authorization_code"),
        ("redirect_uri", &redirect_uri),
    ];

    let client = reqwest::Client::new();
    let token_resp = client
        .post(GOOGLE_TOKEN_URL)
        .form(&params)
        .send()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    let token: GoogleTokenResponse = token_resp
        .json()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    let user_resp = client
        .get(GOOGLE_USERINFO_URL)
        .bearer_auth(&token.access_token)
        .send()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    let user: GoogleUser = user_resp
        .json()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    if user.email != env::var("ALLOWED_EMAIL").expect("ALLOWED_EMAIL not set") {
        return Err(APIError::Unauthorized);
    }

    let expiration = Utc::now()
    .checked_add_signed(Duration::hours(5))
    .expect("valid timestamp")
    .timestamp() as usize;

    
    let claims = Claims {
        email: user.email,
        name: user.name,
        picture: user.picture,
        exp: expiration,
    };

    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            env::var("JWT_SECRET")
                .expect("JWT_SECRET not set")
                .as_bytes(),
        ),
    )
    .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(JwtResponse { token: jwt }))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(google_login).service(google_callback);
}
