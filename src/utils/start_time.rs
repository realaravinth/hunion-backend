use std::time::{SystemTime, UNIX_EPOCH};

use crate::{END_TIME, START_TIME};

use crate::errors::*;

pub fn chech_time() -> ServiceResult<()> {
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    debug!("{:?}", &current_time);
    if current_time.lt(&START_TIME) {
        Err(ServiceError::TooEarly)
    } else if current_time.gt(&END_TIME) {
        Err(ServiceError::Timeout)
    } else {
        Ok(())
    }
}
