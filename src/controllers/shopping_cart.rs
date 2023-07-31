use actix_web::{ web, post, HttpResponse };
use serde::Deserialize;

use crate::{
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    db::{ Pool, shopping_cart },
    models::response::ResponseBody,
    constants::MESSAGE_CREATED,
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
    let user = auth.user;
    match
        shopping_cart::create(
            body.into_inner(),
            user.sub.parse().unwrap(),
            &mut pool.get().unwrap()
        )
    {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(MESSAGE_CREATED, id))),
        Err(e) => Err(e),
    }
}
