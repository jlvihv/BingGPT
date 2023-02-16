use anyhow::Result;
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest::Client,
    cookie_path: String,
}

impl Client {
    pub fn new(cookie_path: &str) -> Self {
        Self {
            client: reqwest::ClientBuilder::new()
                .timeout(tokio::time::Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            cookie_path: cookie_path.to_string(),
        }
    }

    pub async fn get_html(&self, url: &str) -> Result<String> {
        Ok(self
            .client
            .get(url)
            .headers(get_header(&self.cookie_path)?)
            .send()
            .await?
            .text()
            .await?)
    }
}

fn get_header(cookie_path: &str) -> Result<HeaderMap> {
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
    headers.insert("x-forwarded-for", HeaderValue::from_str("8.8.8.8")?);
    headers.insert("Cookie", HeaderValue::from_str(&get_config(cookie_path)?)?);
    Ok(headers)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

fn get_config(cookie_path: &str) -> Result<String> {
    let config_file = if cookie_path.starts_with('~') {
        format!("{}{}", env!("HOME"), cookie_path.trim_start_matches('~'))
    } else {
        cookie_path.to_string()
    };

    let json_str = if let Ok(s) = fs::read_to_string(&config_file) {
        s
    } else {
        println!(
            "{}{}",
            "Config file not found, please create ".red(),
            config_file.red().bold()
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
