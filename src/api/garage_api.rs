use std::collections::HashMap;

use actix_web::{ get, web, HttpResponse, Responder};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use serde_json::json;

use crate::models::model_garage::{GarageModel};


#[get("/api/garage/getgarages")]
pub async fn get_garages(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al obtener la conexion");
    match web::block(move || GarageModel::get_garages(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}

#[get("/api/garage/getgaragebyid")]
pub async fn get_garage_by_id(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,query: web::Query<HashMap<String, i64>>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al obtener la conexion");

    let id_value = query.get("id").unwrap_or(&-1).clone(); //Necesito clonarlo, no me gusta pero funcionará

    if id_value == -1 {
        HttpResponse::Ok().body("Id is not in correct format")
    }else{
        match web::block(move || GarageModel::get_garage_by_id(&mut conn, &id_value)).await {
            Ok(data) => {
                let data = data.unwrap();
                HttpResponse::Ok().json(json!(data))
            }
            Err(err) => HttpResponse::Ok().body(err.to_string()),
        }
    }
}