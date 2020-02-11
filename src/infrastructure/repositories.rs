use crate::domain::repositories::MemoRepository;
use crate::domain::entities::Memo;

use mongodb::Client;
use mongodb::options::ClientOptions;

use bson::doc;

use std::error::Error;

pub struct PersistentMemoRepository {
    client: Client
}

impl PersistentMemoRepository {
    pub fn new() -> PersistentMemoRepository {
        let db_url = "";
        let mut client_options: ClientOptions = ClientOptions::parse(db_url).unwrap();
        client_options.app_name = Some("todo_backend".to_string());
        let client = Client::with_options(client_options).unwrap();
        return PersistentMemoRepository {
            client
        };
    }
}

impl MemoRepository for PersistentMemoRepository {
    fn get_all(&self) -> Result<Vec<Memo>, Error> {
        let database = self.client.database("todo");
        let collection = database.collection("todo");

        let filter = doc! {};
        let all_data_cursor = collection.find(filter, None).unwrap();

        let mut response_data: Vec<Memo> = Vec::new();
        for result in all_data_cursor {
            if result.is_ok() {
                let data: bson::ordered::OrderedDocument = result.unwrap();
                let memo_parse_result = bson::from_bson(bson::Bson::Document(data));
                if memo_parse_result.is_ok() != true {
                    return Err();
                }
                response_data.push(memo_parse_result.unwrap());
            } else {
                return Err();
            }
        }

        return response_data;
    }
}