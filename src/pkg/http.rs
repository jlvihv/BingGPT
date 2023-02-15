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
    let u = r#"1PhVHyrZJR4NWTH6vvKgQzXhMdmtAbKUNnb5FWc24sJcMmrMUpDXrghc41mE40yqTtBdJGM-F98yG_cW5PdbNGE5i8e8vRmiiaCwFz-0lkDWCyxF8wUrqW3gF9XmcU0n_7Yf1aD5k5lz_4lLEOqeDFInvx6ZfMmdJDzTVqrUUJJAW85xMRiEU6cZCW1ic1QyrkkyuMvcVFzAlgWPe3ZKGeQ"#;
    let kiev = r#"FAA6BBRaTOJILtFsMkpLVWSG6AN6C/svRwNmAAAEgAAACG/8Rp29Ow1I+ANTuq04cAnyMCaVwOhjXbhdAto3oH/j0Ve6cMVFvv84OiRLA5I3ik09d6cMqPHUdu9ogkQwyegx1A7tOm1ZjEIprPQswgfVBKWmjENp+hWA0viqvTNQt9VgG1X/T19Hk2HOyoaZLVN5C/hdjBcCPoPKYffiOZ/9kRGFujdyGcuQN8mJ603fUeaNFOePHDlGbAP56zS/JQ1JuN6LotOVfCEH7G1dOYOfZhA+8yY07xN1GnWgZYFuq1EtZ3qN1dMYr+9aNoSKBkK1dL58q6XVuWMP1jDAJWpGz/Ri57u3afHG6Q1v8EUoci7FCS24E/6k/Tu+IZsG3Y219GY8xuq3egpwWOzfF0Giq+6wd5jMFlp7I1oTcgWuwjBr1ILLBuZM80D1d3qe796muw22DnogUWSbApjCKQ/sXH1nAjg90cI5H4moXN2ziYeQ20Eb39ow3UVevEKTCq4WlRE5W12WsQZIcxQueddtLssFlCwySvbKeLQavS3VKHjuUE8Lw5cWdml2cCJfbx7vtUwwDbAtFY5A9l3GlUzDr2+0EztN9OBI+cH1t6gfKXowoybbtU1tdVx+f4sLUd1HGzTWbv42EBljXUGTQbiNZI8jh0vfJgDrd5NoJkuaf6DZC8i7gQnwArfngvUmVmmytKeQMBahPAKWyRTdOp5KvzEn2lpkwiRYbXavWQkDUlmTzwesoTQWzaXAHtOEBrLbC3MEhXW2B6dchpVKV5MlhLeUXE34TZM61fHkGGbOkJJzfu9pg+uSua2akQ32qnSV9osviteDCq2XPqVjyHaRZ4mpY3CdxlY5empAZehP9MrMjaNg07mWFljtujquGyXqVac2osnSIGQ7kq0rD74hFVXmoJ393pKoBf/D85A0hqPQaOTtP5dGSSRYXSD1lGIei32P63XBiM711mQUO3GYDFZftt/jS0BkaIU6LQA5UQieAFBaKgwPumq7b/Vkw34KxdtxqkgYrcv4WedcU3fx1mGxSE8L9T7L6FmqBDyRsYuG83q2B90xQTfo6bwh66DSoz2+EkEWtubozJrjlxepCS4BWzC9KW9SNmOcPttXemrWVDZTpIZPldS0gai9kogDG7UQlgLtiy6vi03YRyl04D76rFclLn5ML9cPAK2ENbby4QS6DldFF6M3qZ5w2rdiA37AbcyvAIXJQKGo5x7/LD2Bp/lGw9TjC4gQ1PxSmH7bF30KSUcEYNGiE2Rawv0/n/JxWDZhz9/nXF66jXB/dhjwWHta5BewPRYdXv606jvHCRcUBdviqA8GThPvRuR7k1Q7vXcIylt7St99WYMONhcN8Y/Jj/cht/W2AU0HO+KAMG1PJzMt7rQ3V05xV7VPdxQAo/q5iFDlAiBBeOWeysDQX4A6FDU="#;
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
        HeaderValue::from_str(&format!("_U={}; KievRPSSecAuth={}", u, kiev))?,
    );
    Ok(headers)
}
