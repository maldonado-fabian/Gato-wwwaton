mod api;
mod model;
mod repository;

use std::env;
extern crate dotenv;
use dotenv::dotenv;

use actix_web::{ web::Data, App, HttpServer };
use api::{
    user_api::{create_user, get_user, put_user, delete_user,get_users},
    book_api::{create_book, get_book},
};
use repository::mongodb_repo::MongoRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host: String = match env::var("HOST") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variable HOST"),
    };
    
    let port: u16 = match env::var("PORT") {
        Ok(p) => p.parse::<u16>().unwrap(),
        Err(_) => 0,
    };

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
        .service(create_book)
        .service(get_book)
    })
    .bind((host, port))?
    .run()
    .await
}
