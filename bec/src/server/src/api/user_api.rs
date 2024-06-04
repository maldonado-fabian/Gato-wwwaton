use crate::{
    model::user_model::User, 
    repository::mongodb_repo::MongoRepo
};
use actix_web::{
    post, put, get, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use mongodb::bson::oid::ObjectId;

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_usr: Json<User>) -> HttpResponse {
    let data = User {
        id : None,
        nombre : new_usr.nombre.to_owned(),
        apellido : new_usr.apellido.to_owned(),
        rut: new_usr.rut.to_owned(),
        direccion : new_usr.direccion.to_owned(),
        celular : new_usr.celular.to_owned(),
    };

    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/user/{id}")]
pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID invalido");
    }

    let user_detail = db.get_user(&id).await;
    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[put("/user/{id}")]
pub async fn put_user(db: Data<MongoRepo>, path: Path<String>, new_usr: Json<User>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let data = User {
        id : Some(ObjectId::parse_str(&id).unwrap()),
        nombre : new_usr.nombre.to_owned(),
        apellido : new_usr.apellido.to_owned(), 
        rut: new_usr.rut.to_owned(),
        direccion : new_usr.direccion.to_owned(),
        celular : new_usr.celular.to_owned(),
    };

    let updated_res = db.update_user(&id, data).await;
    match updated_res {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;
                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
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

#[delete("user/{id}")]
pub async fn delete_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let res = db.delete_user(&id).await;
    match res {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("User Successfully removed.");
            }

            else {
                return HttpResponse::NotFound().json("User with specified ID not found");
            }
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }

}

#[get("/users")]
pub async fn get_users(db: Data<MongoRepo>) -> HttpResponse {
    let users = db.get_all_users().await;
    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}