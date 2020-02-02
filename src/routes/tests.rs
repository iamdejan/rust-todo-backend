use actix_web::test;
use crate::routes::{routes};
use actix_web::body::Body::*;
use std::str;

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

    let body = response.body().as_ref().unwrap();
    match body {
        None => panic!(),
        Empty => panic!(),
        Message(_) => panic!(),
        Bytes(v) => {
            assert_eq!(str::from_utf8(v).unwrap(), "Hello world! Unknown is here");
        }
    }
}

#[actix_rt::test]
async fn testing_hello_world_route_with_name() {
    let request = test::TestRequest::with_uri("/test/Dejan").to_http_request();
    let response = routes::test_route(request).await;
    assert!(response.status().is_success());

    let body = response.body().as_ref().unwrap();
    match body {
        None => panic!(),
        Empty => panic!(),
        Message(_) => panic!(),
        Bytes(v) => {
            assert_eq!(str::from_utf8(v).unwrap(), "Hello world! Unknown is here");
        }
    }
}