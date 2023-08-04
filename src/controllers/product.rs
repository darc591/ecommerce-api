use actix_web::{ web, post, HttpResponse };
use serde::Deserialize;
use validator::Validate;

use crate::{
    db::{ Pool, product::ProductService },
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    models::response::ResponseBody,
    constants::MESSAGE_CREATED,
};

#[derive(Deserialize, Validate)]
pub struct CreateCategoryBody {
    #[validate(length(min = 2, max = 60))]
    pub name: String,
    pub store_id: i32,
}

#[post("/categories")]
async fn create_product_category(
    auth: AuthMiddleware,
    body: web::Json<CreateCategoryBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;

    match
        ProductService::create_category(
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
pub struct CreateVariantBody {
    #[validate(length(min = 2, max = 60))]
    pub name: String,
    pub value: String,
    pub store_id: i32,
}

#[post("/variants")]
async fn create_product_variant(
    auth: AuthMiddleware,
    body: web::Json<CreateVariantBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match
        ProductService::create_variant(
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
pub struct ProductDataBody {
    pub description: String,
    pub image: String,
    pub sku: String,
    pub price: f32,
    pub stock: i32,
    pub variant_id: Option<i32>,
}

#[derive(Deserialize, Validate)]
pub struct CreateProductBody {
    #[validate(length(min = 2, max = 60))]
    pub name: String,
    pub category_id: i32,
    pub store_id: i32,
    #[validate]
    pub data: Vec<ProductDataBody>,
}

#[post("")]
async fn create_product(
    auth: AuthMiddleware,
    body: web::Json<CreateProductBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match
        ProductService::create(
            body.into_inner(),
            user.sub.parse().unwrap(),
            &mut pool.get().unwrap()
        )
    {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(MESSAGE_CREATED, id))),
        Err(e) => Err(e),
    }
}
