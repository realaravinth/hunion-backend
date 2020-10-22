use crate::payload::register::RegisterRequestPayload;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)] //, Queryable)]
#[table_name = "users"]
pub struct InsertableUser {
    pub userid: String,
    pub name: String,
    pub score: i32,
    pub one: Option<bool>,
    pub two: Option<bool>,
    pub three: Option<bool>,
    pub four: Option<bool>,
    pub five: Option<bool>,
    pub six: Option<bool>,
    pub seven: Option<bool>,
}

impl InsertableUser {
    pub fn new(userid: String, name: String) -> Self {
        InsertableUser {
            userid,
            name,
            score: 0,
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
        }
    }
}

impl From<RegisterRequestPayload> for InsertableUser {
    fn from(inbound: RegisterRequestPayload) -> Self {
        let userid = inbound.userid;
        let name = inbound.name;
        InsertableUser {
            userid,
            name,
            score: 0,
            one: None,
            two: None,
            three: None,
            four: None,
            five: None,
            six: None,
            seven: None,
        }
    }
}
