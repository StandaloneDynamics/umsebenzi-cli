use clap::{Parser, Subcommand};


use crate::service::{delete_confirmation, get_request, RequestType, CLIENT_ERROR, CLIENT_RESPONSE_ERROR};
use crate::response::TaskResponse;
use colored::Colorize;
use cli_table::{print_stdout, WithTitle};


const TASK_ENDPOINT: &str = "/tasks/";

#[derive(Subcommand, Debug)]
enum TaskCLI{
    List,
    Add,
    Detail { task_id: String },
    Edit { task_id: String },
    Delete { task_id: String },
}

#[derive(Parser, Debug)]
pub struct TaskArgs{
    #[command(subcommand)]
    command: TaskCLI,
}

pub fn run(args: TaskArgs){
    match args.command {
        TaskCLI::Add => println!("Add new list"),
        TaskCLI::List => list(),
        TaskCLI::Edit { task_id } => println!("Edit task"),
        TaskCLI::Detail { task_id } => detail(task_id),
        TaskCLI::Delete { task_id } => delete(task_id),
        
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
                eprint!("{}: {err}", "Unable to parse response json".red());
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
        println!("{}: {}", "Due Date".green().bold(), task.due_date);
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
