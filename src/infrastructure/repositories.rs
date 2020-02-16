use mongodb::Client;
use mongodb::options::ClientOptions;
use mongodb::results::InsertOneResult;
use bson::{doc, bson, Bson};

use super::forms::AddTODOForm;
use crate::domain::repositories::MemoRepository;
use crate::domain::entities::Memo;

pub struct PersistentMemoRepository {
    client: Client
}

impl PersistentMemoRepository {
    pub fn new() -> PersistentMemoRepository {
        let db_url = "mongodb://localhost:27017";
        let mut client_options: ClientOptions = ClientOptions::parse(db_url).unwrap();
        client_options.app_name = Some("todo_backend".to_string());
        let client = Client::with_options(client_options).unwrap();
        return PersistentMemoRepository {
            client
        };
    }

    fn get_collection(&self) -> mongodb::Collection {
        let database = self.client.database("todo");
        let collection = database.collection("todo");
        return collection;
    }
}

impl MemoRepository for PersistentMemoRepository {
    fn get_all(&self) -> Result<Vec<Memo>, &'static str> {
        let collection = self.get_collection();
        let filter = doc! {};
        let all_data_cursor = collection.find(filter, None).unwrap();

        let mut response_data: Vec<Memo> = Vec::new();
        for result in all_data_cursor {
            if result.is_ok() {
                let data: bson::ordered::OrderedDocument = result.unwrap();
                let memo_parse_result = bson::from_bson(bson::Bson::Document(data));
                if memo_parse_result.is_ok() != true {
                    return Err("data cannot be parsed");
                }
                response_data.push(memo_parse_result.unwrap());
            } else {
                return Err("data cannot be parsed");
            }
        }

        return Ok(response_data);
    }

    fn insert(&self, form: AddTODOForm) -> Result<Bson, &'static str> {
        let collection = self.get_collection();
        let result = collection.insert_one(doc! { "title": form.title.as_str(), "description": form.description }, None);

        if result.is_ok() != true {
            return Err("Failed to insert");
        }

        let insert_one_result: InsertOneResult = result.unwrap();
        return Ok(insert_one_result.inserted_id);
    }
}