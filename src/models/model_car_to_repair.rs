//Lo primero, importamos el modelo del schema
use super::super::schema::CarToRepair;
use super::super::schema::CarToRepair::dsl::*;
use diesel::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

//Ahora los modelos
#[derive(Insertable,Debug)] 
#[diesel(table_name = CarToRepair)]
pub struct NewCarToRepair<'a>{
    pub id: &'a Uuid,
    pub modelo: &'a String,
    pub fecha_entrada: &'a DateTime<Utc>,
    pub fecha_salida: &'a DateTime<Utc>
}

//Ahora el handler, lo necesitaremos para las apis
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewCarHandler{
    pub modelo: String,
    pub fecha_entrada: DateTime<Utc>,
    pub fecha_salida: DateTime<Utc>
}

//Ahora el modelo de datos como struct
#[derive(Queryable,Debug, Deserialize, Serialize)]
pub struct CarToRepairModel{
    pub id: Uuid,
    pub modelo: String,
    pub fecha_entrada: DateTime<Utc>,
    pub fecha_salida: DateTime<Utc>
}

//Ahora las funciones
impl CarToRepairModel{
    pub fn get_cars(conn:  &mut PgConnection)->Result<Vec<CarToRepairModel>, diesel::result::Error>{
        let cars = CarToRepair.load::<CarToRepairModel>(conn);
        cars
    }

    pub fn add_car_to_repair(conn:  &mut PgConnection, car: &NewCarHandler)->Result<CarToRepairModel, diesel::result::Error>{

        let new_car = NewCarToRepair{
            id: &Uuid::new_v4(),
            modelo: &car.modelo,
            fecha_entrada: &car.fecha_entrada,
            fecha_salida: &car.fecha_salida,
        };

        diesel::insert_into(CarToRepair::table)
            .values(new_car)
            .get_result::<CarToRepairModel>(conn)

    }


}

