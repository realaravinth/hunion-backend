use serde::{Deserialize, Serialize};

use crate::START_TIME_STRING;

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct LoginRequest {
    pub user_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct LoginResponse {
    pub start_time: String,
}

impl LoginResponse {
    pub fn new() -> Self {
        LoginResponse {
            start_time: String::from(START_TIME_STRING.as_str()),
        }
    }
}
