use std::vec::Vec;

use super::entities::Memo;

pub trait MemoRepository {
    fn get_all(&self) -> Vec<Memo>;
}