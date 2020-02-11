use super::entities::Memo;
use super::repositories::MemoRepository;

pub struct MemoHandler<T: MemoRepository + 'a> {
    memo_repository: &'a T
}

impl<T: MemoRepository + 'a> MemoHandler<T: MemoRepository + 'a> {
    pub fn new(memo_repository: &dyn MemoRepository) -> Self {
        return MemoHandler {
            memo_repository
        };
    }

    pub fn get_all(&self) -> Vec<Memo> {
        return self.memo_repository.get_all();
    }
}