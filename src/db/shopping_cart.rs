use diesel::{ prelude::*, sql_query, sql_types::Integer };
use crate::{
    models::{
        response::IDResponse,
        shopping_cart::{ ShoppingCart, InsertableShoppingCart },
        product::ProductItem,
        order::InsertableOrderItem,
    },
    error::ServiceError,
    controllers::shopping_cart::NewShoppingCartBody,
};

use super::Connection;

pub fn create(
    payload: NewShoppingCartBody,
    user_id: i32,
    conn: &mut Connection
) -> Result<IDResponse<i32>, ServiceError> {
    use crate::schema::{ shopping_cart, order_item };

    let existing_shopping_cart = sql_query(
        "SELECT * FROM public.shopping_cart WHERE customer_id = $1 AND store_id = $2"
    )
        .bind::<Integer, _>(user_id)
        .bind::<Integer, _>(payload.store_id)
        .get_result::<ShoppingCart>(conn);

    if existing_shopping_cart.is_ok() {
        return Err(ServiceError::Forbidden {
            error_message: "User already has an active ShoppingCart for this Store".to_string(),
        });
    }

    let existing_product_item = sql_query(
        "SELECT * FROM public.product_item WHERE id = $1 AND deleted = false"
    )
        .bind::<Integer, _>(payload.product_item_id)
        .get_result::<ProductItem>(conn);

    match existing_product_item {
        Ok(p_item) => {
            if p_item.stock < payload.quantity {
                return Err(ServiceError::Forbidden {
                    error_message: "Quantity not available".to_string(),
                });
            }

            let new_shopping_cart = InsertableShoppingCart {
                customer_id: user_id,
                store_id: payload.store_id,
            };

            let new_cart_result = diesel
                ::insert_into(shopping_cart::dsl::shopping_cart)
                .values(new_shopping_cart)
                .returning(shopping_cart::dsl::id)
                .get_result::<i32>(conn)
                .unwrap();

            let new_order_item = InsertableOrderItem {
                order_id: None,
                product_item_id: p_item.id,
                quantity: payload.quantity,
                shopping_cart_id: Some(new_cart_result),
                unit_price: p_item.price,
            };

            let order_item_result = diesel
                ::insert_into(order_item::dsl::order_item)
                .values(new_order_item)
                .execute(conn);

            if order_item_result.is_err() {
                return Err(ServiceError::InternalServerError {
                    error_message: order_item_result.unwrap_err().to_string(),
                });
            }

            Ok(IDResponse { id: new_cart_result })
        }
        Err(e) => Err(ServiceError::NotFound { error_message: e.to_string() }),
    }
}
