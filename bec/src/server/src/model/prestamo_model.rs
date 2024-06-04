use serde::{ Serialize, Deserialize };
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Prestamo {
    #[serde(rename="_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub id_ejemplar : String,
    pub tipo_prestamo : String,
    pub fecha_prestamo : String,
    pub hora_prestamo : String,
    pub hora_devolucion : String,
    pub fecha_devolucion : String,
    pub fecha_devolucion_real: String,
    pub hora_devolucion_real : String,
}