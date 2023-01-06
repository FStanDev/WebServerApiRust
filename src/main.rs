extern crate diesel;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;
extern crate tera;
use tera::Context as TeraContext;
use tera::Tera;

mod api;
mod models;
mod schema;
mod web_pages;

#[get("/")] //La ruta siempre arriba, junto al método
async fn hello(template_manager: web::Data<Tera>) -> impl Responder {
    let ctx = TeraContext::new();
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template_manager.render("index.html", &ctx).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //Conexión a la BBDD
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("Error en las pool");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*")).unwrap();
        App::new()
            .service(hello)
            .service(api::car_to_repair_api::new_car_to_repair)
            .service(api::car_to_repair_api::get_cars_to_repair)
            .service(api::garage_api::get_garage_by_id)
            .service(api::garage_api::get_garages)
            .service(api::working_cars_api::assign_car_to_repair)
            .service(api::working_cars_api::get_all_working_cars_assigned)
            .service(api::working_cars_api::repaired_car)
            .service(web_pages::garages_web::get_garages)
            .service(web_pages::garages_web::get_assigned_car_to_garages)
            .service(web_pages::cars_web::create_car)
            .service(web_pages::cars_web::get_all_cars)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
