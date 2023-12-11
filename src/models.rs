use diesel::{Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = crate::schema::short_urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortUrl {
    pub id: i32,
    pub url: String,
    pub token: String,
}
