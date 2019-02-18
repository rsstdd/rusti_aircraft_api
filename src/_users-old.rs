use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::airplanes;

#[derive(AsChangeset, Debug, Queryable, Insertable, Serialize, Deserialize)]
pub struct Users {
    pub id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hashed_password: String,
}

impl Users {
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
            .expect("Error creating new hero");

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
