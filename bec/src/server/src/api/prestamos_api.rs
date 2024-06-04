use crate::{
    model::prestamo_model::Prestamo,
    repository::mongodb_repo::MongoRepo,
};

use actix_web::{
    post, put, get, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use mongodb::bson::oid::ObjectId;

#[post("/prestamo")]
pub async fn create_prestamo(db: Data<MongoRepo>, new_pres: Json<Prestamo>) -> HttpResponse {
    let data = Prestamo {
        id : None,
        id_ejemplar : new_pres.id_ejemplar.to_owned(),
        tipo_prestamo : new_pres.tipo_prestamo.to_owned(),
        fecha_prestamo : new_pres.fecha_prestamo.to_owned(),
        hora_prestamo : new_pres.hora_prestamo.to_owned(),
        hora_devolucion : new_pres.hora_devolucion.to_owned(),
        fecha_devolucion : new_pres.fecha_devolucion.to_owned(),
        fecha_devolucion_real : new_pres.fecha_devolucion_real.to_owned(),
        hora_devolucion_real : new_pres.hora_devolucion_real.to_owned(),
    };

    let pres_detail = db.create_pres(data).await;
    match pres_detail {
        Ok(prestamo) => HttpResponse::Ok().json(prestamo),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/prestamo/{id}")]
pub async fn get_prestamo(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID invalido");
    }

    let prestamo_detail = db.get_prestamo(&id).await;
    match prestamo_detail {
        Ok(pres) => HttpResponse::Ok().json(pres),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/prestamo/{id}")]
pub async fn put_prestamo(db: Data<MongoRepo>, path: Path<String>, new_pres: Json<Prestamo>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let data = Prestamo {
        id : Some(ObjectId::parse_str(&id).unwrap()),
        id_ejemplar : new_pres.id_ejemplar.to_owned(),
        tipo_prestamo : new_pres.tipo_prestamo.to_owned(),
        fecha_prestamo : new_pres.fecha_prestamo.to_owned(),
        hora_prestamo : new_pres.hora_prestamo.to_owned(),
        hora_devolucion : new_pres.hora_devolucion.to_owned(),
        fecha_devolucion : new_pres.fecha_devolucion.to_owned(),
        fecha_devolucion_real : new_pres.fecha_devolucion_real.to_owned(),
        hora_devolucion_real : new_pres.hora_devolucion_real.to_owned(),
    };

    let updated_res = db.update_pres(&id, data).await;
    match updated_res {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_pres_info = db.get_prestamo(&id).await;
                return match updated_pres_info {
                    Ok(pres) => HttpResponse::Ok().json(pres),
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

#[delete("/prestamo/{id}")]
pub async fn delete_prestamo(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let res = db.delete_pres(&id).await;
    match res {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Pres Successfully removed.");
            }

            else {
                return HttpResponse::NotFound().json("Pres with specified ID not found");
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }

}

#[get("/prestamos")]
pub async fn get_prestamos(db: Data<MongoRepo>) -> HttpResponse {
    let prestamos = db.get_all_prestamos().await;
    match prestamos {
        Ok(prest) => HttpResponse::Ok().json(prest),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}