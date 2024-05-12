use crate::{
    model::book_model::Book,
    repository::mongodb_repo::MongoRepo,
};
use actix_web::{
    post, put, get, delete,
    web::{Data, Json, Path},
    HttpResponse,
};

use mongodb::bson::oid::ObjectId;

#[post("/book")]
pub async fn create_book(db: Data<MongoRepo>, new_book: Json<Book> ) -> HttpResponse {
    let data = Book {
        id: None,
        title: new_book.title.to_owned(),
        author: new_book.author.to_owned(),
        editorial: new_book.editorial.to_owned(),
        isbn: new_book.isbn.to_owned(),
        is_available: new_book.is_available.to_owned(),
    };

    let book_detail = db.create_book(data).await;
    match book_detail {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),

    }
}

#[get("/book/{id}")]
pub async fn get_book(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("ID invalido");
    }

    let book_detail = db.get_book(&id).await;
    match book_detail {
        Ok(book) => HttpResponse::Ok().json(book),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}