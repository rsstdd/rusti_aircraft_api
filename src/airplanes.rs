use diesel::pg::PgConnection;

pub mod schema {
    table! {
        aircrafts {
            id -> Nullable<Integer>,
            name  -> Text,
            description  -> Text,
            year_in_service -> Integer,
            country_of_origin -> Text,
            operators -> Text,
            max_speed -> Integer,
            max_range -> Integer,
            ceiling -> Integer,
            engines -> Text,
            img_url -> Text,
        }
    }
}

pub struct Airplanes {
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

impl Airplanes {
    pub fn read(connection: &PgConnection) -> Vec<Airplanes> {
        aircrafts::table
            .order(aircrafts::id)
            .load::<Airplanes>(connection)
            .unwrap()
    }
}
