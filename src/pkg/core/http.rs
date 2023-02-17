use anyhow::{bail, Result};
use rand::Rng;
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
    headers.insert("accept", HeaderValue::from_str("application/json")?);
    headers.insert("accept-language", HeaderValue::from_str("en-US,en;q=0.9")?);
    headers.insert("content-type", HeaderValue::from_str("application/json")?);
    headers.insert(
        "sec-ch-ua",
        HeaderValue::from_str(
            r#""Not_A Brand";v="99", "Microsoft Edge";v="109", "Chromium";v="109""#,
        )?,
    );
    headers.insert("sec-ch-ua-arch", HeaderValue::from_str(r#""x86""#)?);
    headers.insert("sec-ch-ua-bitness", HeaderValue::from_str(r#""64""#)?);
    headers.insert(
        "sec-ch-ua-full-version",
        HeaderValue::from_str(r#""109.0.1518.78""#)?,
    );
    headers.insert("sec-ch-ua-full-version-list", HeaderValue::from_str(r#""Not_A Brand";v="99.0.0.0", "Microsoft Edge";v="109.0.1518.78", "Chromium";v="109.0.5414.120""#)?);
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_str("?0")?);
    headers.insert("sec-ch-ua-model", HeaderValue::from_str("")?);
    headers.insert("sec-ch-ua-platform", HeaderValue::from_str(r#""Windows""#)?);
    headers.insert(
        "sec-ch-ua-platform-version",
        HeaderValue::from_str(r#""15.0.0""#)?,
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_str("empty")?);
    headers.insert("sec-fetch-mode", HeaderValue::from_str("cors")?);
    headers.insert("sec-fetch-site", HeaderValue::from_str("same-origin")?);
    headers.insert(
        "x-ms-client-request-id",
        HeaderValue::from_str(&uuid::Uuid::new_v4().to_string())?,
    );
    headers.insert(
        "x-ms-useragent",
        HeaderValue::from_str(
            "azsdk-js-api-client-factory/1.0.0-beta.1 core-rest-pipeline/1.10.0 OS/Win32",
        )?,
    );
    headers.insert(
        "Referer",
        HeaderValue::from_str("https://www.bing.com/search?q=Bing+AI&showconv=1&FORM=hpcodx")?,
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_str("origin-when-cross-origin")?,
    );
    headers.insert(
        "x-forwarded-for",
        HeaderValue::from_str(&get_random_forwarded_ip())?,
    );
    headers.insert("Cookie", HeaderValue::from_str(&get_cookie(cookie_path)?)?);
    Ok(headers)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

fn get_cookie(cookie_path: &str) -> Result<String> {
    let cookie_path = if cookie_path.starts_with('~') {
        let home = std::env::var("HOME")
            .unwrap_or_else(|_| std::env::var("USERPROFILE").unwrap_or_default());
        if home.is_empty() {
            bail!("Cannot find user home directory, please specify absolute path")
        }
        format!("{}{}", home, cookie_path.trim_start_matches('~'))
    } else {
        cookie_path.to_string()
    };

    let json_str = if let Ok(s) = fs::read_to_string(&cookie_path) {
        s
    } else {
        bail!("Config file not found, please create {}", cookie_path)
    };

    let cookies: Vec<Cookie> = serde_json::from_str(&json_str)?;
    let cookies = cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<String>>()
        .join("; ");
    Ok(cookies)
}

fn get_random_forwarded_ip() -> String {
    let mut rng = rand::thread_rng();
    format!(
        "13.{}.{}.{}",
        rng.gen_range(104..107),
        rng.gen_range(1..255),
        rng.gen_range(1..255)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_forwarded_ip() {
        for _ in 0..10000 {
            let ip = get_random_forwarded_ip();
            let fields = ip.split('.').collect::<Vec<&str>>();
            assert_eq!(fields.len(), 4);
            assert_eq!(fields[0], "13");
            assert!(
                fields[1].parse::<u8>().unwrap() >= 104 && fields[1].parse::<u8>().unwrap() < 107
            );
            assert!(
                fields[2].parse::<u8>().unwrap() >= 1 && fields[2].parse::<u8>().unwrap() < 255
            );
            assert!(
                fields[3].parse::<u8>().unwrap() >= 1 && fields[3].parse::<u8>().unwrap() < 255
            );
        }
    }
}
