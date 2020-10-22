use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct RegisterRequestPayload {
    pub userid: String,
    pub name: String,
}
