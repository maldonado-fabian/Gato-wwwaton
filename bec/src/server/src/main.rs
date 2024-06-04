mod api;
mod model;
mod repository;

use actix_web::{ web::Data, App, HttpServer };
use api::{
    user_api::{create_user, get_user, put_user, delete_user,get_users},
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
    document_api::{create_document, get_document},
=======
>>>>>>> 97dacd3 (minor changes)
=======
    prestamos_api::{create_prestamo, get_prestamo, put_prestamo, delete_prestamo,get_prestamos}
>>>>>>> ca5c1db (added services)
=======
    prestamos_api::{create_prestamo, get_prestamo, put_prestamo, delete_prestamo,get_prestamos}
>>>>>>> e6198d0adf86b86490c24df490dcf90bc9f5478e
};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db = MongoRepo::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
        .service(create_user)
        .service(get_user)
        .service(put_user)
        .service(delete_user)
        .service(get_users)
<<<<<<< HEAD
<<<<<<< HEAD
<<<<<<< HEAD
        .service(create_document)
        .service(get_document)
=======
>>>>>>> 97dacd3 (minor changes)
=======
=======
>>>>>>> e6198d0adf86b86490c24df490dcf90bc9f5478e
        .service(create_prestamo)
        .service(get_prestamo)
        .service(put_prestamo)
        .service(delete_prestamo)
        .service(get_prestamos)
<<<<<<< HEAD
>>>>>>> ca5c1db (added services)
=======
>>>>>>> e6198d0adf86b86490c24df490dcf90bc9f5478e
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
