use chrono::NaiveDateTime;
use diesel::{ Queryable, Insertable, QueryableByName };

#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::shopping_cart)]
pub struct ShoppingCart {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub customer_id: i32,
    pub store_id: i32,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::shopping_cart)]
pub struct InsertableShoppingCart {
    pub customer_id: i32,
    pub store_id: i32,
}
