use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AddChainCommand {
    pub name: String,
    pub policy: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct InsertRuleCommand {
    pub position: u32,
    pub rule: String
}