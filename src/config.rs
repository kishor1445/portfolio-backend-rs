use anyhow::Result;
use once_cell::sync::OnceCell;
use std::env;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
};

static DB: OnceCell<Surreal<Client>> = OnceCell::new();

pub fn get_db() -> &'static Surreal<Client> {
    DB.get().expect("SurrealDB is not initialized")
}

pub async fn init_db() -> Result<()> {
    let surreal_uri = env::var("SURREAL_URI").expect("SURREAL_URI must be set");
    let user = env::var("SURREAL_USER").expect("SURREAL_USER must be set");
    let pass = env::var("SURREAL_PASS").expect("SURREAL_PASS must be set");
    let ns = env::var("SURREAL_NS").expect("SURREAL_NS must be set");
    let db_name = env::var("SURREAL_DB").unwrap_or_else(|_| "portfolio".to_string());

    let surreal = Surreal::new::<Wss>(surreal_uri).await?;

    surreal
        .signin(Root {
            username: &user,
            password: &pass,
        })
        .await?;

    surreal.use_ns(ns).use_db(db_name).await?;

    DB.set(surreal).ok();

    tracing::info!("✅ Connected to SurrealDB");
    Ok(())
}
