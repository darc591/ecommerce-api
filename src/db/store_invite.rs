use diesel::{ QueryResult, QueryDsl, RunQueryDsl };

use crate::{ models::store_invite::StoreInvite, schema::store_invite::dsl::* };

use super::Connection;

pub fn check_valid(invite_code: &str, conn: &mut Connection) -> Option<i32> {
    let result: QueryResult<StoreInvite> = store_invite
        .find(invite_code)
        .first::<StoreInvite>(conn);

    match result {
        Ok(result) => {
            if result.valid { Some(result.store_id) } else { None }
        }
        Err(_) => None,
    }
}
