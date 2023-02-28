use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::bing;

use super::conversation::Conversation;

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

pub fn fill_msg(msg: &str, conversation: &Conversation) -> Result<String> {
    let args = Args {
        arguments: vec![Argument {
            source: "cib".to_string(),
            options_sets: vec![
                "deepleo".to_string(),
                "enable_debug_commands".to_string(),
                "disable_emoji_spoken_text".to_string(),
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
    Ok(serde_json::to_string(&args)? + bing::SPLIT_CHAR.to_string().as_str())
}
