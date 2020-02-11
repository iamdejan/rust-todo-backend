use actix_web::{App, HttpServer};
use std::io::Result;

extern crate serde;

#[macro_use]
extern crate serde_derive;

mod application;
mod domain;
mod infrastructure;
mod db_connection;

#[actix_rt::main]
async fn main() -> Result<()> {
    let bind_address = "127.0.0.1:8080";
    HttpServer::new(move || {
        println!("Starting Actix-Web server...");
        return App::new().configure(application::routes::register_routes);
    }).bind(bind_address)
      .expect("Fail to run server!")
      .run()
      .await
}