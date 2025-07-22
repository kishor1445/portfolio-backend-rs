use actix_web::{test, App};

use portfolio_backend::routes;

#[actix_rt::test]
async fn test_get_about_returns_not_implemented() {
    let app = test::init_service(
        App::new().configure(routes::config)
    ).await;

    let req = test::TestRequest::get()
        .uri("/v1/about")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 501); // Not Implemented
}