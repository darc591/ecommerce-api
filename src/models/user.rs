use super::{response::TokenResponse, store_invite::StoreInvite};
use crate::{
    api::auth_controllers::{UserLoginPayload, UserSignupPayload},
    config::db::Connection,
    schema::user::dsl::*,
    utils::{auth::TokenClaims, password_hash::PasswordHash},
};
use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::time::SystemTime;
use validator::Validate;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub type_: i32,
    pub created_at: NaiveDateTime,
    pub first_name: String,
    pub last_login: NaiveDateTime,
    pub last_name: String,
    pub managed_store_id: Option<i32>,
    pub updated_at: NaiveDateTime,
    pub salt: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum UserType {
    CUSTOMER,
    ADMIN,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user)]
struct InsertableUser {
    email: String,
    password: String,
    type_: i32,
    first_name: String,
    last_name: String,
    managed_store_id: Option<i32>,
    updated_at: SystemTime,
    salt: String,
}

impl User {
    pub fn signup(payload: UserSignupPayload, conn: &mut Connection) -> Result<String, String> {
        match payload.validate() {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        };
        //find user
        let existing_user = Self::find_by_email(&payload.email, conn);

        if existing_user.is_ok() {
            return Err("Email already exists".to_string());
        }

        let hashed_password = PasswordHash::create_hash(&payload.password);

        let insertable_user: InsertableUser;

        let now = diesel::select(diesel::dsl::now)
            .get_result::<SystemTime>(conn)
            .expect("Error getting system time");

        match payload.type_ {
            UserType::ADMIN => {
                if let Some(invite_code) = payload.invite_code {
                    if let Some(store_id) = StoreInvite::check_valid(&invite_code, conn) {
                        insertable_user = InsertableUser {
                            email: payload.email,
                            first_name: payload.first_name,
                            last_name: payload.last_name,
                            managed_store_id: Some(store_id),
                            password: hashed_password.password_hash,
                            salt: hashed_password.salt,
                            updated_at: now,
                            type_: UserType::ADMIN as i32,
                        };
                    } else {
                        return Err("Invite code is not valid".to_string());
                    }
                } else {
                    return Err("Invite code is required for admin users".to_string());
                }
            }
            UserType::CUSTOMER => {
                insertable_user = InsertableUser {
                    email: payload.email,
                    first_name: payload.first_name,
                    last_name: payload.last_name,
                    managed_store_id: None,
                    password: hashed_password.password_hash,
                    salt: hashed_password.salt,
                    updated_at: now,
                    type_: UserType::CUSTOMER as i32,
                };
            }
        }

        let result = diesel::insert_into(user)
            .values(insertable_user)
            .execute(conn);

        match result {
            Ok(_) => Ok("Created".to_string()),
            Err(_) => Err("Error creating user".to_string()),
        }
    }

    pub fn login(
        payload: UserLoginPayload,
        conn: &mut Connection,
    ) -> Result<TokenResponse, String> {
        match payload.validate() {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        };

        let existing_user = Self::find_by_email(&payload.email, conn);

        match existing_user {
            Ok(curr_user) => {
                let user_password = PasswordHash::new(&curr_user.password, &curr_user.salt);

                let matches = user_password.verify_password(&payload.password);

                if !matches {
                    Err("Invalid password".to_string())
                } else {
                    let now = diesel::select(diesel::dsl::now)
                        .get_result::<SystemTime>(conn)
                        .expect("Error getting system time");

                    diesel::update(user)
                        .filter(id.eq(curr_user.id))
                        .set(last_login.eq(now))
                        .execute(conn)
                        .expect("Error updating last login");

                    let token_claims = TokenClaims::new(
                        curr_user.type_,
                        &curr_user.id.to_string(),
                        &curr_user.first_name,
                        &curr_user.last_name,
                    );

                    let token_str = token_claims.sign_token().unwrap();

                    Ok(TokenResponse { token: token_str })
                }
            }
            Err(_) => Err("User does not exist".to_string()),
        }
    }

    fn find_by_email(user_email: &str, conn: &mut Connection) -> QueryResult<User> {
        user.filter(email.eq(user_email)).get_result::<User>(conn)
    }
}
