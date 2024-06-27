use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Documento {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub tipo: String,
    pub titulo: String,
    pub autor: String,
    pub editorial: String,
    pub ano: i32,
    pub edicion: String,
    pub categoria: String,
    pub tipo_medio: Option<String>,
    pub is_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ejemplar {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub id_documento: ObjectId,
    pub estado: String,
    pub ubicacion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Prestamo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub id_ejemplar: ObjectId,
    pub tipo_prestamo: String,
    pub fecha_prestamo: String,
    pub hora_prestamo: String,
    pub fecha_devolucion_prevista: String,
    pub hora_devolucion_prevista: String,
    pub fecha_devolucion_real: String,
    pub hora_devolucion_real: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetalleSolicitud {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub id_solicitud: ObjectId,
    pub id_ejemplar: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolicitudPrestamo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub rut: String,
    pub fecha_solicitud: String,
    pub hora_solicitud: String,
    pub detalles: Vec<DetalleSolicitud>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usuario {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub nombre: String,
    pub apellido: String,
    pub rut: String,
    pub password: String,
    pub ubicacion: String,
    pub direccion: String,
    pub celular: String,
    pub is_admin: bool,
}