use chrono::NaiveDateTime;
use diesel::{ Queryable, Insertable };

#[derive(Queryable, Debug)]
pub struct Store {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub logo_url: Option<String>,
    pub updated_at: NaiveDateTime,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::store)]
pub struct InsertableStore {
    pub name: String,
    pub logo_url: Option<String>,
}
