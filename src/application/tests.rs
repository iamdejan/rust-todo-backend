use std::str;

use actix_web::{test, App};
use serde_json::{Value, Map};

use crate::application::routes;
use crate::domain::entities::Memo;
use crate::infrastructure::forms::AddTODOForm;

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
async fn testing_get_and_insert() {
    let mut app = test::init_service(
        App::new().configure(routes::register_routes)
    ).await;

    let request = test::TestRequest::get().uri("/todos").to_request();
    let result: Vec<Memo> = test::read_response_json(&mut app, request).await;
    let beginning_length = result.len();

    //insert TODO
    let add_form: AddTODOForm = AddTODOForm {
        title: "Victor Robotics Suite".to_string(),
        description: "Create library".to_string()
    };
    let request = test::TestRequest::post().set_form(&add_form).uri("/todos/add").to_request();
    let service_response = test::call_service(&mut app, request).await;
    assert_eq!(200, service_response.status().as_u16());
    let response_body = test::read_body(service_response).await;
    let response_body = str::from_utf8(&response_body).unwrap();
    let parsed: Value = serde_json::from_str(response_body).unwrap();
    let map: Map<String, Value> = parsed.as_object().unwrap().clone();
    assert!(map["$oid"].as_str().unwrap().is_empty() != true);

    let request = test::TestRequest::get().uri("/todos").to_request();
    let result: Vec<Memo> = test::read_response_json(&mut app, request).await;
    let end_length = result.len();

    assert_eq!(1, end_length - beginning_length);
}
