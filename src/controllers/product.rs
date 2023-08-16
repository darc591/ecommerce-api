use actix_web::{ web, post, get, HttpResponse };
use serde::Deserialize;
use validator::Validate;

use crate::{
    db::{ Pool, product::ProductService },
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    models::{ response::ResponseBody, user::UserType },
};

#[derive(Deserialize, Validate)]
pub struct CreateCategoryBody {
    #[validate(length(min = 2, max = 60))]
    pub name: String,
}

#[post("/categories")]
async fn create_product_category(
    auth: AuthMiddleware,
    body: web::Json<CreateCategoryBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let (user_id, store_id) = (auth.user.sub.parse().unwrap(), auth.user.managed_store_id.unwrap());

    match
        ProductService::create_category(
            body.into_inner(),
            &store_id,
            &user_id,
            &mut pool.get().unwrap()
        )
    {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(id))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateVariantBody {
    #[validate(length(min = 2, max = 60))]
    pub name: String,
    pub value: String,
}

#[post("/variants")]
async fn create_product_variant(
    auth: AuthMiddleware,
    body: web::Json<CreateVariantBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let (user_id, store_id) = (auth.user.sub.parse().unwrap(), auth.user.managed_store_id.unwrap());

    match
        ProductService::create_variant(
            body.into_inner(),
            &user_id,
            &store_id,
            &mut pool.get().unwrap()
        )
    {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(id))),
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
    #[validate]
    pub data: Vec<ProductDataBody>,
}

#[post("")]
async fn create_product(
    auth: AuthMiddleware,
    body: web::Json<CreateProductBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let (user_id, store_id) = (auth.user.sub.parse().unwrap(), auth.user.managed_store_id.unwrap());

    match ProductService::create(body.into_inner(), &user_id, &store_id, &mut pool.get().unwrap()) {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(id))),
        Err(e) => Err(e),
    }
}

#[get("/variants")]
async fn list_variants(
    auth: AuthMiddleware,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let (user_type, store_id) = (
        UserType::from_i32(auth.user.type_),
        auth.user.managed_store_id.unwrap(),
    );

    if user_type == UserType::CUSTOMER {
        return Err(ServiceError::Forbidden {
            error_message: "User without permissions".to_string(),
        });
    }

    match ProductService::list_variants(&store_id, &mut pool.get().unwrap()) {
        Ok(values) => Ok(HttpResponse::Ok().json(ResponseBody::new(values))),
        Err(e) => Err(e),
    }
}
