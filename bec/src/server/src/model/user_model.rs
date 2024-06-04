use serde::{ Serialize, Deserialize };
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nombre: String,
    pub apellido: String,
    pub rut: String,
    pub direccion: String,
    pub celular: String,
}
