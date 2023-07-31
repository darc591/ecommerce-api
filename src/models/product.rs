use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{ Queryable, Insertable };
use crate::schema::{ product, product_category, product_item, product_variant };
#[derive(Queryable, Debug)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub deleted: bool,
    pub category_id: i32,
    pub store_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = product)]
pub struct InsertableProduct {
    pub name: String,
    pub category_id: i32,
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

#[derive(Insertable)]
#[diesel(table_name = product_item)]
pub struct InsertableProductItem {
    pub sku: Option<String>,
    pub description: Option<String>,
    pub price: BigDecimal,
    pub stock: i32,
    pub image_url: Option<String>,
    pub product_id: i32,
    pub store_id: i32,
    pub variant_id: Option<i32>,
}

#[derive(Queryable, Debug)]
pub struct ProductCategory {
    pub id: i32,
    pub name: String,
    pub store_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = product_category)]
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
pub struct ProductVariant {
    pub id: i32,
    pub name: String,
    pub value: String,
    pub store_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name = product_variant)]
pub struct InsertableVariant {
    pub name: String,
    pub value: String,
    pub store_id: i32,
}
