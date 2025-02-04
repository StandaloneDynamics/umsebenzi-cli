use crate::description::text_editor;
use crate::response::{ClientErrorResponse, ProjectResponse};
use crate::service::{
    delete_confirmation, get_request, RequestType, CLIENT_ERROR, CLIENT_RESPONSE_ERROR,
};

use clap::{Parser, Subcommand};
use cli_table::{print_stdout, WithTitle};
use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};

const PROJECT_ENDPOINT: &str = "/projects/";
const PROJECT_TITLE_ERROR: &str = "Project title expected";
const PROJECT_CODE_ERROR: &str = "Project code expected";
const PROJECT_DESCRIPTION_ERROR: &str = "Project description expected";

#[derive(Subcommand, Debug)]
enum ProjectCLI {
    List,
    Add,
    Detail { project_id: String },
    Edit { project_id: String },
    Delete { project_id: String },
}

#[derive(Parser, Debug)]
pub struct ProjectArgs {
    #[command(subcommand)]
    command: ProjectCLI,
}

pub fn run(args: ProjectArgs) {
    match args.command {
        ProjectCLI::List => list(),
        ProjectCLI::Add => add(),
        ProjectCLI::Delete { project_id } => delete(project_id),
        ProjectCLI::Detail { project_id } => detail(project_id),
        ProjectCLI::Edit { project_id } => edit(project_id),
    }
}

fn list() {
    let request = match get_request(PROJECT_ENDPOINT, None) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("{}: {err}", CLIENT_ERROR.red().bold());
            std::process::exit(1);
        }
    };

    let resp = match request.client.get(request.url).send() {
        Ok(r) => r,
        Err(err) => {
            eprintln!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
            std::process::exit(1);
        }
    };
    if resp.status().is_success() {
        let proj: Vec<ProjectResponse> = match resp.json() {
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
    //
}

fn add() {
    println!("{}", "Create a new project".green().bold());

    print!("{}: ", "Title".green().bold());
    let _ = io::stdout().flush();
    let mut title_buf = String::new();
    io::stdin()
        .read_line(&mut title_buf)
        .expect(&PROJECT_TITLE_ERROR.red().bold());
    if title_buf.trim().is_empty() {
        eprintln!("{}", PROJECT_TITLE_ERROR.red().bold());
        std::process::exit(1);
    }
    print!("{}: ", "Code".green().bold());
    let _ = io::stdout().flush();
    let mut code_buf = String::new();
    io::stdin()
        .read_line(&mut code_buf)
        .expect(&PROJECT_CODE_ERROR.red());
    if code_buf.trim().is_empty() {
        eprintln!("{}", PROJECT_CODE_ERROR.red().bold());
        std::process::exit(1);
    }
    print!(
        "{}: ",
        "Description [type 'Y' to open editor]".green().bold()
    );
    let _ = io::stdout().flush();
    let mut answer_buf = String::new();
    io::stdin()
        .read_line(&mut answer_buf)
        .expect(&PROJECT_DESCRIPTION_ERROR.red());
    let mut description = String::new();
    if answer_buf.trim() == "Y" {
        description = text_editor(None).expect(&PROJECT_DESCRIPTION_ERROR.red().bold());
    } else {
        eprintln!("{}", "Invalid Command expected Y".red().bold());
        std::process::exit(1);
    }

    let mut project_body = HashMap::new();
    project_body.insert("title", title_buf);
    project_body.insert("description", description);
    project_body.insert("code", code_buf);

    let request = match get_request(PROJECT_ENDPOINT, None) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    let resp = match request.client.post(request.url).json(&project_body).send() {
        Ok(r) => r,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
            std::process::exit(1);
        }
    };
    if resp.status().is_success() {
        println!("{}", "Project created".green().bold())
    } else if resp.status().is_client_error() {
        let response: ClientErrorResponse = resp.json().unwrap();
        println!("{}: {:?}", "error".red(), response);
    } else {
        println!("{}: {}", "Error".red().bold(), resp.status());
        println!("{}: {}", "Error".red().bold(), resp.text().unwrap());
    };
}

fn edit(project_id: String) {
    // First get the exising project
    let proj: ProjectResponse;
    let request = match get_request(PROJECT_ENDPOINT, Some(&project_id)) {
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
    // Update existing project
    if resp.status().is_success() {
        proj = match resp.json() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{} {err}", "Unable to parse response json".red());
                std::process::exit(1);
            }
        };
        // Update values
        print!("{}: ", "Title [leave blank to use existing]".green().bold());
        let _ = io::stdout().flush();
        let mut title_buf = String::new();
        io::stdin()
            .read_line(&mut title_buf)
            .expect(&PROJECT_TITLE_ERROR.red());
        if title_buf.trim().is_empty() {
            title_buf = proj.title
        }

        print!("{}: ", "Code: [leave blank to use existing]".green().bold());
        let _ = io::stdout().flush();
        let mut code_buf = String::new();
        io::stdin()
            .read_line(&mut code_buf)
            .expect(&PROJECT_CODE_ERROR.red());
        if code_buf.trim().is_empty() {
            code_buf = proj.code
        }

        print!(
            "{}: ",
            "Description: [Type E to edit. leave blank to use existing]"
                .green()
                .bold()
        );
        let _ = io::stdout().flush();
        let mut description_buf = String::new();
        io::stdin()
            .read_line(&mut description_buf)
            .expect(&PROJECT_DESCRIPTION_ERROR.red());
        if description_buf.trim().is_empty() {
            description_buf = proj.description
        } else if description_buf.trim() == "E" {
            description_buf =
                text_editor(Some(proj.description)).expect(&PROJECT_DESCRIPTION_ERROR.red());
        }

        let mut project_body = HashMap::new();
        project_body.insert("title", title_buf);
        project_body.insert("description", description_buf);
        project_body.insert("code", code_buf);

        let request = match get_request(PROJECT_ENDPOINT, Some(&project_id)) {
            Ok(c) => c,
            Err(err) => {
                eprint!("{}: {err}", CLIENT_ERROR.red());
                std::process::exit(1);
            }
        };
        let resp = match request.client.put(request.url).json(&project_body).send() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{}: {err}", CLIENT_RESPONSE_ERROR.red());
                std::process::exit(1);
            }
        };
        if resp.status().is_success() {
            println!("{}", "Project Updated".green().bold())
        } else if resp.status().is_client_error() {
            let response: ClientErrorResponse = resp.json().unwrap();
            println!("{}: {:?}", "error".red(), response);
        } else {
            println!("{}: {}", "Error".red().bold(), resp.status());
            println!("{}: {}", "Error".red().bold(), resp.text().unwrap());
        };
    } else if resp.status().is_client_error() {
        eprintln!("{}: Unable to find project code", "Error".red());
    } else {
        eprintln!(
            "{}: Unable to fetch project details:  {:?}",
            "Error".red().bold(),
            resp.status()
        );
        eprintln!("{}: {:?}", "Error".red().bold(), resp.text().ok());
    }
}

fn delete(project_id: String) {
    let is_delete = delete_confirmation(&project_id, RequestType::PROJECT);
    if is_delete {
        let request = match get_request(PROJECT_ENDPOINT, Some(&project_id)) {
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
            println!("{}", "Project Deleted".green().bold());
        } else if resp.status().is_client_error() {
            eprintln!("{}: Unable to delete Project:  {:?}", "Error".red(), resp);
            std::process::exit(1);
        }
    }
}

fn detail(project_id: String) {
    let request = match get_request(PROJECT_ENDPOINT, Some(&project_id)) {
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
    if resp.status().is_success() {
        let proj: ProjectResponse = match resp.json() {
            Ok(r) => r,
            Err(err) => {
                eprint!("{}: {err}", "Unable to parse response json".red().bold());
                std::process::exit(1);
            }
        };
        println!("ID: {}", proj.id);
        println!("Title: {}", proj.title);
        println!("Code: {}", proj.code);
        println!("Created By: {}", proj.created_by);
        println!("Created At: {}", proj.created_at);
        println!("Modified At: {}", proj.modified_at);
        println!("Description:");
        println!("{}", proj.description)
    } else if resp.status().is_client_error() {
        eprintln!(
            "{}: {} Unable to find project code",
            "Error".red(),
            resp.status()
        );
    } else {
        eprintln!("Unable to fetch project details:  {:?}", resp.status());
        eprintln!("{:?}", resp.text().ok());
    }
}
