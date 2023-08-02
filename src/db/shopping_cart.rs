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

use super::{ Connection, order_item as db_order_item, product_item as db_product_item };

pub fn find(id: &i32, conn: &mut Connection) -> Result<ShoppingCart, ServiceError> {
    let shopping_cart_result = sql_query("SELECT * FROM public.shopping_cart WHERE id = $1")
        .bind::<Integer, _>(id)
        .get_result::<ShoppingCart>(conn);

    match shopping_cart_result {
        Ok(s_cart) => Ok(s_cart),
        Err(_) =>
            Err(ServiceError::NotFound { error_message: "Shopping cart not found".to_string() }),
    }
}

pub fn create(
    payload: NewShoppingCartBody,
    user_id: i32,
    conn: &mut Connection
) -> Result<IDResponse<i32>, ServiceError> {
    use crate::schema::shopping_cart;

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

    let product_item = db_product_item::find(&payload.product_item_id, conn)?;

    if product_item.stock < payload.quantity {
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

    db_order_item::create(
        InsertableOrderItem {
            order_id: None,
            product_item_id: product_item.id,
            quantity: payload.quantity,
            shopping_cart_id: Some(new_cart_result),
            unit_price: product_item.price,
        },
        conn
    )?;

    Ok(IDResponse { id: new_cart_result })
}

pub fn edit(
    payload: EditShoppingCartBody,
    shopping_cart_id: i32,
    conn: &mut Connection
) -> Result<(), ServiceError> {
    validate(&payload)?;

    find(&shopping_cart_id, conn)?;

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
                            db_order_item::create(
                                InsertableOrderItem {
                                    product_item_id: payload.product_item_id,
                                    quantity: payload.quantity,
                                    shopping_cart_id: Some(shopping_cart_id),
                                    unit_price: p_item.price,
                                    order_id: None,
                                },
                                conn
                            )?;

                            Ok(())
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
                    db_order_item::update_quantity(
                        &payload.quantity,
                        &existing_order_item.id,
                        conn
                    )?;

                    Ok(())
                }
            }
        }
        Err(e) => {
            return Err(ServiceError::InternalServerError { error_message: e.to_string() });
        }
    }
}

pub fn delete(
    shopping_cart_id: i32,
    user_id: i32,
    conn: &mut Connection
) -> Result<(), ServiceError> {
    find(&shopping_cart_id, conn)?;
    unimplemented!()
}
