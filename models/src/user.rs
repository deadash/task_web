use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub computer_name: String,
    pub ip: String,
}