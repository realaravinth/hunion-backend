use std::time::{SystemTime, UNIX_EPOCH};

use crate::{StringNumer, END_TIME, START_TIME};

use crate::errors::*;

pub fn chech_time() -> ServiceResult<()> {
    let c_time = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let current_time = StringNumer::Numeric(c_time.as_secs());

    if current_time.lt(&START_TIME) {
        Err(ServiceError::TooEarly)
    } else if current_time.gt(&END_TIME) {
        Err(ServiceError::Timeout)
    } else {
        Ok(())
    }
}
