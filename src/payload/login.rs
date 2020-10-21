use serde::{Deserialize, Serialize};

use crate::START_TIME_STRING;

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct LoginRequest {
    pub userID: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct LoginResponse {
    pub startTime: String,
}

impl LoginResponse {
    pub fn new() -> Self {
        LoginResponse {
            startTime: String::from(START_TIME_STRING.as_str()),
        }
    }
}
