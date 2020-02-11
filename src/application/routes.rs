use actix_web::{web, HttpRequest, HttpResponse};
use mongodb::{Client};
use bson::{doc};

use crate::model::memo::Memo;

pub async fn test_route(request: HttpRequest) -> HttpResponse {
    let name = request.match_info().get("name").unwrap_or("Unknown");
    return HttpResponse::Ok().body(format!("Hello world! {} is here", name));
}

#[allow(non_snake_case)]
pub async fn get_all_TODOs(client: web::Data<Client>) -> HttpResponse {
    let db = client.database("todo");
    let collection = db.collection("todo");

    let filter = doc! {};
    let all_data_cursor = collection.find(filter, None).unwrap();

    let mut response_data: Vec<Memo> = Vec::new();
    for result in all_data_cursor {
        if result.is_ok() {
            let data: bson::ordered::OrderedDocument = result.unwrap();
            let memo_parse_result = bson::from_bson(bson::Bson::Document(data));
            if memo_parse_result.is_ok() != true {
                return HttpResponse::InternalServerError().body("");
            }
            response_data.push(memo_parse_result.unwrap());
        } else {
            return HttpResponse::InternalServerError().body("");
        }
    }

    return HttpResponse::Ok().json(response_data);
}

pub fn register_routes(app: &mut web::ServiceConfig) {
    app.route("/test", web::get().to(test_route))
       .route("/test/{name}", web::get().to(test_route))
       .route("/todos", web::get().to(get_all_TODOs));
}