use actix_web::test;

use crate::routes::{routes};

#[test]
fn testing_true() {
    assert!(true)
}

#[test]
fn testing_false() {
    assert_ne!(true, false)
}

#[actix_rt::test]
async fn testing_hello_world_route() {
    let request = test::TestRequest::with_uri("/test").to_http_request();
    let response = routes::test_route(request).await;
    assert!(response.status().is_success());
}