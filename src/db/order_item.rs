use crate::{
    models::{ order::{ InsertableOrderItem, OrderItem }, response::IDResponse },
    error::ServiceError,
};
use diesel::{ prelude::*, sql_query, sql_types::Integer };
use super::Connection;
use crate::schema::order_item;

pub struct OrderItemService;

impl OrderItemService {
    pub fn find_by_shopping_cart(
        shoping_cart_id: &i32,
        conn: &mut Connection
    ) -> Result<Vec<OrderItem>, ServiceError> {
        let order_items_result = sql_query(
            "SELECT id FROM public.order_item WHERE shopping_cart_id = $1"
        )
            .bind::<Integer, _>(shoping_cart_id)
            .get_results::<OrderItem>(conn);

        match order_items_result {
            Ok(items) => Ok(items),
            Err(e) =>
                Err(ServiceError::NotFound { error_message: "Order items not found".to_string() }),
        }
    }

    pub fn create(
        new_order_item: InsertableOrderItem,
        conn: &mut Connection
    ) -> Result<IDResponse<i32>, ServiceError> {
        let order_item_result = diesel
            ::insert_into(order_item::dsl::order_item)
            .values(new_order_item)
            .returning(order_item::dsl::id)
            .get_result::<i32>(conn);

        match order_item_result {
            Ok(id) => Ok(IDResponse { id }),
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }
    pub fn update_quantity(
        quantity: &i32,
        id: &i32,
        conn: &mut Connection
    ) -> Result<(), ServiceError> {
        let update_result = sql_query("UPDATE public.order_item SET quantity = $1 WHERE id = $2")
            .bind::<Integer, _>(quantity)
            .bind::<Integer, _>(id)
            .execute(conn);

        match update_result {
            Ok(_) => Ok(()),
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }
    pub fn delete(ids: Vec<i32>, conn: &mut Connection) -> Result<(), ServiceError> {
        let delete_result = diesel
            ::delete(order_item::dsl::order_item)
            .filter(order_item::dsl::id.eq_any(ids))
            .execute(conn);

        match delete_result {
            Ok(_) => Ok(()),
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }
}
