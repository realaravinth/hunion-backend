use crate::models::InsertableUser;
use serde::Serialize;
#[derive(Debug, Serialize)] //, Queryable)]
pub struct Leaderboard {
    pub top_twenty: Vec<InsertableUser>,
    pub you: InsertableUser,
}
