use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub id: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub problem: String,
    pub root_cause: String,
    pub solution: String,
    pub confidence: u32,
}
