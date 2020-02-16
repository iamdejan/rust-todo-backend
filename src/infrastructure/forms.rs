use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct AddTODOForm {
    pub title: String,
    pub description: String
}