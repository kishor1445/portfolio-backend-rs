pub mod about;

use actix_web::web;

use crate::auth::{google, middleware::Auth};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.configure(google::routes).configure(about::routes);

    cfg.service(web::scope("").wrap(Auth).configure(about::protected_routes));
}
