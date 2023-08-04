use crate::{
    db::{ Pool, address::AddressService },
    constants::{ MESSAGE_CREATED, MESSAGE_OK, MESSAGE_UPDATED },
    error::ServiceError,
    middleware::auth::AuthMiddleware,
    models::{ response::ResponseBody, address::UserAddress },
};
use actix_web::{ delete, get, post, put, web, HttpResponse };
use lazy_static::lazy_static;
use regex::Regex;
use serde::{ Deserialize, Serialize };
use validator::Validate;

lazy_static! {
    static ref REGEX_ONLY_NUMS: Regex = Regex::new(r"^\d+$").unwrap();
}

#[derive(Serialize)]
pub struct FindAddressResponse {
    pub id: i32,
    pub number: String,
    pub city: String,
    pub country: String,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub phone_country_code: Option<String>,
    pub phone_number: Option<String>,
    pub postal_code: String,
}

impl FindAddressResponse {
    pub fn new(address: UserAddress) -> Self {
        Self {
            id: address.id,
            address_line1: address.address_line1,
            address_line2: address.address_line2,
            city: address.city,
            country: address.country,
            number: address.number,
            postal_code: address.postal_code,
            phone_country_code: address.phone_country_code,
            phone_number: address.phone_number,
        }
    }

    pub fn new_vec(addresses: Vec<UserAddress>) -> Vec<Self> {
        addresses
            .into_iter()
            .map(|address| Self {
                id: address.id,
                address_line1: address.address_line1,
                address_line2: address.address_line2,
                city: address.city,
                country: address.country,
                number: address.number,
                postal_code: address.postal_code,
                phone_country_code: address.phone_country_code,
                phone_number: address.phone_number,
            })
            .collect()
    }
}

#[get("/{id}")]
async fn find_address(
    auth: AuthMiddleware,
    path: web::Path<i32>,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user_id = auth.user.sub.parse().unwrap();
    let address_id = path.into_inner();

    let find_result = AddressService::find(&address_id, &user_id, false, &mut pool.get().unwrap());

    match find_result {
        Ok(found_address) =>
            Ok(
                HttpResponse::Ok().json(
                    ResponseBody::new(MESSAGE_OK, FindAddressResponse::new(found_address))
                )
            ),
        Err(e) => Err(ServiceError::NotFound { error_message: e.to_string() }),
    }
}

#[get("")]
async fn list_addresses(
    auth: AuthMiddleware,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match AddressService::list(&user.sub.parse().unwrap(), &mut pool.get().unwrap()) {
        Ok(addresses) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, FindAddressResponse::new_vec(addresses)))),
        Err(e) => Err(ServiceError::NotFound { error_message: e.to_string() }),
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
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match
        AddressService::create(
            body.into_inner(),
            &user.sub.parse().unwrap(),
            &mut pool.get().unwrap()
        )
    {
        Ok(id_res) => {
            Ok(HttpResponse::Created().json(ResponseBody::new(MESSAGE_CREATED, id_res)))
        }
        Err(e) => Err(e),
    }
}

#[put("/{id}")]
async fn edit_address(
    path: web::Path<i32>,
    body: web::Json<CreateAddressBody>,
    auth: AuthMiddleware,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match
        AddressService::edit(
            &path.into_inner(),
            &user.sub.parse().unwrap(),
            body.into_inner(),
            &mut pool.get().unwrap()
        )
    {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_UPDATED, ()))),
        Err(e) => Err(e),
    }
}

#[delete("/{id}")]
async fn delete_address(
    path: web::Path<i32>,
    auth: AuthMiddleware,
    pool: web::Data<Pool>
) -> Result<HttpResponse, ServiceError> {
    let user = auth.user;
    match
        AddressService::delete(
            path.into_inner(),
            user.sub.parse().unwrap(),
            &mut pool.get().unwrap()
        )
    {
        Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(MESSAGE_OK, ()))),
        Err(e) => Err(e),
    }
}
