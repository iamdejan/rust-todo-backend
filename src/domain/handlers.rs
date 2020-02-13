use super::repositories::MemoRepository;
use crate::infrastructure::repositories::PersistentMemoRepository;
use crate::domain::entities::Memo;

pub struct MemoHandler<R: MemoRepository + ?Sized> {
    memo_repository: R
}

impl MemoHandler<PersistentMemoRepository> {
    pub fn new() -> Self {
        return Self {
            memo_repository: PersistentMemoRepository::new()
        };
    }

    pub fn get_all(&self) -> Result<Vec<Memo>, &'static str> {
        return self.memo_repository.get_all();
    }
}