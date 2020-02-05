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

    // for result in all_data_cursor {
    //     if result.is_ok() {
    //         let data: bson::ordered::OrderedDocument = result.unwrap();
    //         let memo_parse_result = bson::from_bson(bson::Bson::Document(data));
    //         if memo_parse_result.is_ok() != true {
    //             return HttpResponse::InternalServerError().body("");
    //         }
    //         response_data.push(memo_parse_result.unwrap());
    //     } else {
    //         return HttpResponse::InternalServerError().body("");
    //     }
    // }

    return HttpResponse::Ok().json(response_data);
}

pub fn register_routes(app: &mut web::ServiceConfig) {
    app.route("/test", web::get().to(route_initial_test))
       .route("/test/{name}", web::get().to(route_initial_test))
       .route("/todos", web::get().to(get_all_TODOs));
}