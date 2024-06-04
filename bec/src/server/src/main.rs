mod api;
mod model;
mod repository;

use actix_web::{ web::Data, App, HttpServer };
use api::{
    user_api::{create_user, get_user, put_user, delete_user,get_users},
<<<<<<< HEAD
    document_api::{create_document, get_document},
=======
>>>>>>> 97dacd3 (minor changes)
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
        .service(create_document)
        .service(get_document)
=======
>>>>>>> 97dacd3 (minor changes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
