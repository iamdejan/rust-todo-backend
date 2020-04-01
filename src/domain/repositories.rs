use bson::Bson;

use super::entities::Memo;
use crate::infrastructure::forms::AddTODOForm;

// only interface to real repositories
pub trait MemoRepository {
    fn get_all(&self) -> Result<Vec<Memo>, &'static str>;
    fn insert(&self, form: AddTODOForm) -> Result<Bson, &'static str>;
}
