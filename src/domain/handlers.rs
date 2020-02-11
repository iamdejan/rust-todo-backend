use super::repositories::MemoRepository;
use crate::infrastructure::repositories::PersistentMemoRepository;
use std::error::Error;
use crate::domain::entities::Memo;

pub struct MemoHandler {
    memo_repository: MemoRepository
}

impl MemoHandler {
    pub fn new() -> MemoHandler {
        //TODO: create strategy pattern based on parameter
        return MemoHandler {
            memo_repository: PersistentMemoRepository::new()
        };
    }

    pub fn get_all(&self) -> Result<Vec<Memo>, Error> {
        return self.memo_repository.get_all();
    }
}