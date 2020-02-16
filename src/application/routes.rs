use actix_web::{web, HttpRequest, HttpResponse};

use crate::domain::handlers::PersistentMemoHandler;

use crate::infrastructure::forms::AddTODOForm;

pub async fn test_route(request: HttpRequest) -> HttpResponse {
    let name = request.match_info().get("name").unwrap_or("Unknown");
    return HttpResponse::Ok().body(format!("Hello world! {} is here", name));
}

#[allow(non_snake_case)]
pub async fn get_all_TODOs() -> HttpResponse {
    let memo_handler = PersistentMemoHandler::new();
    let result = memo_handler.get_all();
    match result {
        Ok(v) => {
            return HttpResponse::Ok().json(v);
        },
        Err(e) => {
            return HttpResponse::InternalServerError().body(e);
        }
    }
}

#[allow(non_snake_case)]
pub async fn add_TODO(form: web::Form<AddTODOForm>) -> HttpResponse {
    let add_form: AddTODOForm = form.into_inner();
    let memo_handler = PersistentMemoHandler::new();
    let result = memo_handler.insert(add_form);
    match result {
        Ok(v) => {
            return HttpResponse::Ok().json(v);
        },
        Err(e) => {
            return HttpResponse::InternalServerError().body(e);
        }
    }
}

pub fn register_routes(app: &mut web::ServiceConfig) {
    app.route("/test", web::get().to(test_route))
       .route("/test/{name}", web::get().to(test_route))
       .route("/todos", web::get().to(get_all_TODOs))
       .route("/todos/add", web::post().to(add_TODO));
}