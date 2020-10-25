use serde::{Deserialize, Serialize};

use crate::models::InsertableUser;

impl From<InsertableUser> for User {
    fn from(insert: InsertableUser) -> User {
        let user_id = insert.userid;
        User {
            user_id,
            name: insert.name,
            score: insert.score,
            progress: [
                option_to_bool(insert.one),
                option_to_bool(insert.two),
                option_to_bool(insert.three),
                option_to_bool(insert.four),
                option_to_bool(insert.five),
                option_to_bool(insert.six),
                option_to_bool(insert.seven),
            ],
        }
    }
}

fn option_to_bool(inbound: Option<bool>) -> bool {
    match inbound {
        Some(_) => true,
        None => false,
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct User {
    pub user_id: String,
    pub name: String,
    pub score: i32,
    pub progress: Progress,
}

pub type Progress = [bool; 7];

impl User {
    pub fn new(user_id: String, name: String) -> Self {
        User {
            user_id,
            name,
            score: 0,
            progress: [false; 7], //Progress::new(),
        }
    }
}
