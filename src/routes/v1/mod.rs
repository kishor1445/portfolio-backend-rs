pub mod about;

use actix_web::web;

use crate::auth::google;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(google::routes).configure(about::routes);
}
