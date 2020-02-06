use super::entities::Memo;
use super::repositories::MemoRepository;

pub struct MemoHandler<'a> {
    memo_repository: &'a dyn MemoRepository
}

impl<'a> MemoHandler<'a> {
    pub fn new(memo_repository: &dyn MemoRepository) -> MemoHandler {
        return MemoHandler {
            memo_repository
        };
    }

    pub fn get_all(&self) -> Vec<Memo> {
        return self.memo_repository.get_all();
    }
}