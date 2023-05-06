use diesel::prelude::*;
use crate::schema::urls;

#[derive(Queryable)]
pub struct Url {
  pub id: i32,
  pub shortened_url: String,
  pub redirect_to: String
}

#[derive(Insertable)]
#[diesel(table_name = urls)]
pub struct NewUrl<'a> {
  pub shortened_url: &'a str,
  pub redirect_to: &'a str
}