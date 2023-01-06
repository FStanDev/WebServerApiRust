use super::super::schema::Garage::dsl::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

//Ahora los modelos
#[derive(Queryable,Debug, Deserialize, Serialize)]
#[diesel(table_name = Garage)]
pub struct GarageModel{
    pub id: i64,
    pub name: String,
    pub location: String,
    pub capacity: i32
}


impl GarageModel{
    pub fn get_garages(conn:  &mut PgConnection)->Result<Vec<GarageModel>, diesel::result::Error>{
        let garages = Garage.load::<GarageModel>(conn);
        garages
    }

    pub fn get_garage_by_id(conn:  &mut PgConnection, id_to_search: &i64)->Result<Vec<GarageModel>, diesel::result::Error>{
        let garage = Garage.filter(id.eq(id_to_search)).load::<GarageModel>(conn);
        garage
    }
}