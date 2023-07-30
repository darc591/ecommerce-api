use chrono::NaiveDateTime;
use diesel::{ QueryableByName, Queryable, Insertable };
use serde::Serialize;
use serde_repr::{ Deserialize_repr, Serialize_repr };
use std::time::SystemTime;

#[derive(Serialize, Queryable, Debug, QueryableByName)]
#[diesel(table_name = crate::schema::user)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub first_name: String,
    pub last_name: String,
    pub type_: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_login: NaiveDateTime,
    pub managed_store_id: Option<i32>,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserType {
    CUSTOMER,
    ADMIN,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
pub struct InsertableUser {
    pub email: String,
    pub password: String,
    pub type_: i32,
    pub first_name: String,
    pub last_name: String,
    pub managed_store_id: Option<i32>,
    pub salt: String,
}
