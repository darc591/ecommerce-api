use crate::{config::db::Connection, schema::user::dsl::*, utils::password_hash::PasswordHash};

use chrono::NaiveDateTime;
use diesel::{ExpressionMethods, Insertable, QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use validator::{Validate, ValidationError, ValidationErrors};

use super::store_invite::StoreInvite;

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
#[derive(Serialize, Deserialize, Validate, Insertable)]
#[diesel(table_name = user)]
pub struct UserPayload {
    #[validate(length(max = 60))]
    first_name: String,
    #[validate(length(max = 60))]
    last_name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 6))]
    password: String,
    invite_code: Option<String>,
    type_: UserType,
}

impl User {
    pub fn signup(user_payload: UserPayload, conn: &mut Connection) -> Result<UserPayload, String> {
        //validation
        match user_payload.validate() {
            Ok(_) => (),
            Err(e) => return Err(e.to_string()),
        };
        //find user
        let existing_user = Self::find_by_email(&user_payload.email, conn);

        if existing_user.is_ok() {
            return Err("Email already exists".to_string());
        }

        let hashed_password = PasswordHash::create_hash(&user_payload.password);

        match user_payload.type_ {
            UserType::ADMIN => {
                if let Some(invite_code) = user_payload.invite_code {
                    if !StoreInvite::check_valid(&invite_code, conn) {
                        Err("Invite code is not valid".to_string())
                    } else {
                        todo!()
                    }
                } else {
                    Err("Invite code is required for admin users".to_string())
                }
            }
            UserType::CUSTOMER => diesel::insert_into(user).values(records),
        }
    }

    pub fn find_all(conn: &mut Connection) -> QueryResult<Vec<User>> {
        user.order(id.asc()).load::<User>(conn)
    }

    pub fn find_by_email(user_email: &str, conn: &mut Connection) -> QueryResult<User> {
        user.filter(email.eq(user_email)).get_result::<User>(conn)
    }
}
