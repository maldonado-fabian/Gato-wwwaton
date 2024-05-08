use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{bson::{doc, extjson::de::Error, oid::ObjectId}, results::InsertOneResult, Client, Collection};

use crate::model::user_model::User;

pub struct MongoRepo {
    col: Collection<User>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"), 
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("rustDB");
        let col: Collection<User> = db.collection("User");

        MongoRepo { col }
    }

    pub async fn create_user(&self, new_usr: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_usr.name,
            rut: new_usr.rut,
        };

        let user = self
        .col
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
        .col
        .find_one(filter, None)
        .await
        .expect("Error getting user detail.");

        Ok(usr_detail.unwrap())
    }
}