use actix_web::{ web, post, put, HttpResponse, delete };
use serde::Deserialize;
use validator::Validate;
use crate::{
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    db::{ Pool, shopping_cart::ShoppingCartService, order_item::OrderItemService },
    models::response::ResponseBody,
};

#[derive(Deserialize)]
pub struct NewShoppingCartBody {
    pub store_id: i32,
    pub product_item_id: i32,
    pub quantity: i32,
}

#[post("")]
async fn create_shopping_cart(
    auth: AuthMiddleware,
    body: web::Json<NewShoppingCartBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match
        ShoppingCartService::create(
            body.into_inner(),
            auth.user.sub.parse().unwrap(),
            &mut pool.get().unwrap()
        )
    {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(id))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize, Validate)]
pub struct EditShoppingCartBody {
    pub store_id: i32,
    pub product_item_id: i32,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

#[put("/{id}")]
async fn edit_shopping_cart(
    _: AuthMiddleware,
    path: web::Path<i32>,
    body: web::Json<EditShoppingCartBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match ShoppingCartService::edit(body.into_inner(), path.into_inner(), &mut pool.get().unwrap()) {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(e),
    }
}

#[delete("/{id}")]
async fn delete_shopping_cart(
    _: AuthMiddleware,
    path: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let shopping_cart_id = path.into_inner();

    let mut conn = pool.get().unwrap();

    ShoppingCartService::find(&shopping_cart_id, &mut conn)?;

    let order_items = OrderItemService::find_by_shopping_cart(&shopping_cart_id, &mut conn);

    if order_items.is_ok() {
        let order_items_ids_to_delete: Vec<i32> = order_items
            .unwrap()
            .into_iter()
            .map(|o_item| o_item.id)
            .collect();

        OrderItemService::delete(order_items_ids_to_delete, &mut conn)?;
    }

    ShoppingCartService::delete(&shopping_cart_id, &mut conn)?;
    
    Ok(HttpResponse::Ok().finish())
}
