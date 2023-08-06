use std::time::SystemTime;

use super::{ Connection, store::StoreService };
use crate::{
    models::{ user::{ User, InsertableUser, UserType }, response::{ TokenResponse, IDResponse } },
    controllers::auth::{ UserSignupPayload, UserLoginPayload },
    utils::{ password_hash::PasswordHash, jwt_auth::TokenClaims, validation::validate },
    error::ServiceError,
};
use diesel::{ RunQueryDsl, ExpressionMethods, QueryDsl };

pub struct UserService;

impl UserService {
    pub fn find_by_email(user_email: &str, conn: &mut Connection) -> Result<User, ServiceError> {
        use crate::schema::user::dsl::*;
        let user_result = user.filter(email.eq(user_email)).get_result(conn);

        match user_result {
            Ok(found_user) => Ok(found_user),
            Err(_) => Err(ServiceError::NotFound { error_message: "User not found!".to_string() }),
        }
    }

    pub fn signup(
        payload: UserSignupPayload,
        conn: &mut Connection
    ) -> Result<IDResponse<i32>, ServiceError> {
        use crate::schema::user::dsl::*;

        validate(&payload)?;

        let existing_user = Self::find_by_email(&payload.email, conn);

        if existing_user.is_ok() {
            return Err(ServiceError::Forbidden {
                error_message: "Email already exists".to_string(),
            });
        }

        let hashed_password = PasswordHash::create_hash(&payload.password);

        let insertable_user: InsertableUser;

        match payload.type_ {
            UserType::ADMIN => {
                if let Some(invite_code) = payload.invite_code {
                    if let Some(store_id) = StoreService::check_store_invite(&invite_code, conn) {
                        insertable_user = InsertableUser {
                            email: payload.email,
                            first_name: payload.first_name,
                            last_name: payload.last_name,
                            managed_store_id: Some(store_id),
                            password: hashed_password.password_hash,
                            salt: hashed_password.salt,
                            type_: UserType::ADMIN as i32,
                        };
                    } else {
                        return Err(ServiceError::Forbidden {
                            error_message: "Invite code is not valid".to_string(),
                        });
                    }
                } else {
                    return Err(ServiceError::Forbidden {
                        error_message: "Invite code is required for admin users".to_string(),
                    });
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
                    type_: UserType::CUSTOMER as i32,
                };
            }
        }

        let result = diesel
            ::insert_into(user)
            .values(insertable_user)
            .returning(id)
            .get_result::<i32>(conn);

        match result {
            Ok(new_id) => Ok(IDResponse { id: new_id }),
            Err(e) => Err(ServiceError::InternalServerError { error_message: e.to_string() }),
        }
    }

    pub fn login(
        payload: UserLoginPayload,
        conn: &mut Connection
    ) -> Result<TokenResponse, ServiceError> {
        use crate::schema::user::dsl::*;

        validate(&payload)?;

        let existing_user = Self::find_by_email(&payload.email, conn)?;

        let user_password = PasswordHash::new(&existing_user.password, &existing_user.salt);

        let password_matches = user_password.verify_password(&payload.password);

        if !password_matches {
            Err(ServiceError::Unauthorized {
                error_message: "Invalid password".to_string(),
            })
        } else {
            let now = diesel
                ::select(diesel::dsl::now)
                .get_result::<SystemTime>(conn)
                .expect("Error getting system time");

            diesel
                ::update(user)
                .filter(id.eq(existing_user.id))
                .set(last_login.eq(now))
                .execute(conn)
                .expect("Error updating last login");

            let token_claims = TokenClaims::new(
                existing_user.type_,
                &existing_user.id.to_string(),
                &existing_user.first_name,
                &existing_user.last_name,
                existing_user.managed_store_id,
            );

            let token_str = token_claims.sign_token().unwrap();

            Ok(TokenResponse { token: token_str })
        }
    }
}
