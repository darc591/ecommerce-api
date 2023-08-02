use std::time::SystemTime;

use crate::{
    models::{ address::{ Address, InsertableAddress, UserAddres }, response::IDResponse },
    controllers::address::CreateAddressBody,
    error::ServiceError,
    utils::validation::validate,
};

use super::Connection;
use derive_builder::Builder;
use diesel::{
    sql_query,
    QueryResult,
    sql_types::{ Integer, Text, Nullable, Timestamp },
    RunQueryDsl,
};
use log::info;
#[derive(Builder, Debug)]
pub struct AddressParams<'req> {
    #[builder(setter(strip_option))]
    pub id: Option<&'req i32>,
    #[builder(setter(strip_option))]
    pub user_id: Option<&'req i32>,
    #[builder(setter(strip_option))]
    pub deleted: Option<&'req bool>,
}

pub fn asdad() {
    let new_addr_params = AddressParamsBuilder::default()
        .id(&1)
        .build().unwrap();
    info!("{:?}", new_addr_params)
}

pub struct AddressService;

impl AddressService {
    pub fn find(
        id: &i32,
        user_id: &i32,
        deleted: bool,
        conn: &mut Connection
    ) -> Result<UserAddres, ServiceError> {
        // let result = sql_query(
        //     "SELECT * FROM public.user_address WHERE id = $1 AND deleted = $2"
        // ).bind::<Integer, _>(&id);

        // return sql_query(
        //     "SELECT
        // id,
        // number,
        // city,
        // country,
        // address_line1,
        // address_line2,
        // phone_country_code,
        // phone_number,
        // postal_code
        // FROM
        // public.user_address
        // WHERE
        // deleted = false
        // AND
        // id = $1
        // AND user_id = $2"
        // )
        //     .bind::<Integer, _>(id)
        //     .bind::<Integer, _>(user_id)
        //     .get_result::<Address>(conn);
        unimplemented!()
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
        user_id = $1"
        )
            .bind::<Integer, _>(user_id)
            .get_results::<Address>(conn);
    }
    pub fn create(
        payload: CreateAddressBody,
        user_id_: i32,
        conn: &mut Connection
    ) -> Result<IDResponse<i32>, ServiceError> {
        use crate::schema::user_address::dsl::*;

        validate(&payload)?;

        let new_address = InsertableAddress {
            user_id: user_id_,
            address_line1: payload.address_line1,
            address_line2: payload.address_line2,
            city: payload.city,
            country: payload.country,
            deleted: false,
            number: payload.number,
            phone_country_code: payload.phone_country_code,
            phone_number: payload.phone_number,
            postal_code: payload.postal_code,
        };

        match
            diesel
                ::insert_into(user_address)
                .values(new_address)
                .returning(id)
                .get_result::<i32>(conn)
        {
            Ok(created_id) => Ok(IDResponse { id: created_id }),
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }

    pub fn edit(
        address_id: i32,
        user_id_: i32,
        payload: CreateAddressBody,
        conn: &mut Connection
    ) -> Result<(), ServiceError> {
        validate(&payload)?;

        match Self::find(&address_id, &user_id_, false, conn) {
            Ok(addr) => {
                let now = diesel
                    ::select(diesel::dsl::now)
                    .get_result::<SystemTime>(conn)
                    .expect("Error getting system time");

                match
                    sql_query(
                        "
                UPDATE public.user_address
                SET
                address_line1 = $1,
                address_line2 = $2,
                city = $3,
                country = $4,
                number = $5,
                postal_code = $6,
                phone_number = $7,
                phone_country_code = $8,
                updated_at = $9
                WHERE id = $10
                "
                    )
                        .bind::<Text, _>(payload.address_line1)
                        .bind::<Nullable<Text>, _>(payload.address_line2)
                        .bind::<Text, _>(payload.city)
                        .bind::<Text, _>(payload.country)
                        .bind::<Text, _>(payload.number)
                        .bind::<Text, _>(payload.postal_code)
                        .bind::<Nullable<Text>, _>(payload.phone_number)
                        .bind::<Nullable<Text>, _>(payload.phone_country_code)
                        .bind::<Timestamp, _>(now)
                        .bind::<Integer, _>(addr.id)
                        .execute(conn)
                {
                    Ok(_) => Ok(()),
                    Err(e) =>
                        Err(ServiceError::InternalServerError { error_message: e.to_string() }),
                }
            }
            Err(_) =>
                Err(ServiceError::NotFound { error_message: "Address not found".to_string() }),
        }
    }
    pub fn delete(
        address_id: i32,
        user_id_: i32,
        conn: &mut Connection
    ) -> Result<(), ServiceError> {
        let now = diesel
            ::select(diesel::dsl::now)
            .get_result::<SystemTime>(conn)
            .expect("Error getting system time");

        match Self::find(&address_id, &user_id_, false, conn) {
            Ok(addr) =>
                match
                    sql_query(
                        "
                UPDATE public.user_address
                SET
                deleted = true,
                updated_at = $1
                WHERE id = $2
                "
                    )
                        .bind::<Timestamp, _>(now)
                        .bind::<Integer, _>(addr.id)
                        .execute(conn)
                {
                    Ok(_) => Ok(()),
                    Err(e) =>
                        Err(ServiceError::InternalServerError { error_message: e.to_string() }),
                }
            Err(_) =>
                Err(ServiceError::NotFound { error_message: "Address not found".to_string() }),
        }
    }
}
