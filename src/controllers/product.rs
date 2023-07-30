use actix_web::{ web, post, HttpResponse };
use serde::Deserialize;
use validator::Validate;

use crate::{
    db::{ Pool, product },
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    models::response::ResponseBody,
    constants::MESSAGE_OK,
};

#[derive(Deserialize, Validate)]
pub struct CreateCategoryBody {
    #[validate(length(min = 2, max = 60))]
    pub name: String,
}

#[post("{store_id}/product-category")]
async fn create_product_category(
    auth: AuthMiddleware,
    path: web::Path<i32>,
    body: web::Json<CreateCategoryBody>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;

    match
        product::create_category(
            body.into_inner(),
            path.into_inner(),
            user.sub.parse().unwrap(),
            &mut pool.get().unwrap()
        )
    {
        Ok(id) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, id))),
        Err(e) => Err(e),
    }
}
