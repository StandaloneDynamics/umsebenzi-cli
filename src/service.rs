use crate::config::Data;
use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::header;

pub fn get_headers(token: &str) -> Result<header::HeaderMap> {
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

pub fn build_url(conf: &Data, endpoint: &str, instance: Option<&String>) -> Result<String> {
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

pub fn get_client(config: &Data) -> Result<Client> {
    let headers = get_headers(&config.credentials)?;
    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}
