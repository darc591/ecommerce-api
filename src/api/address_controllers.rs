use crate::{
    config::db::Pool,
    constants::{MESSAGE_CREATED, MESSAGE_OK},
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    models::{
        address::UserAddres,
        response::{IDResponse, ResponseBody},
    },
};
use actix_web::{get, post, web, HttpResponse};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

lazy_static! {
    static ref REGEX_ONLY_NUMS: Regex = Regex::new(r"^\d+$").unwrap();
}

#[get("/{id}")]
async fn find_address(
    path: web::Path<i32>,
    auth: AuthMiddleware,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let address_id = path.into_inner();
    let user = auth.user;
    match UserAddres::find(
        address_id,
        user.sub.parse().unwrap(),
        &mut pool.get().unwrap(),
    ) {
        Ok(values) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, values))),
        Err(e) => Err(ServiceError::NotFound {
            error_message: e.to_string(),
        }),
    }
}

#[get("")]
async fn list_addresses(
    auth: AuthMiddleware,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match UserAddres::list(user.sub.parse().unwrap(), &mut pool.get().unwrap()) {
        Ok(values) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, values))),
        Err(e) => Err(ServiceError::NotFound {
            error_message: e.to_string(),
        }),
    }
}

#[derive(Deserialize, Validate)]
pub struct CreateAddressBody {
    pub number: String,
    #[validate(length(min = 2, max = 60))]
    pub city: String,
    #[validate(length(min = 2, max = 60))]
    pub country: String,
    #[validate(length(min = 2, max = 60))]
    pub address_line1: String,
    #[validate(length(max = 60))]
    pub address_line2: Option<String>,
    #[validate(regex = "REGEX_ONLY_NUMS")]
    pub phone_country_code: Option<String>,
    #[validate(regex = "REGEX_ONLY_NUMS")]
    pub phone_number: Option<String>,
    #[validate(regex = "REGEX_ONLY_NUMS")]
    pub postal_code: String,
}

#[post("")]
async fn create_address(
    body: web::Json<CreateAddressBody>,
    auth: AuthMiddleware,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match UserAddres::create(
        body.into_inner(),
        user.sub.parse().unwrap(),
        &mut pool.get().unwrap(),
    ) {
        Ok(id) => {
            Ok(HttpResponse::Created().json(ResponseBody::new(MESSAGE_CREATED, IDResponse { id })))
        }
        Err(e) => Err(ServiceError::InternalServerError { error_message: e }),
    }
}

//TODO editar
//TODO excluir