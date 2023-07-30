use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct ShoppingCart {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub customer_id: i32,
    pub store_id: i32,
}