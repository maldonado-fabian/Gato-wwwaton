use std::env;
extern crate dotenv;
use dotenv::dotenv;

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId}, 
    results::{InsertOneResult, UpdateResult, DeleteResult}, 
    Client, Collection};

use crate::model::{
    user_model::User,
    book_model::Book,
};


pub struct MongoRepo {
    //usuarios
    col_user: Collection<User>,
    //libros
    col_book: Collection<Book>,
}

//User logic
impl MongoRepo {

    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"), 
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col_user: Collection<User> = db.collection("User");
        let col_book: Collection<Book> = db.collection("Book");

        MongoRepo { col_user, col_book }
    }

    pub async fn create_user(&self, new_usr: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            nombre: new_usr.nombre,
            apellido: new_usr.apellido,
            rut: new_usr.rut,
            direccion: new_usr.direccion,
            celular: new_usr.celular,
        };

        let user = self
        .col_user
        .insert_one(new_doc, None)
        .await
        .ok()
        .expect("Error creating user");

        Ok(user)
    }

    pub async fn get_user(&self, usr_id: &String) -> Result<User, Error> {
        let o_id = ObjectId::parse_str(usr_id).unwrap();
        let filter = doc! {"_id" : o_id};
        let usr_detail = self
        .col_user
        .find_one(filter, None)
        .await
        .expect("Error getting user detail.");

        Ok(usr_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_usr: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set" : {
                "id" : new_usr.id,
                "nombre" : new_usr.nombre,
                "apellido" : new_usr.apellido,
                "rut" : new_usr.rut,
                "direccion" : new_usr.direccion,
                "celular" : new_usr.celular
            },
        };

        let updated_doc = self
        .col_user
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Could not update user");
        
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let o_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": o_id };
        let user_detail = self
        .col_user
        .delete_one(filter, None)
        .await
        .ok()
        .expect("Could not delete user.");

        Ok(user_detail)

    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursor = self
                            .col_user
                            .find(None,None)
                            .await
                            .ok()
                            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursor
                .try_next()
                .await
                .ok()
                .expect("Error mapping through cursor.") {
                    users.push(user)
                }
        Ok(users)
    }

}

//Book logic
impl MongoRepo {
    pub async fn create_book(&self, new_book: Book) -> Result<InsertOneResult,Error> {
        let new_doc: Book = Book {
            id: None,
            title: new_book.title,
            author: new_book.author,
            editorial: new_book.editorial,
            isbn: new_book.isbn,
            is_available: new_book.is_available,
        };

        let book = self
            .col_book
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating book");
        
        Ok(book)       
    }

    pub async fn get_book(&self, book_id: &String) -> Result<Book, Error> {
        let obj_id = ObjectId::parse_str(book_id).unwrap();
        let filter = doc! { "_id": obj_id };
        let book_detail = self
        .col_book
        .find_one(filter, None)
        .await
        .expect("Error getting book detail");

        Ok(book_detail.unwrap())
    }

}