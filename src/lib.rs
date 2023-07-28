use crate::models::NewUrl;
use diesel::{PgConnection, RunQueryDsl};
use models::Url;

pub mod models;
pub mod schema;

pub fn get_url_entries(conn: &mut PgConnection) {}

pub fn create_url_entry(conn: &mut PgConnection, shortened_url: &str, redirect_to: &str) -> Url {
    use crate::schema::urls;

    let new_url = NewUrl {
        shortened_url,
        redirect_to,
    };

    diesel::insert_into(urls::table)
        .values(&new_url)
        // Can run `.execute()` here instead if we don't want the return values
        .get_result(conn)
        .expect("Error while saving URL record!")
}
