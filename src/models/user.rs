use chrono::NaiveDateTime;
use diesel::{ QueryableByName, Queryable, Insertable, Identifiable };
use serde::Serialize;
use serde_repr::{ Deserialize_repr, Serialize_repr };
#[derive(Serialize, Queryable, Debug, QueryableByName, Identifiable)]
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

#[derive(Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum UserType {
    CUSTOMER,
    ADMIN,
}

impl UserType {
    pub fn from_i32(int_type: i32) -> Self {
        match int_type {
            0 => UserType::CUSTOMER,
            1 => UserType::ADMIN,
            _ => panic!("Unknown value: {}", int_type),
        }
    }
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
