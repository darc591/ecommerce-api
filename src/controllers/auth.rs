use crate::{
    db::{ Pool, user::UserService },
    error::ServiceError,
    models::{ response::ResponseBody, user::UserType },
};
use actix_web::{ post, web, HttpResponse, Result };
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserSignupPayload {
    #[validate(length(max = 60))]
    pub first_name: String,
    #[validate(length(max = 60))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub invite_code: Option<String>,
    pub type_: UserType,
}

#[post("/signup")]
async fn signup(
    body: web::Json<UserSignupPayload>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match UserService::signup(body.into_inner(), &mut pool.get().unwrap()) {
        Ok(id) => Ok(HttpResponse::Created().json(ResponseBody::new(id))),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize, Validate)]
pub struct UserLoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[post("/login")]
async fn login(
    body: web::Json<UserLoginPayload>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match UserService::login(body.into_inner(), &mut pool.get().unwrap()) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(token_res))),
        Err(e) => Err(e),
    }
}
