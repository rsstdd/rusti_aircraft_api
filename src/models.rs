use diesel::{Insertable, Query};

#[derive(Queryable)]
pub struct Aircraft {
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

#[derive(Insertable)]
#[table_name = "airplanes"]
pub struct NewAircraft<'a> {
  pub name: &'a str,
  pub description: &'a str,
  pub year_in_service: &'a i32,
  pub country_of_origin: &'a str,
  pub operators: &'a str,
  pub max_speed: &'a i32,
  pub max_range: &'a i32,
  pub ceiling: i32,
  pub engines: &'a str,
  pub img_url: &'a str,
}
