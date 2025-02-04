use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::{
    fs::{self, File},
    path,
};
use toml;
use url;

use crate::defaults::show_status_options;

const CONFIG_DIR: &str = "XDG_CONFIG_HOME";

#[derive(Deserialize, Serialize)]
pub struct Data {
    pub host: String,
    pub credentials: String,
}

#[derive(Subcommand, Debug)]
enum Config {
    Add,
    TaskStatus,
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
        Err(err) => {
            eprintln!("Unable to read toml file: {err}");
            std::process::exit(1);
        }
    }
}

fn config_file_path() -> Result<path::PathBuf> {
    let config_dir = std::env::var(CONFIG_DIR)?;
    let path = path::Path::new(&config_dir).join("umsebenzi");
    if path.is_dir() {
        Ok(path)
    } else {
        let _ = fs::create_dir(&path);
        Ok(path)
    }
}
pub fn read_toml_file() -> Result<Data> {
    let directory = match config_file_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Can't create config directory: {err}");
            std::process::exit(1)
        }
    };
    let file_path = path::Path::new(&directory).join("umsebenzi.toml");
    if file_path.is_file() {
        let toml_str = fs::read_to_string(file_path)?;
        let file: Data = toml::from_str(&toml_str)?;
        return Ok(file);
    }
    Err(anyhow!("umsebenzi.toml file not found"))
}
fn write_toml_file(data: &Data) -> Result<()> {
    let directory = match config_file_path() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("Can't create config directory: {err}");
            std::process::exit(1)
        }
    };
    let file_path = path::Path::new(&directory).join("umsebenzi.toml");

    let toml_string = toml::to_string(data)?;
    let mut file = File::create(&file_path)?;
    file.write_all(toml_string.as_bytes())?;
    Ok(())
}
