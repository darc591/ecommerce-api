use std::time::SystemTime;

use super::{ Connection, store_invite };
use crate::{
    models::{ user::{ User, InsertableUser, UserType }, response::{ TokenResponse, IDResponse } },
    controllers::auth::{ UserSignupPayload, UserLoginPayload },
    utils::{ password_hash::PasswordHash, jwt_auth::TokenClaims, validation::validate },
    error::ServiceError,
};
use diesel::{ QueryResult, RunQueryDsl, ExpressionMethods, QueryDsl };

pub fn find_by_email(user_email: &str, conn: &mut Connection) -> QueryResult<User> {
    use crate::schema::user::dsl::*;
    user.filter(email.eq(user_email)).get_result(conn)
}

pub fn signup(
    payload: UserSignupPayload,
    conn: &mut Connection
) -> Result<IDResponse<i32>, ServiceError> {
    use crate::schema::user::dsl::*;

    validate(&payload)?;

    let existing_user = find_by_email(&payload.email, conn);

    if existing_user.is_ok() {
        return Err(ServiceError::Forbidden { error_message: "Email already exists".to_string() });
    }

    let hashed_password = PasswordHash::create_hash(&payload.password);

    let insertable_user: InsertableUser;

    match payload.type_ {
        UserType::ADMIN => {
            if let Some(invite_code) = payload.invite_code {
                if let Some(store_id) = store_invite::check_valid(&invite_code, conn) {
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

    let existing_user = find_by_email(&payload.email, conn);

    match existing_user {
        Ok(curr_user) => {
            let user_password = PasswordHash::new(&curr_user.password, &curr_user.salt);

            let matches = user_password.verify_password(&payload.password);

            if !matches {
                Err(ServiceError::Unauthorized { error_message: "Invalid password".to_string() })
            } else {
                let now = diesel
                    ::select(diesel::dsl::now)
                    .get_result::<SystemTime>(conn)
                    .expect("Error getting system time");

                diesel
                    ::update(user)
                    .filter(id.eq(curr_user.id))
                    .set(last_login.eq(now))
                    .execute(conn)
                    .expect("Error updating last login");

                let token_claims = TokenClaims::new(
                    curr_user.type_,
                    &curr_user.id.to_string(),
                    &curr_user.first_name,
                    &curr_user.last_name
                );

                let token_str = token_claims.sign_token().unwrap();

                Ok(TokenResponse { token: token_str })
            }
        }
        Err(_) => Err(ServiceError::NotFound { error_message: "User does not exist".to_string() }),
    }
}
