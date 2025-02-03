use clap::{Parser, Subcommand};


use crate::service::{delete_confirmation, get_request, RequestType, CLIENT_ERROR, CLIENT_RESPONSE_ERROR};
use crate::response::{TaskResponse, ClientErrorResponse, TaskErrorResponse};
use colored::Colorize;
use cli_table::{print_stdout, WithTitle};
use std::collections::HashMap;
use std::io::{self, Write};
use crate::description::text_editor;
use crate::defaults::{show_issue_options, show_status_options};
use crate::enums::{TaskStatus, Issue};
use crate::request::TaskRequest;


const TASK_ENDPOINT: &str = "/tasks/";
const TASK_TITLE_ERROR: &str = "Task title expected";
const TASK_DESCRIPTION_ERROR: &str = "Task description expected";
const TASK_ISSUE_ERROR: &str = "Task issue expected";
const TASK_PROJECT_ERROR: &str = "Task project expected";
const TASK_PARENT_ERROR: &str = "Task parent expected";
const TASK_DATE_ERROR: &str = "Task date expected";
const TASK_STATUS_ERROR: &str = "Task status expected";

#[derive(Subcommand, Debug)]
enum TaskCLI{
    List,
    Add,
    Detail { task_id: String },
    Edit { task_id: String },
    Delete { task_id: String },
    Status {task_id: String, status: String}
} 

#[derive(Parser, Debug)]
pub struct TaskArgs{
    #[command(subcommand)]
    command: TaskCLI,
}

pub fn run(args: TaskArgs){
    match args.command {
        TaskCLI::Add => add(),
        TaskCLI::List => list(),
        TaskCLI::Edit { task_id } => edit(task_id),
        TaskCLI::Detail { task_id } => detail(task_id),
        TaskCLI::Delete { task_id } => delete(task_id),
        TaskCLI::Status { task_id, status } => status_update(task_id, status),

        
    }
}


fn list(){
    let request = match get_request(TASK_ENDPOINT, None){
        Ok(r) => r,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red().bold());
            std::process::exit(1)
        }
    };

    let resp = match request.client.get(request.url).send() {
        Ok(r) => r,
        Err(err) => {
            eprintln!("{}: {err}", CLIENT_RESPONSE_ERROR.red().bold());
            std::process::exit(1);
        }
        
    };

    if resp.status().is_success() {
        let proj: Vec<TaskResponse> = match resp.json() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{}: {err:?}", "Unable to parse response json".red());
                std::process::exit(1);
            }
        };
        let _ = print_stdout(proj.with_title()).is_ok();
    } else {
        println!("{} {}", "Error".red().bold(), resp.status());
    }
}

fn detail(task_id: String){
    let request = match get_request(TASK_ENDPOINT, Some(&task_id)) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    let resp = match request.client.get(request.url).send() {
        Ok(r) => r,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
            std::process::exit(1);
        }
    };
    if resp.status().is_success(){
        let task: TaskResponse = match resp.json() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{}: {err}", "Unable to parse response json".red().bold());
                std::process::exit(1);
            }
        };
        println!("{}: {}", "Title".green().bold(),task.title);
        println!("{}: {}", "Code".green().bold(), task.code);
        println!("{}: {}", "Status".green().bold(),task.status);
        println!("{}: {}", "Due Date".green().bold(), task.due_date.unwrap_or_default());
        println!("{}: {}", "Created By".green().bold(), task.created_by);
        println!("{}: {}", "Created At".green().bold(), task.created_at);
        println!("");
        println!("{}:", "Description".green().bold());
        println!("{}", task.description);
        println!("");
        
        if let Some(t) = task.subtasks {
            if !t.is_empty(){
                println!("{}:", "Subtasks".green().bold());
                let _ = print_stdout(t.with_title()).is_ok();
            }
        }
    }
}

fn delete(task_id: String){
    let is_delete = delete_confirmation(&task_id, RequestType::TASK);
    if is_delete{
        let request = match get_request(TASK_ENDPOINT,Some(&task_id)) {
            Ok(c) => c,
            Err(err) => {
                eprint!("{}: {err}", CLIENT_ERROR.red());
                std::process::exit(1);
            }
        };
        let resp = match request.client.delete(request.url).send() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
                std::process::exit(1);
            }
        };
        if resp.status().is_success() {
            println!("{}","Task Deleted".green().bold());
        } else if resp.status().is_client_error() {
            eprintln!("{}: Unable to delete Task:  {:?}", "Error".red(), resp);
            std::process::exit(1);
        }
    }

}


fn add(){
    println!("{}", "Create a new task".green().bold());

    print!("{}: ", "Project [ID]".green().bold());
    let _ = io::stdout().flush();
    let mut project_buf = String::new();
    io::stdin()
        .read_line(&mut project_buf)
        .expect(&TASK_PROJECT_ERROR.red().bold());
    if project_buf.trim().is_empty() {
        eprintln!("{}", TASK_PROJECT_ERROR.red().bold());
        std::process::exit(1);
    }
    let proj_id = match project_buf.trim().parse::<i32>(){
        Ok(i) => i,
        Err(_) =>{
            eprintln!("{}", "Project ID is should be a number".red().bold());
        std::process::exit(1);
        }
    };

    print!("{}: ", "Title".green().bold());
    let _ = io::stdout().flush();
    let mut title_buf = String::new();
    io::stdin()
        .read_line(&mut title_buf)
        .expect(&TASK_TITLE_ERROR.red().bold());
    if title_buf.trim().is_empty() {
        eprintln!("{}", TASK_TITLE_ERROR.red().bold());
        std::process::exit(1);
    }

    print!("{}: ","Description [type 'Y' to open editor]".green().bold());
    let _ = io::stdout().flush();
    let mut answer_buf = String::new();
    io::stdin()
        .read_line(&mut answer_buf)
        .expect(&TASK_DESCRIPTION_ERROR.red());
    let mut description = String::new();
    if answer_buf.trim() == "Y"{
        description = text_editor(None).expect(&TASK_DESCRIPTION_ERROR.red().bold());
    }else{
        eprintln!("{}", "Invalid Command expected Y".red().bold());
        std::process::exit(1);
    }

    show_issue_options();
    print!("{}: ", "Issue [default=1]".green().bold());
    let _ = io::stdout().flush();
    let mut issue_buf = String::new();
    io::stdin()
        .read_line(&mut issue_buf)
        .expect(&TASK_ISSUE_ERROR.red().bold());
    if issue_buf.trim().is_empty() {
        issue_buf = "1".to_string();
    }
    let issue = match Issue::from_str(&issue_buf.trim()){
        Ok(i) => i,
        Err(err) => {
            eprintln!("{}: {err}", TASK_ISSUE_ERROR.red().bold());
            std::process::exit(1);
        }
    };
    
    let mut parent_id = None;
    if issue == Issue::SubTask{
        print!("{}: ", "Parent Task ID".green().bold());
        let _ = io::stdout().flush();
        let mut parent_buf = String::new();
        io::stdin()
            .read_line(&mut parent_buf)
            .expect(&TASK_PARENT_ERROR.red().bold());
        if parent_buf.trim().is_empty() {
            eprintln!("{}", TASK_PARENT_ERROR.red().bold());
            std::process::exit(1);
        }
        match parent_buf.trim().parse::<i32>(){
            Ok(i) => {parent_id = Some(i);},
            Err(_) =>{
                eprintln!("{}", "Task parent ID is should be a number".red().bold());
                std::process::exit(1);
            }
        };

    }
    show_status_options(false);
    print!("{}: ", "Status [default=1]".green().bold());
    let _ = io::stdout().flush();
    let mut status_buf = String::new();
    io::stdin()
        .read_line(&mut status_buf).unwrap();
    if status_buf.trim().is_empty() {
        status_buf = "1".to_string();
    }
    let status = match TaskStatus::from_str(&status_buf.trim()){
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}: {err}", TASK_STATUS_ERROR.red().bold());
            std::process::exit(1);
        }
    };

    let mut due_date = None;
    print!("{}: ", "Add due date [Y/N]?".green().bold());
    let _ = io::stdout().flush();
    let mut date_buf = String::new();
    io::stdin()
        .read_line(&mut date_buf)
        .expect(&"answer Y or N".red().bold());
    if date_buf.trim().is_empty() {
        eprintln!("{}", &"answer Y or N".red().bold());
        std::process::exit(1);
    }
    if date_buf.trim() == "Y"{
        print!("{}: ", "Due Date [YYYY-MM-DD]".green().bold());
        let _ = io::stdout().flush();
        let mut due_buf = String::new();
        io::stdin()
            .read_line(&mut due_buf)
            .expect(&TASK_DATE_ERROR.red().bold());
        if due_buf.trim().is_empty() {
            eprintln!("{}", TASK_DATE_ERROR.red().bold());
            std::process::exit(1);
        }
        due_date = Some(due_buf.trim().to_string());
    }
    let task_request = TaskRequest{
        project_id: proj_id,
        title: title_buf,
        description: description,
        status: status.to_value(),
        issue: issue.to_value(),
        assigned_to_id: 1,
        parent_id: parent_id,
        due_date: due_date
    };

    let request = match get_request(TASK_ENDPOINT, None) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    let resp = match request.client.post(request.url).json(&task_request).send() {
        Ok(r) => r,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
            std::process::exit(1);
        }
    };
    if resp.status().is_success() {
        println!("{}", "task created".green().bold())
    } else if resp.status().is_client_error() {
        let response: TaskErrorResponse = resp.json().unwrap();
        println!("{}: {:?}", "error".red(), response);
    } else {
        println!("{}: {}", "Error".red().bold(),resp.status());
        println!("{}: {}", "Error".red().bold(), resp.text().unwrap());
    };


    
}

fn status_update(task_id: String, status: String){
    let new_status = match TaskStatus::from_str(&status){
        Ok(s) => s,
        Err(err) => {
            eprintln!("{}: {err}", TASK_STATUS_ERROR.red().bold());
            std::process::exit(1);
        }
    };
    let request = match get_request(TASK_ENDPOINT, Some(&task_id)) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    let mut data  = HashMap::new();
    data.insert("status", new_status.to_value());

    let resp = match request.client.patch(request.url).json(&data).send() {
        Ok(r) => r,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
            std::process::exit(1);
        }
    };
    if resp.status().is_success() {
        println!("{}", "Task status updated".green().bold())
    } else if resp.status().is_client_error() {
        let response: TaskErrorResponse = resp.json().unwrap();
        println!("{}: {:?}", "error".red(), response);
    } else {
        println!("{}: {}", "Error".red().bold(),resp.status());
        println!("{}: {}", "Error".red().bold(), resp.text().unwrap());
    };


}


fn edit(task_id: String){
    let task: TaskResponse;
    let request = match get_request(TASK_ENDPOINT, Some(&task_id)) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    let resp = match request.client.get(request.url).send() {
        Ok(r) => r,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
            std::process::exit(1);
        }
    };
    // Update existing task
    if resp.status().is_success() {
        task = match resp.json() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{} {err}", "Unable to parse response json".red());
                std::process::exit(1);
            }
        };
        
        print!("{}: ","Title [leave blank to use existing]".green().bold());
        let _ = io::stdout().flush();
        let mut title_buf = String::new();
        io::stdin()
            .read_line(&mut title_buf)
            .expect(&TASK_TITLE_ERROR.red());
        if title_buf.trim().is_empty() {
            title_buf = task.title
        }

        print!("{}: ", "Description: [Type E to edit. leave blank to use existing]".green().bold());
        let _ = io::stdout().flush();
        let mut description_buf = String::new();
        io::stdin()
            .read_line(&mut description_buf)
            .expect(&TASK_DESCRIPTION_ERROR.red());
        if description_buf.trim().is_empty() {
            description_buf = task.description
        }else if description_buf.trim() == "E" {
            description_buf =  text_editor(Some(task.description)).expect(&TASK_DESCRIPTION_ERROR.red());
        }

        show_issue_options();
        print!("{}: ", "Issue [leave blank to use existing]".green().bold());
        let _ = io::stdout().flush();
        let mut issue_buf = String::new();
        io::stdin()
            .read_line(&mut issue_buf)
            .expect(&TASK_ISSUE_ERROR.red().bold());
        if issue_buf.trim().is_empty() {
            issue_buf = Issue::from_api_str(&task.issue).expect("invalid task issue");
        }
        let issue = match Issue::from_str(&issue_buf.trim()){
            Ok(i) => i,
            Err(err) => {
                eprintln!("{}: {err}", TASK_ISSUE_ERROR.red().bold());
                std::process::exit(1);
            }
        };
        let current_issue = Issue::from_api_str(&task.issue).expect("Invalid task status");
        if current_issue == Issue::EPIC && issue == Issue::SUBTASK{
            // if changing from Epic to subtask, must have parent.
        }else if current_issue == Issue::SUBTASK && issue == Issue::EPIC{
            // if changing from subtask to epic, remove parent.
        }

        let status = TaskStatus::from_api_string(&task.status).expect("Invalid task status");
        let task_upadate = TaskRequest{
            project_id: task.project.id,
            title: title_buf,
            description: description_buf,
            issue: issue.to_value(),
            due_date: task.due_date,
            assigned_to_id: task.assigned_to.id,
            parent_id: task.parent,
            status: status.to_value()
        };

        let request = match get_request(TASK_ENDPOINT,Some(&task_id)) {
            Ok(c) => c,
            Err(err) => {
                eprint!("{}: {err}", CLIENT_ERROR.red());
                std::process::exit(1);
            }
        };
        let resp = match request.client.put(request.url).json(&task_upadate).send() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
                std::process::exit(1);
            }
        };
        if resp.status().is_success() {
            println!("{}", "Task Updated".green().bold())
        } else if resp.status().is_client_error() {
            let response: ClientErrorResponse = resp.json().unwrap();
            println!("{}: {:?}", "error".red(), response);
        } else {
            println!("{}: {}", "Error".red().bold(),resp.status());
            println!("{}: {}", "Error".red().bold(), resp.text().unwrap());
        };

        
    }else if resp.status().is_client_error(){
        eprintln!("{}: Unable to find task code", "Error".red());
    } else{
        eprintln!("{}: Unable to fetch task details:  {:?}", "Error".red().bold(), resp.status());
        eprintln!("{}: {:?}", "Error".red().bold(),resp.text().ok());
    }
}