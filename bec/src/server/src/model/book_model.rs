use serde::{ Deserialize, Serialize };
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
    pub editorial: String,
    pub isbn: String,
    pub is_available: bool,
}