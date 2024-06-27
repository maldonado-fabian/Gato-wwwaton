use crate::{
    model::document_model::Document,
    repository::mongodb_repo::MongoRepo,
};
use serde::Deserialize;
use mongodb::Collection;
use actix_web::{
    post, get, put, delete, Responder, 
    web::{Data, Json, Path},
    web,
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
        libros: new_document.libros.to_owned(),
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

#[get("/documents")]
pub async fn get_all_docs(db: Data<MongoRepo>) -> HttpResponse {
    let documents = db.get_all_documents().await;
    match documents {
        Ok(documents) => HttpResponse::Ok().json(documents),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct AvailabilityUpdate {
    availability: bool,
}
#[put("/document/{document_id}/{book_id}")]
pub async fn update_book_availability(
    db: Data<MongoRepo>, 
    path: Path<(String, String)>, 
    availability_update: Json<AvailabilityUpdate>
) -> HttpResponse {
    let (document_id, book_id) = path.into_inner();

    if document_id.is_empty() || book_id.is_empty() {
        return HttpResponse::BadRequest().body("ID inválido");
    }

    let update_result = db.update_book_availability(&document_id, &book_id, availability_update.availability).await;
    
    match update_result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}