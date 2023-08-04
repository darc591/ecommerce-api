use crate::{
    controllers::store::NewStorePayload,
    models::{
        response::IDResponse,
        store::InsertableStore,
        user::{ InsertableUser, UserType },
        payment_method::InsertablePaymentMethod,
        shipping::InsertableShippingMethod,
        store_invite::StoreInvite,
    },
    utils::{ password_hash::PasswordHash, validation::validate },
    error::ServiceError,
};
use diesel::prelude::*;
use super::{ Connection, user as db_user };

pub struct StoreService;

impl StoreService {
    pub fn check_store_admin(
        store_id: i32,
        user_id: i32,
        conn: &mut Connection
    ) -> Result<(), ServiceError> {
        use crate::schema::{ user, store };

        let store_admins: QueryResult<Vec<i32>> = store::table
            .inner_join(user::table)
            .filter(user::dsl::managed_store_id.eq(store_id))
            .select(user::dsl::id)
            .load::<i32>(conn);

        match store_admins {
            Ok(admin_ids) => {
                if admin_ids.is_empty() {
                    return Err(ServiceError::NotFound {
                        error_message: "Store not found".to_string(),
                    });
                } else if admin_ids.contains(&user_id) {
                    return Ok(());
                } else {
                    Err(ServiceError::Forbidden {
                        error_message: "User without permissions for this store".to_string(),
                    })
                }
            }
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }

    pub fn check_store_invite(invite_code: &str, conn: &mut Connection) -> Option<i32> {
        use crate::schema::store_invite;
        
        let result: QueryResult<StoreInvite> = store_invite::dsl::store_invite
            .find(invite_code)
            .first::<StoreInvite>(conn);

        match result {
            Ok(result) => {
                if result.valid { Some(result.store_id) } else { None }
            }
            Err(_) => None,
        }
    }

    pub fn create(
        payload: NewStorePayload,
        conn: &mut Connection
    ) -> Result<IDResponse<i32>, ServiceError> {
        use crate::schema::{ store, user, payment_method, shipping_method };

        validate(&payload)?;

        let existing_user = db_user::find_by_email(&payload.email, conn);

        if existing_user.is_ok() {
            return Err(ServiceError::Forbidden {
                error_message: "Email already exists".to_string(),
            });
        }

        let new_store = InsertableStore {
            name: payload.store_name,
            logo_url: payload.logo_img,
        };

        match
            diesel
                ::insert_into(store::dsl::store)
                .values(new_store)
                .returning(store::dsl::id)
                .get_result::<i32>(conn)
        {
            Ok(new_store_id) => {
                let hashed_password = PasswordHash::create_hash(&payload.password);

                let new_user = InsertableUser {
                    email: payload.email,
                    first_name: payload.first_name,
                    last_name: payload.last_name,
                    password: hashed_password.password_hash,
                    salt: hashed_password.salt,
                    type_: UserType::ADMIN as i32,
                    managed_store_id: Some(new_store_id),
                };

                let user_result = diesel
                    ::insert_into(user::dsl::user)
                    .values(new_user)
                    .execute(conn);

                if user_result.is_err() {
                    return Err(ServiceError::InternalServerError {
                        error_message: user_result.unwrap_err().to_string(),
                    });
                }

                let default_payment_methods: Vec<InsertablePaymentMethod> = vec![
                    "Bank transfer",
                    "Credit card",
                    "Debit card",
                    "PayPal",
                    "Cash"
                ]
                    .into_iter()
                    .map(|method| InsertablePaymentMethod {
                        name: method.to_string(),
                        store_id: new_store_id,
                    })
                    .collect();

                let payment_method_result = diesel
                    ::insert_into(payment_method::dsl::payment_method)
                    .values(default_payment_methods)
                    .execute(conn);

                if payment_method_result.is_err() {
                    return Err(ServiceError::InternalServerError {
                        error_message: payment_method_result.unwrap_err().to_string(),
                    });
                }

                let default_shipping_methods: Vec<InsertableShippingMethod> = vec!["Ups", "FedEx"]
                    .into_iter()
                    .map(|method| InsertableShippingMethod {
                        name: method.to_string(),
                        store_id: new_store_id,
                    })
                    .collect();

                let shipping_method_result = diesel
                    ::insert_into(shipping_method::dsl::shipping_method)
                    .values(default_shipping_methods)
                    .execute(conn);

                if shipping_method_result.is_err() {
                    return Err(ServiceError::InternalServerError {
                        error_message: shipping_method_result.unwrap_err().to_string(),
                    });
                }
                return Ok(IDResponse { id: new_store_id });
            }
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }
}
