use crate::models::InsertableUser;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize)] //, Queryable)]
pub struct Leaderboard {
    pub topTwenty: Vec<InsertableUser>,
    pub you: InsertableUser,
}
