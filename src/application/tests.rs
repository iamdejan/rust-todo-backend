use actix_web::{test, App};
use crate::db_connection;
use crate::routes::{routes};
use crate::model::memo::Memo;
use std::str;
use mongodb::{Client, options::ClientOptions};

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
    let mut app = test::init_service(App::new().configure(routes::register_routes)).await;
    let request = test::TestRequest::with_uri("/test").to_request();
    let response = test::call_service(&mut app, request).await;
    assert!(response.status().is_success());

    let data = test::read_body(response).await;
    assert_eq!("Hello world! Unknown is here", str::from_utf8(&data).unwrap());
}

#[actix_rt::test]
async fn testing_hello_world_route_with_name() {
    let mut app = test::init_service(App::new().configure(routes::register_routes)).await;
    let request = test::TestRequest::with_uri("/test/Dejan").param("name", "Dejan").to_request();
    let response = test::call_service(&mut app, request).await;
    assert!(response.status().is_success());

    let data = test::read_body(response).await;
    assert_eq!("Hello world! Dejan is here", str::from_utf8(&data).unwrap());
}

#[allow(non_snake_case)]
#[actix_rt::test]
async fn testing_get_all_TODOs() {
    //TODO: use mock database
    let mut app = test::init_service(
        App::new().configure(routes::register_routes)
    ).await;
    let request = test::TestRequest::get().uri("/todos").to_request();
    let result: Vec<Memo> = test::read_response_json(&mut app, request).await;

    //TODO: change this test
    assert_eq!(1, result.len());
}