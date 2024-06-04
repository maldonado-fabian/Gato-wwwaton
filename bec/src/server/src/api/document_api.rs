use crate::{
    model::document_model::Document,
    repository::mongodb_repo::MongoRepo,
};
use actix_web::{
    post, get,
    web::{Data, Json, Path},
    HttpResponse,
};


#[post("/document")]
pub async fn create_document(db: Data<MongoRepo>, new_document: Json<Document> ) -> HttpResponse {
    let data = Document {
        id: None,
        tipo: new_document.tipo.to_owned(),
        titulo: new_document.titulo.to_owned(),
        autor: new_document.autor.to_owned(),
        editorial: new_document.editorial.to_owned(),
        ano: new_document.ano.to_owned(),
        edicion: new_document.edicion.to_owned(),
        categoria: new_document.categoria.to_owned(),
        isbn: new_document.isbn.to_owned(),
        esta_disponible: new_document.esta_disponible.to_owned(),
    };

    let document_detail = db.create_document(data).await;
    match document_detail {
        Ok(document) => HttpResponse::Ok().json(document),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),

    }
}

#[get("/document/{id}")]
pub async fn get_document(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID invalido");
    }

    let document_detail = db.get_document(&id).await;
    match document_detail {
        Ok(document) => HttpResponse::Ok().json(document),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}