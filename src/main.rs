use actix_web::{App, HttpServer};

use std::io::Result;

mod routes;

#[actix_rt::main]
async fn main() -> Result<()> {
    let bind_address = "127.0.0.1:8080";

    HttpServer::new(|| {
        App::new().configure(routes::register_routes)
    }).bind(bind_address)?
      .run()
      .await
}
