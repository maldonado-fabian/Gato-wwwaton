use crate::{model::user_model::User, repository::mongodb_repo::MongoRepo };
use actix_web::{
    post,
    get,
    web::{Data, Json, Path},
    HttpResponse,
};

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_usr: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        name: new_usr.name.to_owned(),
        rut: new_usr.rut.to_owned(),
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