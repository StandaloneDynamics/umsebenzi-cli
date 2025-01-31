use serde::{Serialize, Deserialize};
use cli_table::Table;
use std::{fmt, ops::Sub};

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct User {
    id: i32,
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
struct ProjectTaskResponse{
    title: String,
    code: String,
    created_at: String
}

impl fmt::Display for ProjectTaskResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.code, self.title)
    }
}




#[derive(Serialize, Deserialize, Debug, Table)]
pub struct SubTaskResponse{
    title: String,
    code: String,
    created_at: String
}

fn display_subtasks(tasks: &Option<Vec<SubTaskResponse>>) -> impl fmt::Display{
    if let Some(v) = tasks{
        if v.is_empty(){
            format!("No")
        }else{
            format!("Yes")
        }
    }else{
        format!("----")
    }
}

#[derive(Serialize, Deserialize, Debug, Table)]
pub struct TaskResponse{
    #[table(skip)]
    project: ProjectTaskResponse,
    pub title: String,
    pub code: String,
    #[table(skip)]
    pub description: String,
    assigned_to: User,
    #[table(skip)]
    pub created_by: User,
    pub status: String,
    pub due_date: String,
    pub created_at: String,
    #[table(skip)]
    modified_at: String,
    #[table(display_fn="display_subtasks")]
    pub subtasks: Option<Vec<SubTaskResponse>>,
    issue: String
}