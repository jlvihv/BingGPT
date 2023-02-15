use std::fs;

use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

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

/// 获取 header
fn get_header() -> Result<HeaderMap> {
    let bing_cookies = get_config()?;
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
    headers.insert(
        "Cookie",
        HeaderValue::from_str(&format!(
            "_U={}; KievRPSSecAuth={}",
            bing_cookies.u, bing_cookies.kiev
        ))?,
    );
    Ok(headers)
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BingCookies {
    pub u: String,
    pub kiev: String,
}

/// Get config from ~/.config/bing-cookies.toml
fn get_config() -> Result<BingCookies> {
    let config_file = format!("{}/.config/bing-cookies.toml", env!("HOME"));
    let config = fs::read_to_string(config_file)?;
    let config: BingCookies = toml::from_str(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_config() {
        let config = get_config().unwrap();
        println!("{:#?}", config)
    }
}
