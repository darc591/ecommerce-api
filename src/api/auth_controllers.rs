use crate::{
    config::db::Pool,
    constants::MESSAGE_OK,
    error::ServiceError,
    models::{ response::{ ResponseBody, TokenResponse }, user::User },
};
use actix_web::{ web, HttpRequest, HttpResponse, Responder, Result };
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum UserType {
    CUSTOMER,
    ADMIN,
}
#[derive(Serialize, Deserialize)]
pub struct SignupBody {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    invite_code: String,
    user_type: UserType,
}

#[derive(Serialize, Deserialize)]
pub struct FindByEmail {
    email: String,
}

pub async fn signup(body: web::Json<SignupBody>, pool: web::Data<Pool>) -> impl Responder {
    HttpResponse::Ok().json(ResponseBody::new("test", TokenResponse::new("token")))
}

pub async fn find_all(pool: web::Data<Pool>) -> impl Responder {
    let results = User::find_all(&mut pool.get().unwrap()).unwrap();
    HttpResponse::Ok().json(results)
}

pub async fn find_by_email(
    body: web::Json<FindByEmail>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    match User::find_by_email(&body.email, &mut pool.get().unwrap()) {
        Ok(users) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, users))),
        Err(message) =>
            Err(ServiceError::NotFound {
                error_message: "Email not found".to_string(),
            }),
    }
}
