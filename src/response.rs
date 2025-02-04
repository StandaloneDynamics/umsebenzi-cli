use cli_table::Table;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct User {
    pub id: i32,
    username: String,
    email: String,
}
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct ProjectResponse {
    pub id: i32,
    pub created_by: User,
    pub title: String,
    #[table(skip)]
    pub description: String,
    pub code: String,
    pub created_at: String,
    #[table(skip)]
    pub modified_at: String,
}

impl fmt::Display for ProjectResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.code, self.title)
    }
}

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct ProjectTaskResponse {
    pub id: i32,
    title: String,
    code: String,
    created_at: String,
}

impl fmt::Display for ProjectTaskResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.code, self.title)
    }
}

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct SubTaskResponse {
    title: String,
    code: String,
    status: String,
    created_at: String,
}

fn display_subtasks(tasks: &Option<Vec<SubTaskResponse>>) -> impl fmt::Display {
    if let Some(v) = tasks {
        if v.is_empty() {
            format!("No")
        } else {
            format!("Yes")
        }
    } else {
        format!("----")
    }
}

fn display_due_date(tasks: &Option<String>) -> impl fmt::Display {
    if let Some(v) = tasks {
        format!("{}", v)
    } else {
        format!("N/A")
    }
}

fn display_parent(tasks: &Option<i32>) -> impl fmt::Display {
    if let Some(v) = tasks {
        format!("{}", v)
    } else {
        format!("N/A")
    }
}

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct TaskResponse {
    id: i32,
    #[table(skip)]
    pub project: ProjectTaskResponse,
    pub title: String,
    pub code: String,
    pub issue: String,
    #[table(skip)]
    pub description: String,
    #[table(skip)]
    pub created_by: User,
    pub status: String,
    #[table(display_fn = "display_due_date")]
    pub due_date: Option<String>,
    #[table(skip)]
    modified_at: String,
    #[table(display_fn = "display_subtasks")]
    pub subtasks: Option<Vec<SubTaskResponse>>,
    pub assigned_to: User,
    pub created_at: String,
    #[table(display_fn = "display_parent")]
    pub parent: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientErrorResponse {
    pub title: Option<Vec<String>>,
    pub description: Option<Vec<String>>,
    pub code: Option<Vec<String>>,
    pub detail: Option<String>,
    pub non_field_errors: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskErrorResponse {
    pub project_id: Option<Vec<String>>,
    pub title: Option<Vec<String>>,
    pub description: Option<Vec<String>>,
    pub status: Option<Vec<String>>,
    pub issue: Option<Vec<String>>,
    pub due_date: Option<Vec<String>>,
    pub assigned_to_id: Option<Vec<String>>,
    pub parent_id: Option<Vec<String>>,
    pub detail: Option<String>,
    pub non_field_errors: Option<Vec<String>>,
}
