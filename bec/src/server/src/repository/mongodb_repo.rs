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
    document_model::Document,
    prestamo_model::Prestamo,
};


pub struct MongoRepo {
    //usuarios
    col_user: Collection<User>,
    //todo tipo de documentos
    col_document: Collection<Document>,
    //prestamos
    col_pres: Collection<Prestamo>
    
}
//Login logic

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
        let col_document: Collection<Document> = db.collection("Document");
        let col_pres: Collection<Prestamo> = db.collection("Prestamo");

        MongoRepo { col_user, col_pres, col_document }
    }


    pub async fn create_user(&self, new_usr: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            nombre: new_usr.nombre,
            apellido: new_usr.apellido,
            rut: new_usr.rut,
            direccion: new_usr.direccion,
            celular: new_usr.celular,
            admin: new_usr.admin,
            pass: new_usr.pass,
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

    pub async fn get_user_by_rut(&self, usr_rut: &String) -> Option<User> {
        match self.col_user.find_one(doc! {"rut": usr_rut}, None).await {
            Ok(Some(usr_detail)) => Some(usr_detail),
            _ => None,
        }
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
                "celular" : new_usr.celular,
                "admin": new_usr.admin,
                "pass": new_usr.pass
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

//document logic
impl MongoRepo {
    pub async fn create_document(&self, new_document: Document) -> Result<InsertOneResult, Error> {
        let mut books_with_ids = Vec::new();

        for book in new_document.libros {
            let mut book_with_id = book.clone();
            book_with_id.id = Some(ObjectId::new());
            books_with_ids.push(book_with_id);
        }

        let new_doc = Document {
            id: None,
            tipo: new_document.tipo,
            titulo: new_document.titulo,
            autor: new_document.autor,
            editorial: new_document.editorial,
            ano: new_document.ano,
            edicion: new_document.edicion,
            categoria: new_document.categoria,
            isbn: new_document.isbn,
            libros: books_with_ids,
        };

        let document = self
            .col_document
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating document");

        Ok(document)
    }

    pub async fn get_document(&self, document_id: &String) -> Result<Document, Error> {
        let obj_id = ObjectId::parse_str(document_id).unwrap();
        let filter = doc! { "_id": obj_id };
        let document_detail = self
        .col_document
        .find_one(filter, None)
        .await
        .expect("Error getting document detail");

        Ok(document_detail.unwrap())
    }

    pub async fn update_book_availability(&self, document_id: &String, book_id: &String, availability: bool) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(document_id).unwrap();
        let book_obj_id = ObjectId::parse_str(book_id).unwrap();
        let filter = doc! { "_id": obj_id, "libros._id": book_obj_id };
        let update = doc! {
            "$set": { "libros.$.disponibilidad": availability }
        };

        let update_result = self
            .col_document
            .update_one(filter, update, None)
            .await
            .ok()
            .expect("Could not update book availability");

        Ok(update_result)
    }


}

// Logica del prestamo
impl MongoRepo {
    pub async fn create_pres(&self, new_pres: Prestamo) -> Result<InsertOneResult, Error> {
        let new_doc = Prestamo {
            id : None,
            id_ejemplar : new_pres.id_ejemplar,
            tipo_prestamo : new_pres.tipo_prestamo,
            fecha_prestamo : new_pres.fecha_prestamo,
            hora_prestamo : new_pres.hora_prestamo,
            hora_devolucion : new_pres.hora_devolucion,
            fecha_devolucion : new_pres.fecha_devolucion,
            fecha_devolucion_real : new_pres.fecha_devolucion_real,
            hora_devolucion_real : new_pres.hora_devolucion_real,
        };

        let pres = self
        .col_pres
        .insert_one(new_doc, None)
        .await
        .ok()
        .expect("Error creando prestamo");

        Ok(pres)
    }
    
    pub async fn get_prestamo(&self, pres_id: &String) -> Result<Prestamo, Error> {
        let o_id = ObjectId::parse_str(pres_id).unwrap();
        let filter = doc! {"_id" : o_id};
        let prestamo_detail = self
        .col_pres
        .find_one(filter, None)
        .await
        .expect("Error getting pres detail.");

        Ok(prestamo_detail.unwrap())
    }

    pub async fn update_pres(&self, id: &String, new_pres: Prestamo) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set" : {
                "id" : new_pres.id,
                "id_ejemplar" : new_pres.id_ejemplar,
                "tipo_prestamo" : new_pres.tipo_prestamo,
                "fecha_prestamo" : new_pres.fecha_prestamo,
                "hora_prestamo" : new_pres.hora_prestamo,
                "hora_devolucion" : new_pres.hora_devolucion,
                "fecha_devolucion" : new_pres.fecha_devolucion,
                "fecha_devolucion_real" : new_pres.fecha_devolucion_real,
                "hora_devolucion_real" : new_pres.hora_devolucion_real,
            },
        };

        let updated_doc = self
        .col_pres
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Could not update pres");
        
        Ok(updated_doc)
    }

    pub async fn delete_pres(&self, id: &String) -> Result<DeleteResult, Error> {
        let o_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": o_id };
        let pres_detail = self
        .col_user
        .delete_one(filter, None)
        .await
        .ok()
        .expect("Could not delete pres.");

        Ok(pres_detail)

    }

    pub async fn get_all_prestamos(&self) -> Result<Vec<Prestamo>, Error> {
        let mut cursor = self
                            .col_pres
                            .find(None,None)
                            .await
                            .ok()
                            .expect("Error getting list of pres.");
        let mut prestamos: Vec<Prestamo> = Vec::new();
        while let Some(pres) = cursor
                .try_next()
                .await
                .ok()
                .expect("Error mapping through cursor.") {
                    prestamos.push(pres)
                }
        Ok(prestamos)
    }

}