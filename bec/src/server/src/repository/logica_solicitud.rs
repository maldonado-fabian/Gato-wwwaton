use crate::{
    repository::mongodb_repo::MongoRepo,
    model::model::SolicitudPrestamo,
};

use futures::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId,to_bson}, 
    results::{InsertOneResult, UpdateResult, DeleteResult}, 
};

impl MongoRepo {
    pub async fn create_sol(&self, new_sol: SolicitudPrestamo) -> Result<InsertOneResult, Error> {
        let new_doc = SolicitudPrestamo {
            id : None,
            rut : new_sol.rut,
            fecha_solicitud : new_sol.fecha_solicitud,
            hora_solicitud : new_sol.hora_solicitud,
            detalles : new_sol.detalles,
        };

        let solicitud = self
        .col_soli
        .insert_one(new_doc, None)
        .await
        .ok()
        .expect("Error creando solicitud de prestamo");

        Ok(solicitud)
    }
    
    pub async fn get_solpres(&self, sol_id: &String) -> Result<SolicitudPrestamo, Error> {
        let o_id = ObjectId::parse_str(sol_id).unwrap();
        let filter = doc! {"_id" : o_id};
        let solicitud_detail = self
        .col_soli
        .find_one(filter, None)
        .await
        .expect("Error getting pres detail.");

        Ok(solicitud_detail.unwrap())
    }

    pub async fn update_soli(&self, id: &String, new_sol: SolicitudPrestamo) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let detalles_bson = to_bson(&new_sol.detalles).unwrap();
        let new_doc = doc! {
            "$set" : {
                "id" : new_sol.id,
                "rut" : new_sol.rut,
                "fecha_solicitud" : new_sol.fecha_solicitud,
                "hora_solicitud" : new_sol.hora_solicitud,
                "detalles" : detalles_bson,
            },
        };

        let updated_doc = self
        .col_soli
        .update_one(filter, new_doc, None)
        .await
        .ok()
        .expect("Could not update pres");
        
        Ok(updated_doc)
    }

    pub async fn delete_sol(&self, id: &String) -> Result<DeleteResult, Error> {
        let o_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! { "_id": o_id };
        let sol_detail = self
        .col_soli
        .delete_one(filter, None)
        .await
        .ok()
        .expect("Could not delete pres.");

        Ok(sol_detail)

    }

    pub async fn get_all_solicitudes(&self) -> Result<Vec<SolicitudPrestamo>, Error> {
        let mut cursor = self
                            .col_soli
                            .find(None,None)
                            .await
                            .ok()
                            .expect("Error getting list of pres.");
        let mut solicitudes: Vec<SolicitudPrestamo> = Vec::new();
        while let Some(sol) = cursor
                .try_next()
                .await
                .ok()
                .expect("Error mapping through cursor.") {
                    solicitudes.push(sol)
                }
        Ok(solicitudes)
    }
 
}