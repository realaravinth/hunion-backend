use crate::errors::*;
use actix_identity::Identity;

pub async fn check(id: &Identity) -> ServiceResult<bool> {
    debug!("{:?}", id.identity());
    // access request identity
    if let Some(id) = id.identity() {
        Ok(true)
    } else {
        Err(ServiceError::AuthorizationRequired)
    }
}
