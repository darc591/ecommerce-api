use crate::{
    controllers::store::NewStorePayload,
    models::{
        store::InsertableStore,
        user::{ InsertableUser, UserType },
        payment_method::InsertablePaymentMethod,
        shipping_method::InsertableShippingMethod,
    },
    utils::password_hash::PasswordHash,
};
use diesel::RunQueryDsl;
use validator::Validate;
use super::{ Connection, user as db_user };

pub fn create(payload: NewStorePayload, conn: &mut Connection) -> Result<i32, String> {
    use crate::schema::{ store, user, payment_method, shipping_method };
    match payload.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(e.to_string());
        }
    }

    let existing_user = db_user::find_by_email(&payload.email, conn);

    if existing_user.is_ok() {
        return Err("Email already exists".to_string());
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

            let user_result = diesel::insert_into(user::dsl::user).values(new_user).execute(conn);

            if user_result.is_err() {
                return Err("Error creating user".to_string());
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
                return Err("Error creating default payment methods".to_string());
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
                return Err("Error creating default shipping methods".to_string());
            }
            return Ok(new_store_id);
        }
        Err(_) => Err("Error creating store".to_string()),
    }
}
