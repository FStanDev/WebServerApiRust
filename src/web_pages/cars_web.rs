use actix_web::{ get, web, HttpResponse, Responder};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

use crate::TeraContext;
use tera::Tera;

use crate::models::model_car_to_repair::{CarToRepairModel};

#[get("/cars/create_car")]
pub async fn create_car(template_manager: web::Data<Tera>) -> impl Responder {

    let ctx = TeraContext::new();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("create_car.html", &ctx).unwrap())
}

#[get("/cars")]
pub async fn get_all_cars(pool: web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>,template_manager: web::Data<Tera>) -> impl Responder {

    let mut ctx = TeraContext::new();
    let mut conn = pool.get().expect("Problemas al obtener la conexion");

    match web::block(move || CarToRepairModel::get_cars(&mut conn)).await {
        Ok(data) => {
            ctx.insert("cars", &data.unwrap());
            HttpResponse::Ok().content_type("text/html").body(
                template_manager.render("cars.html", &ctx).unwrap()
            )
        }
        Err(err) => HttpResponse::Ok().body(err.to_string()),
    }
}