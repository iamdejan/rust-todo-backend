extern crate serde;

#[macro_use]
extern crate serde_derive;

mod application;
mod domain;
mod infrastructure;

use std::io::Result;

use actix_web::{App, HttpServer};
use mongodb::{Client, options::ClientOptions};

use domain::repositories::MemoRepository;
use infrastructure::repositories::PersistentMemoRepository;

use domain::handlers::MemoHandler;

fn initiate_mongodb(db_url: &str) -> Client {
    let mut client_options: ClientOptions = ClientOptions::parse(db_url).unwrap();
    client_options.app_name = Some("todo_backend".to_string());
    let client = Client::with_options(client_options).unwrap();
    return client;
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let bind_address = "127.0.0.1:8080";

    HttpServer::new(|| {
        let memo_repository: &'static dyn MemoRepository = &PersistentMemoRepository::new(initiate_mongodb("mongodb://localhost:27017"));
        println!("Starting Actix-Web server...");
        return App::new().data(MemoHandler::new(memo_repository)).configure(application::routes::register_routes);
    }).bind(bind_address)
      .expect("Fail to run server!")
      .run()
      .await
}