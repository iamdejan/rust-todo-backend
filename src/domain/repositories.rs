use super::entities::Memo;
use std::error::Error;

// only interface to real repositories
pub trait MemoRepository {
    fn get_all(&self) -> Result<Vec<Memo>, Error>;
}