use super::{conversation::Conversation, msg::fill_msg};
use anyhow::{Ok, Result};
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
}

impl ChatHub {
    pub async fn new(cookie_path: &str) -> Result<Self> {
        let conversation = Conversation::new(cookie_path).await?;
        let mut chat_hub = Self {
            conversation,
            read: None,
            write: None,
        };
        chat_hub.create_websocket().await?;
        chat_hub.send_protocol().await?;
        Ok(chat_hub)
    }

    async fn create_websocket(&mut self) -> Result<()> {
        let url = "wss://sydney.bing.com/sydney/ChatHub";
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        let (write, read) = ws_stream.split();
        self.write = Some(read);
        self.read = Some(write);
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

    pub async fn send_msg(&mut self, msg: &str) -> Result<()> {
        let write = self.read.as_mut().unwrap();
        write
            .send(tungstenite::Message::Text(fill_msg(
                msg,
                &self.conversation,
            )?))
            .await?;
        self.conversation.invocation_id += 1;
        Ok(())
    }

    pub async fn recv_msg(&mut self) -> Result<String> {
        let read = self.write.as_mut().unwrap();
        Ok(read.next().await.unwrap()?.to_string())
    }
}
