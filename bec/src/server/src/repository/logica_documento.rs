use crate::{
    repository::mongodb_repo::MongoRepo,
    model::model::Documento,
};

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId,to_bson}, 
    results::{InsertOneResult, UpdateResult, DeleteResult}, 
};

impl MongoRepo {
    pub async fn create_document(&self, new_document: Documento) -> Result<InsertOneResult,Error> {
        let new_doc: Documento = Documento {
            id: None,
            tipo: new_document.tipo,
            titulo: new_document.titulo,
            autor: new_document.autor,
            editorial: new_document.editorial,
            ano: new_document.ano,
            edicion: new_document.edicion,
            categoria: new_document.categoria,
            is_available: new_document.is_available,
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
}