use std::io::{stdout, Write};

use super::http;
use anyhow::{Ok, Result};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message as OtherMessage, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Clone)]
pub struct Conversation {
    pub client_id: String,
    pub conversation_id: String,
    pub conversation_signature: String,
    pub invocation_id: i32,
}

impl Conversation {
    pub async fn new() -> Result<Self> {
        let json_str = http::Client::new()
            .get_html("https://www.bing.com/turing/conversation/create")
            .await
            .map_err(|e| {
                println!("create_conversation error: {}", e);
                e
            })?;
        if gjson::get(&json_str, "result.value").to_string() == "Success" {
            // println!("create_conversation success: {}", json_str);
            Ok(Conversation {
                client_id: gjson::get(&json_str, "clientId").to_string(),
                conversation_id: gjson::get(&json_str, "conversationId").to_string(),
                conversation_signature: gjson::get(&json_str, "conversationSignature").to_string(),
                invocation_id: 0,
            })
        } else {
            Err(anyhow::anyhow!(
                "create_conversation error: {}",
                gjson::get(&json_str, "result.message").to_string()
            ))
        }
    }
}

pub struct ChatHub {
    conversation: Conversation,
    read: Option<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, OtherMessage>>,
    write: Option<SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
}

impl ChatHub {
    pub async fn new() -> Result<Self> {
        let conversation = Conversation::new().await?;
        Ok(Self {
            conversation,
            read: None,
            write: None,
        })
    }

    pub async fn create_websocket(&mut self) -> Result<()> {
        let url = "wss://sydney.bing.com/sydney/ChatHub";
        let (ws_stream, _) = tokio_tungstenite::connect_async(url).await?;
        // println!("Connected to the server");
        let (write, read) = ws_stream.split();
        self.write = Some(read);
        self.read = Some(write);
        Ok(())
    }

    pub async fn send_protocol(&mut self) -> Result<()> {
        let write = self.read.as_mut().unwrap();
        let read = self.write.as_mut().unwrap();
        write
            .send(OtherMessage::Text(
                r#"{"protocol": "json", "version": 1}"#.to_string() + "",
            ))
            .await?;
        read.next().await.unwrap()?;
        Ok(())
    }

    pub async fn send_msg(&mut self, msg: &str) -> Result<()> {
        let write = self.read.as_mut().unwrap();
        let read = self.write.as_mut().unwrap();

        write
            .send(OtherMessage::Text(fill_msg(msg, &self.conversation)))
            .await?;
        self.conversation.invocation_id += 1;

        println!("Bing:");
        let mut index = 0;
        loop {
            let msg = read.next().await.unwrap()?;
            let msg = msg.to_string();
            // println!("{}", msg);
            if gjson::get(&msg, "type").i32() == 1 {
                let answer = gjson::get(&msg, "arguments.0.messages.0.text").to_string();
                if !answer.is_empty() {
                    print!("{}", utf8_slice::from(&answer, index));
                    stdout().flush().unwrap();
                    index = utf8_slice::len(&answer);
                }
            }
            if gjson::get(&msg, "type").i32() == 2 {
                break;
            }
        }

        Ok(())
    }

    pub fn input() -> String {
        println!("You:");
        let mut input = String::new();
        loop {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            if line.trim().is_empty() {
                break;
            }
            input.push_str(&line);
        }
        input
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            let input = Self::input();
            if input.trim().is_empty() {
                continue;
            }
            if input.trim() == "!exit" || input.trim() == "!quit" || input.trim() == "!q" {
                break;
            }
            self.send_msg(&input).await?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chat_hub() {
        let mut chat_hub = ChatHub::new().await.unwrap();
        chat_hub.create_websocket().await.unwrap();
        chat_hub.send_protocol().await.unwrap();
        chat_hub.send_msg("你好").await.unwrap();
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Args {
    pub arguments: Vec<Argument>,
    #[serde(rename = "invocationId")]
    pub invocation_id: String,
    pub target: String,
    #[serde(rename = "type")]
    pub type_: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Argument {
    pub source: String,
    #[serde(rename = "optionsSets")]
    pub options_sets: Vec<String>,
    #[serde(rename = "isStartOfSession")]
    pub is_start_of_session: bool,
    pub message: Message,
    #[serde(rename = "conversationSignature")]
    pub conversation_signature: String,
    pub participant: Participant,
    #[serde(rename = "conversationId")]
    pub conversation_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub author: String,
    #[serde(rename = "inputMethod")]
    pub input_method: String,
    pub text: String,
    #[serde(rename = "messageType")]
    pub message_type: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Participant {
    pub id: String,
}

fn fill_msg(msg: &str, conversation: &Conversation) -> String {
    let args = Args {
        arguments: vec![Argument {
            source: "cib".to_string(),
            options_sets: vec![
                "nlu_direct_response_filter".to_string(),
                "deepleo".to_string(),
                "enable_debug_commands".to_string(),
                "disable_emoji_spoken_text".to_string(),
                "responsible_ai_policy_235".to_string(),
                "enablemm".to_string(),
            ],
            is_start_of_session: conversation.invocation_id == 0,
            message: Message {
                author: "user".to_string(),
                input_method: "Keyboard".to_string(),
                text: msg.to_string(),
                message_type: "Chat".to_string(),
            },
            conversation_signature: conversation.conversation_signature.clone(),
            participant: Participant {
                id: conversation.client_id.clone(),
            },
            conversation_id: conversation.conversation_id.clone(),
        }],
        invocation_id: conversation.invocation_id.to_string(),
        target: "chat".to_string(),
        type_: 4,
    };
    serde_json::to_string(&args).unwrap_or_default() + ""
}
