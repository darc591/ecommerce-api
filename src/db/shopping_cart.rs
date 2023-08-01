use diesel::{ prelude::*, sql_query, sql_types::Integer };
use crate::{
    models::{
        response::IDResponse,
        shopping_cart::{ ShoppingCart, InsertableShoppingCart },
        product::ProductItem,
        order::{ InsertableOrderItem, OrderItem },
    },
    error::ServiceError,
    controllers::shopping_cart::{ NewShoppingCartBody, EditShoppingCartBody },
    utils::validation::validate,
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
        "SELECT * FROM public.product_item WHERE id = $1 AND store_id = $2 AND deleted = false"
    )
        .bind::<Integer, _>(payload.product_item_id)
        .bind::<Integer, _>(payload.store_id)
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

pub fn edit(
    payload: EditShoppingCartBody,
    shopping_cart_id: i32,
    conn: &mut Connection
) -> Result<(), ServiceError> {
    use crate::schema::order_item;

    validate(&payload)?;

    let existings_shopping_cart = sql_query("SELECT * FROM public.shopping_cart WHERE id = $1")
        .bind::<Integer, _>(shopping_cart_id)
        .get_result::<ShoppingCart>(conn);

    if existings_shopping_cart.is_err() {
        return Err(ServiceError::NotFound { error_message: "Shopping cart not found".to_string() });
    }

    let result = sql_query(
        "SELECT * FROM public.order_item o
        INNER JOIN public.product_item p ON p.id = o.product_item_id
        WHERE o.shopping_cart_id = $1
        AND p.id = $2
        "
    )
        .bind::<Integer, _>(shopping_cart_id)
        .bind::<Integer, _>(payload.product_item_id)
        .get_results::<(OrderItem, ProductItem)>(conn);

    match result {
        Ok(select_result) => {
            if select_result.is_empty() {
                match
                    sql_query("SELECT * FROM public.product_item WHERE id = $1")
                        .bind::<Integer, _>(payload.product_item_id)
                        .get_result::<ProductItem>(conn)
                {
                    Ok(p_item) => {
                        if payload.quantity > p_item.stock {
                            return Err(ServiceError::Forbidden {
                                error_message: "Quantity not available".to_string(),
                            });
                        } else {
                            let new_order_item = InsertableOrderItem {
                                product_item_id: payload.product_item_id,
                                quantity: payload.quantity,
                                shopping_cart_id: Some(shopping_cart_id),
                                unit_price: p_item.price,
                                order_id: None,
                            };

                            let order_item_result = diesel
                                ::insert_into(order_item::dsl::order_item)
                                .values(new_order_item)
                                .execute(conn);

                            if order_item_result.is_err() {
                                return Err(ServiceError::InternalServerError {
                                    error_message: order_item_result.unwrap_err().to_string(),
                                });
                            } else {
                                return Ok(());
                            }
                        }
                    }
                    Err(_) => {
                        return Err(ServiceError::NotFound {
                            error_message: "Product not found".to_string(),
                        });
                    }
                }
            } else {
                let (existing_order_item, existing_product_item) = &select_result[0];

                if payload.quantity > existing_product_item.stock {
                    return Err(ServiceError::Forbidden {
                        error_message: "Quantity not available".to_string(),
                    });
                } else {
                    match
                        sql_query("UPDATE public.order_item SET quantity = $1 WHERE id = $2")
                            .bind::<Integer, _>(payload.quantity)
                            .bind::<Integer, _>(existing_order_item.id)
                            .execute(conn)
                    {
                        Ok(_) => {
                            return Ok(());
                        }
                        Err(e) => {
                            return Err(ServiceError::InternalServerError {
                                error_message: e.to_string(),
                            });
                        }
                    }
                }
            }
        }
        Err(e) => {
            return Err(ServiceError::InternalServerError { error_message: e.to_string() });
        }
    }
}
