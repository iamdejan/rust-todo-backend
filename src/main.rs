use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use std::io::Result;

async fn greet(request: HttpRequest) -> impl Responder {
    let name: String = request.match_info().get("name").unwrap_or("Unknown").to_string();
    return format!("Hello world! {} is here", name);
}

#[actix_rt::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new().route("/", web::get().to(greet))
                  .route("/{name}", web::get().to(greet))
    }).bind("127.0.0.1:8080")?
      .run()
      .await
}
