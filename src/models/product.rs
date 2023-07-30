use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Queryable, Insertable};

#[derive(Queryable, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub deleted: bool,
    pub category_id: i32,
    pub store_id: i32,
}

#[derive(Queryable, Debug)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub store_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::product_category)]
pub struct InsertableCategory {
    pub name: String,
    pub store_id: i32,
}


#[derive(Queryable, Debug)]
pub struct ProductDiscount {
    pub id: i32,
    pub percentual: BigDecimal,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub store_id: i32,
}

#[derive(Queryable, Debug)]
pub struct ProductItem {
    pub id: i32,
    pub sku: Option<String>,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub deleted: bool,
    pub created_at: NaiveDateTime,
    pub discount_id: Option<i32>,
    pub image_url: Option<String>,
    pub product_id: i32,
    pub store_id: i32,
    pub updated_at: NaiveDateTime,
    pub variant_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct ProductVariant {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub store_id: i32,
}