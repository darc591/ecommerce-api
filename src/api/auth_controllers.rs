use crate::{
    config::db::Pool,
    constants::MESSAGE_OK,
    error::ServiceError,
    models::{
        response::{ResponseBody, TokenResponse},
        user::{User, UserPayload},
    },
    utils::password_hash::PasswordHash,
};
use actix_web::{web, HttpRequest, HttpResponse, Responder, Result};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FindByEmail {
    email: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyPassword {
    password: String,
}

pub async fn signup(
    body: web::Json<UserPayload>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match User::signup(body.into_inner(), &mut pool.get().unwrap()) {
        Ok(values) => Ok(HttpResponse::Ok().json(ResponseBody::new("test", values))),
        Err(message) => Err(ServiceError::InternalServerError {
            error_message: message.to_string(),
        }),
    }
}

pub async fn find_all(pool: web::Data<Pool>) -> impl Responder {
    let results = User::find_all(&mut pool.get().unwrap()).unwrap();
    HttpResponse::Ok().json(results)
}

pub async fn find_by_email(
    body: web::Json<FindByEmail>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    match User::find_by_email(&body.email, &mut pool.get().unwrap()) {
        Ok(users) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, users))),
        Err(message) => Err(ServiceError::NotFound {
            error_message: "Email not found".to_string(),
        }),
    }
}

pub async fn verify_password(
    body: web::Json<VerifyPassword>,
) -> Result<HttpResponse, ServiceError> {
    let attemp_password = body.into_inner().password;

    let my_password: PasswordHash = PasswordHash::new(
        "65fd80510930e91125c582bd3b221314e6e9e9a2cb4837d5d211d0e0ee44fa03",
        "Z8mzT3pRa5yVD4jdHyZWA54d",
    );

    if my_password.verify_password(&attemp_password) {
        Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, attemp_password)))
    } else {
        Err(ServiceError::Unauthorized {
            error_message: "not authorized".to_string(),
        })
    }
}
