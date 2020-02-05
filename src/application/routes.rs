use actix_web::{web, HttpRequest, HttpResponse};

use crate::domain::entities::Memo;
use crate::domain::handlers::MemoHandler;

pub async fn route_initial_test(request: HttpRequest) -> HttpResponse {
    let name = request.match_info().get("name").unwrap_or("Unknown");
    return HttpResponse::Ok().body(format!("Hello world! {} is here", name));
}

#[allow(non_snake_case)]
pub async fn get_all_TODOs(memo_handler: web::Data<MemoHandler<'_>>) -> HttpResponse {
    let response_data: Vec<Memo> = memo_handler.get_all();
    return HttpResponse::Ok().json(response_data);
}

pub fn register_routes(app: &mut web::ServiceConfig) {
    app.route("/test", web::get().to(route_initial_test))
       .route("/test/{name}", web::get().to(route_initial_test))
       .route("/todos", web::get().to(get_all_TODOs));
}