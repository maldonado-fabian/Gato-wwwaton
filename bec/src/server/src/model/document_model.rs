use serde::{ Deserialize, Serialize };
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub ubicacion: String,
    pub disponibilidad: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    #[serde(rename="_id", skip_serializing_if="Option::is_none")]
    pub id: Option<ObjectId>,
    pub tipo: String,
    pub titulo: String,
    pub autor: String,
    pub editorial: String,
    pub ano: String,
    pub edicion: String,
    pub categoria: String,
    pub isbn: String,
    pub libros: Vec<Book>,  // Nuevo campo
}