use diesel::pg::PgConnection;
use diesel::prelude::*;
use schema::airplanes;

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Aircrafts {
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

impl Aircrafts {
    pub fn read(connection: &PgConnection) -> Vec<Aircrafts> {
        airplanes::table
            .order(airplanes::id)
            .load::<Aircrafts>(connection)
            .unwrap()
    }

    pub fn read_id(id: i32, connection: &PgConnection) -> Vec<Aircrafts> {
        println!("{}", id);
        airplanes::table
            .find(id)
            .load::<Aircrafts>(connection)
            .unwrap()
    }
}
