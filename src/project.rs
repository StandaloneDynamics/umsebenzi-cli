use crate::config::read_toml_file;
use crate::service::{build_url, get_client};
use crate::description::text_editor;

use anyhow::Result;
use clap::{Parser, Subcommand};
use cli_table::{print_stdout, Table, WithTitle};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fmt,
    io::{self, Write},
};
use colored::Colorize;

const PROJECT_TITLE_ERROR: &str = "Project title expected";
const PROJECT_CODE_ERROR: &str = "Project code expected";
const PROJECT_DESCRIPTION_ERROR: &str = "Project description expected";
const CLIENT_ERROR: &str = "Unable to create request client";
const CLIENT_RESPONSE_ERROR: &str = "Response Error";



#[derive(Serialize, Deserialize, Debug, Table)]
struct User {
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
struct ProjectResponse {
    id: i32,
    created_by: User,
    title: String,
    #[table(skip)]
    description: String,
    code: String,
    created_at: String,
    #[table(skip)]
    modified_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientErrorResponse {
    title: Option<Vec<String>>,
    description: Option<Vec<String>>,
    code: Option<Vec<String>>,
    detail: Option<String>,
}

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

struct PrepClient {
    client: Client,
    url: String,
}

fn prepare_client(instance: Option<&String>) -> Result<PrepClient> {
    let config = match read_toml_file() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}: {err}", "Unable to read toml file".red());
            std::process::exit(1);
        }
    };
    let url = match build_url(&config, "/api/v1/projects/", instance) {
        Ok(u) => u,
        Err(err) => {
            eprintln!("{}: {err}", "Unable to build url".red());
            std::process::exit(1);
        }
    };
    let client = match get_client(&config) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    Ok(PrepClient {
        client: client,
        url: url,
    })
}

fn list() {
    let prep = match prepare_client(None) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };

    let resp = match prep.client.get(prep.url).send() {
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
        //println!("{:?}", proj);
        let _ = print_stdout(proj.with_title()).is_ok();
    } else {
        println!("{}", resp.status());
        println!("{}", resp.url());
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
    println!("{}: ","Description [type 'END' on a new line to finish]".green().bold());
    let description = text_editor(None).expect(&PROJECT_DESCRIPTION_ERROR.red().bold());
   
    let mut project_body = HashMap::new();
    project_body.insert("title", title_buf);
    project_body.insert("description", description);
    project_body.insert("code", code_buf);

    let prep = match prepare_client(None) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    let resp = match prep.client.post(prep.url).json(&project_body).send() {
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
        println!("{}: {}", "Error".red().bold(),resp.status());
        println!("{}: {}", "Error".red().bold(), resp.text().unwrap());
    };
}

fn edit(project_id: String) {
    // First get the exising project
    let proj: ProjectResponse;
    let prep = match prepare_client(Some(&project_id)) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    let resp = match prep.client.get(prep.url).send() {
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
                eprint!("Unable to parse response json: {err}");
                std::process::exit(1);
            }
        };
        // Update values
        print!("{}: ","Title [leave blank to use existing]".green().bold());
        let _ = io::stdout().flush();
        let mut title_buf = String::new();
        io::stdin()
            .read_line(&mut title_buf)
            .expect(&PROJECT_TITLE_ERROR.red());
        if title_buf.trim().is_empty() {
            title_buf = proj.title
        }

        print!("{}", "Code: [leave blank to use existing]".green().bold());
        let _ = io::stdout().flush();
        let mut code_buf = String::new();
        io::stdin()
            .read_line(&mut code_buf)
            .expect(&PROJECT_CODE_ERROR.red());
        if code_buf.trim().is_empty() {
            code_buf = proj.code
        }

        print!("Description: [Type E to edit. leave blank to use existing]: ");
        let _ = io::stdout().flush();
        let mut description_buf = String::new();
        io::stdin()
            .read_line(&mut description_buf)
            .expect("Project description expected");
        if description_buf.trim().is_empty() {
            description_buf = proj.description
        }else if description_buf.trim() == "E" {
            description_buf =  text_editor(Some(proj.description)).expect("Project description expected");
        }

        let mut project_body = HashMap::new();
        project_body.insert("title", title_buf);
        project_body.insert("description", description_buf);
        project_body.insert("code", code_buf);

        let prep = match prepare_client(Some(&project_id)) {
            Ok(c) => c,
            Err(err) => {
                eprint!("unable to create client {:?}", err);
                std::process::exit(1);
            }
        };
        let resp = match prep.client.put(prep.url).json(&project_body).send() {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Unable to send request {err}");
                std::process::exit(1);
            }
        };
        if resp.status().is_success() {
            println!("Project Updated")
        } else if resp.status().is_client_error() {
            let response: ClientErrorResponse = resp.json().unwrap();
            println!("{:?}", response);
        } else {
            println!("{}", resp.status());
            println!("{}", resp.text().unwrap());
        };

    } else if resp.status().is_client_error(){
        eprintln!("Unable to find project code");
    } else{
        eprintln!("Unable to fetch project details:  {:?}", resp.status());
        eprintln!("{:?}", resp.text().ok());
    }
}

fn delete(project_id: String) {
    let yes = "Y";
    let no = "N";
    print!("Are you sure you want to delete project with ID={project_id}: [Y/N] ");
    let _ = io::stdout().flush();
    let mut confirm_buf = String::new();
    io::stdin()
        .read_line(&mut confirm_buf)
        .expect("Expected Y or N");
    let input = confirm_buf.trim();
    if input.is_empty() {
        eprintln!("Expected Y or N");
        std::process::exit(1);
    } else if input != yes && input != no {
        eprintln!("Options are Y or N");
        std::process::exit(1);
    }
    if input.eq(yes) {
        let prep = match prepare_client(Some(&project_id)) {
            Ok(c) => c,
            Err(err) => {
                eprint!("unable to create client {:?}", err);
                std::process::exit(1);
            }
        };
        let resp = match prep.client.delete(prep.url).send() {
            Ok(r) => r,
            Err(err) => {
                eprintln!("Invalid Request {err}");
                std::process::exit(1);
            }
        };
        if resp.status().is_success() {
            println!("Project Deleted");
        } else if resp.status().is_client_error() {
            eprintln!("Unable to delete Project:  {:?}", resp);
            std::process::exit(1);
        }
    }
}

fn detail(project_id: String) {
    let prep = match prepare_client(Some(&project_id)) {
        Ok(c) => c,
        Err(err) => {
            eprint!("unable to create client {:?}", err);
            std::process::exit(1);
        }
    };
    let resp = match prep.client.get(prep.url).send() {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Invalid Request {err}");
            std::process::exit(1);
        }
    };
    if resp.status().is_success() {
        let proj: ProjectResponse = match resp.json() {
            Ok(r) => r,
            Err(err) => {
                eprint!("Unable to parse response json: {err}");
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
    } else if resp.status().is_client_error(){
        eprintln!("Unable to find project code");
    } else{
        eprintln!("Unable to fetch project details:  {:?}", resp.status());
        eprintln!("{:?}", resp.text().ok());
    }
}