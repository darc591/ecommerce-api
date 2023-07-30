use validator::Validate;

use crate::error::ServiceError;

pub fn validate<T>(payload: &T) -> Result<(), ServiceError> where T: Validate {
    match payload.validate() {
        Ok(_) => Ok(()),
        Err(e) => Err(ServiceError::BadRequest { error_message: e.to_string() }),
    }
}
