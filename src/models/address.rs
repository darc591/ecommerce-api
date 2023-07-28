use crate::{
    api::address_controllers::CreateAddressBody, config::db::Connection, schema::user_address::dsl,
};
use chrono::NaiveDateTime;
use diesel::{
    sql_query, sql_types::Integer, Insertable, QueryResult, Queryable, QueryableByName, RunQueryDsl,
};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use validator::Validate;

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
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub phone_country_code: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: String,
    pub user_id: i32,
}

impl UserAddres {
    pub fn find(address_id: i32, user_id: i32, conn: &mut Connection) -> QueryResult<Address> {
        return sql_query(
            "SELECT 
        id, 
        number,
        city,
        country,
        address_line1,
        address_line2,
        phone_country_code,
        phone_number,
        postal_code
        FROM
        public.user_address 
        WHERE 
        deleted = false 
        AND
        id = $1 
        AND user_id = $2",
        )
        .bind::<Integer, _>(address_id)
        .bind::<Integer, _>(user_id)
        .get_result::<Address>(conn);
    }
    pub fn list(user_id: i32, conn: &mut Connection) -> QueryResult<Vec<Address>> {
        return sql_query(
            "SELECT 
        id, 
        number,
        city,
        country,
        address_line1,
        address_line2,
        phone_country_code,
        phone_number,
        postal_code
        FROM
        public.user_address 
        WHERE 
        deleted = false 
        AND
        user_id = $1",
        )
        .bind::<Integer, _>(user_id)
        .get_results::<Address>(conn);
    }
    pub fn create(
        payload: CreateAddressBody,
        user_id: i32,
        conn: &mut Connection,
    ) -> Result<i32, String> {
        match payload.validate() {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        }

        let now = diesel::select(diesel::dsl::now)
            .get_result::<SystemTime>(conn)
            .expect("Error getting system time");

        let new_address = InsertableAddress {
            user_id,
            address_line1: payload.address_line1,
            address_line2: payload.address_line2,
            city: payload.city,
            country: payload.country,
            created_at: now,
            deleted: false,
            number: payload.number,
            updated_at: now,
            phone_country_code: payload.phone_country_code,
            phone_number: payload.phone_number,
            postal_code: payload.postal_code,
        };

        match diesel::insert_into(dsl::user_address)
            .values(new_address)
            .returning(dsl::id)
            .get_result::<i32>(conn)
        {
            Ok(id) => Ok(id),
            Err(e) => Err(e.to_string()),
        }
    }
}
