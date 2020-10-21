use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)] //, Queryable)]
pub struct User {
    pub userID: String,
    pub name: String,
    pub score: usize,
    pub progress: [bool; 7],
}

impl User {
    pub fn new(userID: String, name: String) -> Self {
        User {
            userID,
            name,
            score: 0,
            progress: [false; 7],
        }
    }
}
