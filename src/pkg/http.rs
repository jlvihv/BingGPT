use anyhow::Result;
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
                .timeout(tokio::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn get_html(&self, url: &str) -> Result<String> {
        Ok(self
            .client
            .get(url)
            .headers(get_header()?)
            .send()
            .await?
            .text()
            .await?)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn get_header() -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert("user-agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.41")?);
    headers.insert("origin", HeaderValue::from_str("https://www.bing.com")?);
    headers.insert("referer", HeaderValue::from_str("https://www.bing.com/")?);
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_str(
            r#""Chromium";v="110", "Not A(Brand";v="24", "Microsoft Edge";v="110""#,
        )?,
    );
    headers.insert("sec-ch-ua-platform", HeaderValue::from_str("Windows")?);
    headers.insert("Cookie", HeaderValue::from_str(&get_config()?)?);
    Ok(headers)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

/// Get config from ~/.config/bing-cookies.json
fn get_config() -> Result<String> {
    let config_file = format!("{}/.config/bing-cookies.json", env!("HOME"));
    let json_str = if let Ok(s) = fs::read_to_string(config_file) {
        s
    } else {
        println!(
            "{}",
            "Config file not found, please create ~/.config/bing-cookies.json".red()
        );
        std::process::exit(1);
    };
    let cookies: Vec<Cookie> = serde_json::from_str(&json_str)?;
    let cookies = cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<String>>()
        .join("; ");
    Ok(cookies)
}
