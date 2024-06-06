use crate::{
    repository::mongodb_repo::MongoRepo,
    model::model::Prestamo,
};

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId}, 
    results::{InsertOneResult, UpdateResult, DeleteResult}, 
};

impl MongoRepo {
    pub async fn create_pres(&self, new_pres: Prestamo) -> Result<InsertOneResult, Error> {
        let new_doc = Prestamo {
            id : None,
            id_ejemplar : new_pres.id_ejemplar,
            tipo_prestamo : new_pres.tipo_prestamo,
            fecha_prestamo : new_pres.fecha_prestamo,
            hora_prestamo : new_pres.hora_prestamo,
            hora_devolucion_prevista : new_pres.hora_devolucion_prevista,
            fecha_devolucion_prevista : new_pres.fecha_devolucion_prevista,
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
                "fecha_devolucion_prevista" : new_pres.fecha_devolucion_prevista,
                "hora_devolucion_prevista" : new_pres.hora_devolucion_prevista,
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