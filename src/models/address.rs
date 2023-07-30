use chrono::NaiveDateTime;
use diesel::{ Insertable, Queryable, QueryableByName };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Queryable, Debug)]
pub struct UserAddres {
    pub id: i32,
    pub number: String,
    pub city: String,
    pub country: String,
    pub deleted: bool,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub created_at: NaiveDateTime,
    pub phone_country_code: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: String,
    pub updated_at: NaiveDateTime,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::user_address)]
pub struct Address {
    pub id: i32,
    pub number: String,
    pub city: String,
    pub country: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub phone_country_code: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_address)]
pub struct InsertableAddress {
    pub number: String,
    pub city: String,
    pub country: String,
    pub deleted: bool,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub phone_country_code: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: String,
    pub user_id: i32,
}
