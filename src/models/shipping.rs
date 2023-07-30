use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{ Queryable, Insertable };

#[derive(Queryable, Debug)]
pub struct ShippingMethod {
    pub id: i32,
    pub name: String,
    pub inactive: bool,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub store_id: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::shipping_method)]
pub struct InsertableShippingMethod {
    pub name: String,
    pub store_id: i32,
}



#[derive(Queryable, Debug)]
pub struct ShippingInformation {
    pub id: i32,
    pub status: i32,
    pub address_id: i32,
    pub created_at: NaiveDateTime,
    pub shipping_method_id: i32,
    pub shipping_price: BigDecimal,
    pub tracking_number: Option<String>,
    pub updated_at: NaiveDateTime,
}