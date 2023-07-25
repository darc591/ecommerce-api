use crate::{config::db::Connection, schema::store_invite::dsl::*};
use chrono::NaiveDateTime;
use diesel::{QueryDsl, QueryResult, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct StoreInvite {
    pub id: String,
    pub valid: bool,
    pub created_at: NaiveDateTime,
    pub store_id: i32,
}

impl StoreInvite {
    pub fn check_valid(invite_code: &str, conn: &mut Connection) -> bool {
        let result: QueryResult<StoreInvite> =
            store_invite.find(invite_code).first::<StoreInvite>(conn);

        match result {
            Ok(result) => {
                if result.valid {
                    return true;
                } else {
                    return false;
                }
            }
            Err(_) => return false,
        }
    }
}
