use actix_web::{ web, post, put, HttpResponse };
use serde::Deserialize;
use validator::Validate;
use crate::{
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    db::{ Pool, shopping_cart },
    models::response::ResponseBody,
    constants::{ MESSAGE_CREATED, MESSAGE_OK },
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

#[derive(Deserialize, Validate)]
pub struct EditShoppingCartBody {
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
    match
        shopping_cart::edit(
            body.into_inner(),
            path.into_inner(),
            &mut pool.get().unwrap()
        )
    {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, ""))),
        Err(e) => Err(e),
    }
}
