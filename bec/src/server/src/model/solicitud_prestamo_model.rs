use serde::{ Serialize, Deserialize };
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct SolicitudPrestamo {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id : Option<ObjectId>,
    pub id_solicitud: String,
    pub rut: String,
    pub fecha_solicitud: String,
    pub hora_solicitud: String,
}