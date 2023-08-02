use crate::{ models::{ order::InsertableOrderItem, response::IDResponse }, error::ServiceError };
use diesel::{ prelude::*, sql_query, sql_types::Integer };
use super::Connection;
use crate::schema::order_item;

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
    order_item_id: &i32,
    conn: &mut Connection
) -> Result<(), ServiceError> {
    let update_result = sql_query("UPDATE public.order_item SET quantity = $1 WHERE id = $2")
        .bind::<Integer, _>(quantity)
        .bind::<Integer, _>(order_item_id)
        .execute(conn);

    match update_result {
        Ok(_) => Ok(()),
        Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
    }
}
