use super::{conversation::Conversation, msg::fill_msg};
use anyhow::{bail, Ok, Result};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite, MaybeTlsStream, WebSocketStream};

pub struct ChatHub {
    conversation: Conversation,
    read: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, tungstenite::Message>>,
    write: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    msg_cache: String,
    cookie_path: String,
}

impl ChatHub {
    pub async fn new(cookie_path: &str) -> Result<Self> {
        let conversation = Conversation::new(cookie_path).await?;
        let mut chat_hub = Self {
            conversation,
            read: None,
            write: None,
            msg_cache: String::new(),
            cookie_path: cookie_path.to_string(),
        };
        chat_hub.create_websocket().await?;
        chat_hub.send_protocol().await?;
        Ok(chat_hub)
    }

    async fn create_websocket(&mut self) -> Result<()> {
        let url = "wss://sydney.bing.com/sydney/ChatHub";
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (read, write) = ws_stream.split();
        self.read = Some(read);
        self.write = Some(write);
        Ok(())
    }

    async fn send_protocol(&mut self) -> Result<()> {
        let write = self.read.as_mut().unwrap();
        let read = self.write.as_mut().unwrap();
        write
            .send(tungstenite::Message::Text(
                r#"{"protocol": "json", "version": 1}"#.to_string() + "",
            ))
            .await?;
        read.next().await.unwrap()?;
        Ok(())
    }

    pub async fn reset(&mut self) -> Result<()> {
        self.close().await?;
        self.conversation = Conversation::new(&self.cookie_path).await?;
        self.create_websocket().await?;
        self.send_protocol().await?;
        self.msg_cache = String::new();
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        if self.read.is_some() {
            self.read.as_mut().unwrap().close().await?;
        }
        Ok(())
    }

    pub async fn send_msg(&mut self, msg: &str) -> Result<()> {
        let write = match self.read.as_mut() {
            Some(write) => write,
            None => {
                bail!("Connection aborted, send message failed");
            }
        };
        write
            .send(tungstenite::Message::Text(fill_msg(
                msg,
                &self.conversation,
            )?))
            .await?;
        self.conversation.invocation_id += 1;
        Ok(())
    }

    pub async fn recv_raw_json(&mut self) -> Result<String> {
        let read = match self.write.as_mut() {
            Some(read) => read,
            None => {
                bail!("Connection aborted, receive message failed");
            }
        };
        self.msg_cache = read.next().await.unwrap()?.to_string();
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
}
