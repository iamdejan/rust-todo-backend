use super::entities::Memo;

// only interface to real repositories
pub trait MemoRepository {
    fn get_all(&self) -> Result<Vec<Memo>, &'static str>;
}