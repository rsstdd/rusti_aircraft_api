use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::airplanes;

#[derive(AsChangeset, Debug, Queryable, Insertable, Serialize, Deserialize)]
pub struct Airplane {
    pub id: Option<i32>,
    pub name: String,
    pub description: String,
    pub year_in_service: i32,
    pub country_of_origin: String,
    pub operators: String,
    pub max_speed: i32,
    pub max_range: i32,
    pub ceiling: i32,
    pub engines: String,
    pub img_url: String,
}

impl Airplane {
    pub fn read(connection: &PgConnection) -> Vec<Airplane> {
        airplanes::table
            .order(airplanes::id)
            .load::<Airplane>(connection)
            .unwrap()
    }

    pub fn create(aircraft: Airplane, connection: &PgConnection) -> Airplane {
        diesel::insert_into(airplanes::table)
            .values(&aircraft)
            .execute(connection)
            .expect("Error creating new Aircraft");

        airplanes::table
            .order(airplanes::id.desc())
            .first(connection)
            .unwrap()
    }

    pub fn read_id(id: i32, connection: &PgConnection) -> Vec<Airplane> {
        airplanes::table
            .find(id)
            .load::<Airplane>(connection)
            .unwrap()
    }

    pub fn update(id: i32, aircraft: Airplane, connection: &PgConnection) -> bool {
        println!("{:?}", aircraft);
        diesel::update(airplanes::table.find(id))
            .set(&aircraft)
            .execute(connection)
            .is_ok()
    }

    pub fn delete(id: i32, connection: &PgConnection) -> bool {
        diesel::delete(airplanes::table.find(id))
            .execute(connection)
            .is_ok()
    }
}
