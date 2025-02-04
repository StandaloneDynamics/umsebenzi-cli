use crate::config::read_toml_file;
use crate::config::Data;
use anyhow::Result;
use colored::Colorize;
use reqwest::blocking::Client;
use reqwest::header;
use std::io::{self, Write};

pub const CLIENT_ERROR: &str = "Unable to create request client";
pub const CLIENT_RESPONSE_ERROR: &str = "Response Error";
pub const PROJECT_DELETE_CONFIRMATION: &str = "Are you sure you want to delete project with ID=";
pub const TASK_DELETE_CONFIRMATION: &str =
    "Are you sure you want to delete task, including subtasks with code=";

pub enum RequestType {
    PROJECT,
    TASK,
}

impl RequestType {
    fn value(&self) -> &str {
        match *self {
            Self::PROJECT => PROJECT_DELETE_CONFIRMATION,
            Self::TASK => TASK_DELETE_CONFIRMATION,
        }
    }
}

pub struct RequestClient {
    pub client: Client,
    pub url: String,
}

fn get_headers(token: &str) -> Result<header::HeaderMap> {
    let mut auth = String::from("Token ");
    auth.push_str(token);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&auth)?);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );
    Ok(headers)
}

fn build_url(conf: &Data, endpoint: &str, instance: Option<&String>) -> Result<String> {
    let mut api_url = String::new();
    api_url.push_str(&conf.host);
    api_url.push_str(endpoint);
    match instance {
        Some(id) => {
            api_url.push_str(&id);
            api_url.push_str("/");
        }
        _ => {}
    }
    Ok(api_url)
}

fn get_client_builder(config: &Data) -> Result<Client> {
    let headers = get_headers(&config.credentials)?;
    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

pub fn prepare_client(endpoint: &str, instance: Option<&String>) -> Result<RequestClient> {
    let config = match read_toml_file() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}: {err}", "Unable to read toml file".red());
            std::process::exit(1);
        }
    };
    let url = match build_url(&config, endpoint, instance) {
        Ok(u) => u,
        Err(err) => {
            eprintln!("{}: {err}", "Unable to build url".red());
            std::process::exit(1);
        }
    };
    let client = match get_client_builder(&config) {
        Ok(c) => c,
        Err(err) => {
            eprint!("{}: {err}", CLIENT_ERROR.red());
            std::process::exit(1);
        }
    };
    Ok(RequestClient {
        client: client,
        url: url,
    })
}

pub fn get_request(endpoint: &str, instance: Option<&String>) -> Result<RequestClient> {
    let prep = prepare_client(endpoint, instance)?;
    Ok(prep)
}

pub fn delete_confirmation(item_id: &String, request: RequestType) -> bool {
    let yes = "Y";
    let no = "N";
    print!("{} {} [Y/N]: ", request.value().red().bold(), item_id);
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
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_client() {
        let item_id = "1".to_string();
        let request = get_request("/endpoint/", Some(&item_id));
        assert!(request.is_ok());
        assert_eq!(
            request.is_ok_and(|r| r.url == "http://localhost:8000/api/v1/endpoint/1/"),
            true
        );
    }
}
