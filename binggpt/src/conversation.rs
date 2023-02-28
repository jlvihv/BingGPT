use anyhow::Result;

use super::http;

#[derive(Debug, Clone)]
pub struct Conversation {
    pub client_id: String,
    pub conversation_id: String,
    pub conversation_signature: String,
    pub invocation_id: i32,
}

impl Conversation {
    pub async fn new(cookie_path: &str) -> Result<Self> {
        let json_str = http::Client::new(cookie_path)
            .get_html("https://edgeservices.bing.com/edgesvc/turing/conversation/create")
            .await?;
        if gjson::get(&json_str, "result.value").to_string() == "Success" {
            Ok(Conversation {
                client_id: gjson::get(&json_str, "clientId").to_string(),
                conversation_id: gjson::get(&json_str, "conversationId").to_string(),
                conversation_signature: gjson::get(&json_str, "conversationSignature").to_string(),
                invocation_id: 0,
            })
        } else {
            Err(anyhow::anyhow!(
                gjson::get(&json_str, "result.message").to_string()
            ))
        }
    }
}
