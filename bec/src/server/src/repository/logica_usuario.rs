use crate::{
    repository::mongodb_repo::MongoRepo,
    model::model::Usuario,
};

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId}, 
    results::{InsertOneResult, UpdateResult, DeleteResult}, 
};

impl MongoRepo {
    pub async fn create_user(&self, new_usr: Usuario) -> Result<InsertOneResult, Error> {
        let new_doc = Usuario {
            id: None,
            nombre: new_usr.nombre,
            apellido: new_usr.apellido,
            rut: new_usr.rut,
            password: new_usr.password,
            ubicacion: new_usr.ubicacion,
            direccion: new_usr.direccion,
            celular: new_usr.celular,
            is_admin: false,
        };

        let user = self
        .col_user
        .insert_one(new_doc, None)
        .await
        .ok()
        .expect("Error creating user");

        Ok(user)
    }

    pub async fn create_admin(&self, new_admin: Usuario) -> Result<InsertOneResult, Error> {
        let new_doc = Usuario {
            id: None,
            nombre: new_admin.nombre,
            apellido: new_admin.apellido,
            rut: new_admin.rut,
            password: new_admin.password,
            ubicacion: new_admin.ubicacion,
            direccion: new_admin.direccion,
            celular: new_admin.celular,
            is_admin: true,
        };

        let admin = self
        .col_user
        .insert_one(new_doc, None)
        .await
        .ok()
        .expect("Error creating user");

        Ok(admin)
    }

    pub async fn get_user(&self, usr_id: &String) -> Result<Usuario, Error> {
        let o_id = ObjectId::parse_str(usr_id).unwrap();
        let filter = doc! {"_id" : o_id};
        let usr_detail = self
        .col_user
        .find_one(filter, None)
        .await
        .expect("Error getting user detail.");

        Ok(usr_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_usr: Usuario) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set" : {
                "id" : new_usr.id,
                "nombre" : new_usr.nombre,
                "apellido" : new_usr.apellido,
                "rut" : new_usr.rut,
                "password" : new_usr.password,
                "ubicacion" : new_usr.ubicacion,
                "direccion" : new_usr.direccion,
                "celular" : new_usr.celular,
                "is_admin" : new_usr.is_admin,
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
}