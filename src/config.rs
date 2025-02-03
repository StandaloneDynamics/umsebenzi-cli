use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use toml;
use url;

use crate::defaults::show_status_options;

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub host: String,
    pub credentials: String,
}

#[derive(Subcommand, Debug)]
enum Config {
    Add,
    TaskStatus
}

#[derive(Parser, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    command: Option<Config>,
}

pub fn run(args: ConfigArgs) {
    if let Some(v) = args.command {
        match v {
            Config::Add => add(),
            Config::TaskStatus => show_status_options(true),
        }
    } else {
        show();
    }
}

fn add() {
    let mut host = "http://localhost:8000/api/v1";

    print!("Host [http://localhost:8000/api/v1]: ");
    let _ = io::stdout().flush();
    let mut host_buf = String::new();
    io::stdin()
        .read_line(&mut host_buf)
        .expect("host url expected");
    if !host_buf.trim().is_empty() {
        match url::Url::parse(host_buf.trim()) {
            Ok(_) => host = host_buf.trim(),
            Err(_) => {
                eprintln!("Invalid url");
                std::process::exit(1)
            }
        }
    }

    print!("Token: ");
    let _ = io::stdout().flush();
    let mut cred_buf = String::new();
    io::stdin()
        .read_line(&mut cred_buf)
        .expect("token expected");
    if cred_buf.trim().is_empty() {
        eprintln!("Token expected");
        std::process::exit(1);
    }
    let token = cred_buf.trim();

    let data = Data {
        host: host.to_string(),
        credentials: token.to_string(),
    };
    match write_toml_file(&data) {
        Ok(_) => println!("Config file created"),
        Err(err) => {
            eprintln!("Can't write file: {}", err);
            std::process::exit(1)
        }
    }
}

fn show() {
    let data = read_toml_file();
    match data {
        Ok(d) => {
            println!("Host: {}", d.host);
            println!("Token: {}", d.credentials);
        }
        Err(_) => {
            eprintln!("Unable to read toml file");
            std::process::exit(1);
        }
    }
}

pub fn read_toml_file() -> Result<Data> {
    let toml_str = fs::read_to_string("umsebenzi.toml")?;
    let file: Data = toml::from_str(&toml_str)?;
    Ok(file)
}
fn write_toml_file(data: &Data) -> Result<()> {
    let toml_string = toml::to_string(data)?;
    let mut file = File::create("umsebenzi.toml")?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
