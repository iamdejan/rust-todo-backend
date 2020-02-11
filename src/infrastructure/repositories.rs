use mongodb::{Client, Collection};

use crate::domain::repositories::MemoRepository;
use crate::domain::entities::Memo;

use std::vec::Vec;

#[derive(Clone)]
pub struct PersistentMemoRepository {
    mongodb_client: Client
}

impl PersistentMemoRepository {
    fn get_collection(&self) -> Collection {
        return self.mongodb_client.database("todo").collection("todo");
    }

    pub fn new(client: Client) -> Self {
        return PersistentMemoRepository {
            mongodb_client: client
        };
    }
}

impl MemoRepository for &PersistentMemoRepository {
    fn get_all(&self) -> Vec<Memo> {
        let collection = (*self).get_collection();
        return Vec::new();
    }
}