use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskRequest {
    pub project_id: i32,
    pub title: String,
    pub description: String,
    pub status: i32,
    pub issue: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    pub assigned_to_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i32>,
}
