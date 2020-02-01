use actix_web::{web, HttpRequest, HttpResponse, Responder};

async fn test_route(request: HttpRequest) -> impl Responder {
    let name: String = request.match_info().get("name").unwrap_or("Unknown").to_string();
    return HttpResponse::Ok().body(format!("Hello world! {} is here", name));
}

pub fn register_routes(app: &mut web::ServiceConfig) {
    app.route("/test", web::get().to(test_route))
       .route("/test/{name}", web::get().to(test_route));
}