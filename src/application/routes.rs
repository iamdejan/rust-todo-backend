use actix_web::{web, HttpRequest, HttpResponse};

use crate::domain::handlers::MemoHandler;

pub async fn test_route(request: HttpRequest) -> HttpResponse {
    let name = request.match_info().get("name").unwrap_or("Unknown");
    return HttpResponse::Ok().body(format!("Hello world! {} is here", name));
}

#[allow(non_snake_case)]
pub async fn get_all_TODOs() -> HttpResponse {
    let memo_handler: MemoHandler = MemoHandler::new();
    let result = memo_handler.get_all();
    if result.is_ok() != true {
        return HttpResponse::InternalServerError().body("");
    }

    return HttpResponse::Ok().json(result.unwrap);
}

pub fn register_routes(app: &mut web::ServiceConfig) {
    app.route("/test", web::get().to(test_route))
       .route("/test/{name}", web::get().to(test_route))
       .route("/todos", web::get().to(get_all_TODOs));
}