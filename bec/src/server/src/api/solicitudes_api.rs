use crate::{
    model::solicitud_prestamo_model::SolicitudPrestamo,
    repository::mongodb_repo::MongoRepo,
};

use actix_web::{
    post, put, get, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use mongodb::bson::oid::ObjectId;

#[post("/solicitud_prestamo")]
pub async fn create_soli(db: Data<MongoRepo>, new_sol: Json<SolicitudPrestamo>) -> HttpResponse {
    let data = SolicitudPrestamo {
        id : None,
        id_solicitud : new_sol.id_solicitud.to_owned(),
        rut : new_sol.rut.to_owned(),
        fecha_solicitud : new_sol.fecha_solicitud.to_owned(),
        hora_solicitud : new_sol.hora_solicitud.to_owned(),
    };

    let sol_detail = db.create_sol(data).await;
    match sol_detail {
        Ok(solicitud) => HttpResponse::Ok().json(solicitud),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/solicitudes/{id}")]
pub async fn get_solicitud(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID invalido");
    }

    let soli_detail = db.get_solpres(&id).await;
    match soli_detail {
        Ok(sol) => HttpResponse::Ok().json(sol),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/solicitudes/{id}")]
pub async fn put_soli(db: Data<MongoRepo>, path: Path<String>, new_sol: Json<SolicitudPrestamo>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let data = SolicitudPrestamo {
        id : Some(ObjectId::parse_str(&id).unwrap()),
        id_solicitud : new_sol.id_solicitud.to_owned(),
        rut : new_sol.rut.to_owned(),
        fecha_solicitud : new_sol.fecha_solicitud.to_owned(),
        hora_solicitud : new_sol.hora_solicitud.to_owned(),
    };

    let updated_res = db.update_soli(&id, data).await;
    match updated_res {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_pres_info = db.get_solpres(&id).await;
                return match updated_pres_info {
                    Ok(sol) => HttpResponse::Ok().json(sol),
                    Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                };
            }

            else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        },

        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),

    }   
}

#[delete("/solicitudes/{id}")]
pub async fn delete_soli(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let res = db.delete_sol(&id).await;
    match res {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Solicitud Successfully removed.");
            }

            else {
                return HttpResponse::NotFound().json("Solicitud with specified ID not found");
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }

}

#[get("/solicitudes")]
pub async fn get_solicitudes(db: Data<MongoRepo>) -> HttpResponse {
    let solicitudes = db.get_all_solicitudes().await;
    match solicitudes {
        Ok(sol) => HttpResponse::Ok().json(sol),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}