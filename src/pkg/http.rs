use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};

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
    headers.insert("Cookie",HeaderValue::from_str("_U=1L4rfeU0UdJbXsieeB8lKemROH4O_i8iqVJZBv9j0t9jrHBmvxOFsGVOTpXAB6KglADDSDlWPxtu8NDZdH5szy0GNtMfxEP0AmDvZ7fbqCxcZjKs1kwSirN3-RkvzPzwFdjtEsitWDKqrY5Pg6bjhQPiUEe7ouDDU1s3s8iZPbhZMWJ6GQEHWVuw9VElC4aIzYnc9QdEPhmii3yhs2lzN-A")?);
    Ok(headers)
}
