use actix_web::{ get, web, HttpResponse, Responder};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

use crate::TeraContext;
use tera::Tera;

use crate::models::model_garage::{GarageModel};
use crate::models::model_working_cars::{WorkingCarsModel};

#[get("/garages")]
pub async fn get_garages(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,template_manager: web::Data<Tera>) -> impl Responder {

    let mut ctx = TeraContext::new();
    let mut conn = pool.get().expect("Problemas al obtener la conexion");

    match web::block(move || GarageModel::get_garages(&mut conn)).await {
        Ok(data) => {
            ctx.insert("garages", &data.unwrap());
            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("garages.html", &ctx).unwrap()
            )
            //HttpResponse::Ok().body(respuesta)
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}

#[get("/assgined_car_to_garage/{id}")]
pub async fn get_assigned_car_to_garages(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,template_manager: web::Data<Tera>, 
    id: web::Path<String>) -> impl Responder {

    let mut ctx = TeraContext::new();
    let mut conn = pool.get().expect("Problemas al obtener la conexion");

    let garage_id = id.parse::<i64>().unwrap();

    match web::block(move || WorkingCarsModel::get_working_cars_assigned_to_garage(&mut conn, &garage_id)).await {
        Ok(data) => {
            ctx.insert("cars", &data.unwrap());
            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("assigned_cars.html", &ctx).unwrap()
            )
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}