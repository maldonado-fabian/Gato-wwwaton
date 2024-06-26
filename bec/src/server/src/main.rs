mod api;
mod model;
mod repository;

use actix_cors::Cors;
use actix_web::{ web::Data, App, HttpServer };
use api::{
    user_api::{create_user, get_user, put_user, delete_user,get_users, login},
    document_api::{create_document, get_document, update_book_availability, get_all_docs},
    prestamos_api::{create_prestamo, get_prestamo, put_prestamo, delete_prestamo,get_prestamos}

};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
        .wrap(
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
        )
        .app_data(db_data.clone())
        .service(create_user)
        .service(get_user)
        .service(put_user)
        .service(delete_user)
        .service(get_users)
        .service(login)
        .service(update_book_availability)
        .service(create_document)
        .service(get_document)
        .service(get_all_docs)
        .service(create_prestamo)
        .service(get_prestamo)
        .service(put_prestamo)
        .service(delete_prestamo)
        .service(get_prestamos)

    })
    .bind(("localhost", 8080))?
    .run()
    .await
}