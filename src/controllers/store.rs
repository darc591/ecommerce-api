use actix_web::{ web, post, HttpResponse };
use serde::Deserialize;
use validator::Validate;
use crate::{
    db::{ Pool, store::StoreService },
    error::ServiceError,
    models::response::ResponseBody,
    middleware::auth::AuthMiddleware,
};

#[derive(Deserialize, Validate)]
pub struct NewStorePayload {
    #[validate(length(min = 2, max = 60))]
    pub store_name: String,
    pub logo_img: Option<String>,
    #[validate(length(max = 60))]
    pub first_name: String,
    #[validate(length(max = 60))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[post("")]
async fn create_store(
    body: web::Json<NewStorePayload>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match StoreService::create(body.into_inner(), &mut pool.get().unwrap()) {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(id))),
        Err(e) => Err(e),
    }
}

#[post("/store-invite")]
async fn create_store_invite(
    auth: AuthMiddleware,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match
        StoreService::create_store_invite(
            &auth.user.managed_store_id.unwrap(),
            &mut pool.get().unwrap()
        )
    {
        Ok(id_res) => Ok(HttpResponse::Created().json(ResponseBody::new(id_res))),
        Err(e) => Err(e),
    }
}
