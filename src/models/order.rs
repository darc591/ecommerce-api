use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Queryable, Debug)]
pub struct Order {
    pub id: i32,
    pub status: i16,
    pub created_at: NaiveDateTime,
    pub customer_id: i32,
    pub payment_method_id: i32,
    pub shipping_information_id: i32,
    pub store_id: i32,
    pub total_discount: Option<BigDecimal>,
    pub total_price: BigDecimal,
}

#[derive(Queryable, Debug)]
pub struct OrderItem {
    pub id: i32,
    pub quantity: i32,
    pub order_id: Option<i32>,
    pub product_item_id: i32,
    pub shopping_cart_id: Option<i32>,
    pub unit_price: BigDecimal,
}