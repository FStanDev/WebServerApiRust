use std::collections::HashMap;

use actix_web::{ post, delete, web, HttpResponse, Responder};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use serde_json::json;

use crate::models::model_working_cars::{NewWorkingCarHandler, WorkingCarsModel};



#[post("/api/workingcar/assingcar")]
pub async fn assign_car_to_repair(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,item: web::Json<NewWorkingCarHandler>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al obtener la conexion");
    match web::block(move || WorkingCarsModel::add_new_working_car(&mut conn,&item)).await {
        Ok(data) => {
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}

#[post("/api/workingcar/getworkingcars")]
pub async fn get_all_working_cars_assigned(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al obtener la conexion");
    match web::block(move || WorkingCarsModel::get_all_working_cars(&mut conn)).await {
        Ok(data) => {
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}

#[delete("/api/workingcar/carrepaired")]
pub async fn repaired_car(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,query: web::Query<HashMap<String, i64>>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al obtener la conexion");

    let id_value = query.get("id").unwrap_or(&-1).clone(); //Necesito clonarlo, no me gusta pero funcionará

    match web::block(move || WorkingCarsModel::repaired_car(&mut conn, &id_value)).await {
        Ok(data) => {
            let data = data.unwrap();
            HttpResponse::Ok().json(json!(data))
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}