use super::{conversation::Conversation, msg::fill_msg};
use anyhow::{bail, Ok, Result};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};

pub const SPLIT_CHAR: char = '';

pub struct Bing {
    conversation: Conversation,
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    msg_cache: String,
    cookie_path: String,
}

impl Bing {
    pub async fn new(cookie_path: &str) -> Result<Self> {
        let conversation = Conversation::new(cookie_path).await?;
        let mut bing = Self {
            conversation,
            ws_stream: Self::create_websocket().await?,
            msg_cache: String::new(),
            cookie_path: cookie_path.to_string(),
        };
        bing.send_protocol().await?;
        Ok(bing)
    }

    async fn create_websocket() -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let url = "wss://sydney.bing.com/sydney/ChatHub";
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        Ok(ws_stream)
    }

    async fn send_protocol(&mut self) -> Result<()> {
        self.ws_stream
            .send(tungstenite::Message::Text(
                r#"{"protocol": "json", "version": 1}"#.to_string()
                    + SPLIT_CHAR.to_string().as_str(),
            ))
            .await?;
        let Some(msg) = self.ws_stream.next().await else {
            bail!("websocket closed");
        };
        msg?;
        Ok(())
    }

    pub async fn reset(&mut self) -> Result<()> {
        self.close().await?;
        self.conversation = Conversation::new(&self.cookie_path).await?;
        self.ws_stream = Self::create_websocket().await?;
        self.send_protocol().await?;
        self.msg_cache = String::new();
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        self.ws_stream.close(None).await?;
        Ok(())
    }

    pub async fn send_msg(&mut self, msg: &str) -> Result<()> {
        self.ws_stream
            .send(tungstenite::Message::Text(fill_msg(
                msg,
                &self.conversation,
            )?))
            .await?;
        self.conversation.invocation_id += 1;
        Ok(())
    }

    pub async fn recv_raw_json(&mut self) -> Result<String> {
        let Some(msg) =  self.ws_stream.next().await else{
            bail!("websocket closed");
        };
        self.msg_cache = msg?.to_string();
        Ok(self.msg_cache.clone())
    }

    pub async fn recv_text(&mut self) -> Result<Option<String>> {
        let msg = self.recv_raw_json().await?;
        if gjson::get(&msg, "type").i32() == 1 {
            return Ok(Some(
                gjson::get(&msg, "arguments.0.messages.0.adaptiveCards.0.body.0.text").to_string(),
            ));
        }
        Ok(None)
    }

    pub async fn recv_text_only(&mut self) -> Result<Option<String>> {
        let msg = self.recv_raw_json().await?;
        if gjson::get(&msg, "type").i32() == 1 {
            return Ok(Some(
                gjson::get(&msg, "arguments.0.messages.0.text").to_string(),
            ));
        }
        Ok(None)
    }

    pub fn recv_suggesteds(&mut self) -> Result<Option<Vec<String>>> {
        let msg = self.msg_cache.clone();
        if gjson::get(&msg, "type").i32() == 2 {
            if gjson::get(&msg, "item.result.value").str() == "Throttled" {
                let msg = gjson::get(&msg, "item.result.message").to_string();
                bail!(format!("Error: Throttled: {msg}"))
            }
            let suggesteds = gjson::get(&msg, "item.messages.1.suggestedResponses.#.text");
            self.msg_cache = String::new();
            return Ok(Some(
                suggesteds.array().iter().map(|s| s.to_string()).collect(),
            ));
        }
        Ok(None)
    }

    pub fn is_done(&mut self) -> bool {
        let msg = self.msg_cache.clone();
        if gjson::get(&msg, "type").i32() == 2 {
            return true;
        }
        false
    }
}
