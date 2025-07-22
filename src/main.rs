use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use dotenvy::dotenv;
use std::env;
use tracing_subscriber::FmtSubscriber;

use crate::error::APIError;

mod auth;
mod config;
mod error;
mod models;
mod routes;
mod services;

async fn root() -> impl Responder {
    HttpResponse::Ok().body("Kishor's Portfolio Backend API")
}

async fn not_found() -> Result<HttpResponse, APIError> {
    Err(APIError::NotFound)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global tracing subscriber");

    if let Err(e) = config::init_db().await {
        tracing::error!("❌ Failed to init DB: {:?}", e);
        std::process::exit(1);
    }

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    tracing::info!("🚀 Server starting at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(root))
            .configure(routes::config)
            .default_service(web::route().to(not_found))
    })
    .bind((host, port))?
    .run()
    .await
}
