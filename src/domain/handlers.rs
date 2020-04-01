use bson::Bson;

use super::repositories::MemoRepository;
use crate::infrastructure::repositories::PersistentMemoRepository;
use crate::domain::entities::Memo;
use crate::infrastructure::forms::AddTODOForm;

pub struct PersistentMemoHandler<R: MemoRepository + ?Sized> {
    memo_repository: R
}

impl PersistentMemoHandler<PersistentMemoRepository> {
    pub fn new() -> Self {
        return Self {
            memo_repository: PersistentMemoRepository::new()
        };
    }

    pub fn get_all(&self) -> Result<Vec<Memo>, &'static str> {
        return self.memo_repository.get_all();
    }

    pub fn insert(&self, form: AddTODOForm) -> Result<Bson, &'static str> {
        return self.memo_repository.insert(form);
    }
}
