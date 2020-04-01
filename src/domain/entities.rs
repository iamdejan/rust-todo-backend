#[derive(Serialize, Deserialize, Debug)]
pub struct Memo {
    #[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,

    pub title: String,
    pub description: String
}
