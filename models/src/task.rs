use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, FromRow)]
pub struct NewTask {
    pub creator: String,
    pub branch: String,
    pub svn_merge_number: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default, FromRow)]
pub struct Task {
    pub id: i64,
    pub created_at: String,
    pub creator: String,
    pub branch: String,
    pub svn_merge_number: String,
    pub status: String,
}
