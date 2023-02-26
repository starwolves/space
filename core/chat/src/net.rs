use bevy::prelude::Color;
use serde::{Deserialize, Serialize};
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ChatClientMessage {
    InputChatMessage(String),
}
/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ChatServerMessage {
    ChatMessage(ChatMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub sections: Vec<NetTextSection>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetTextSection {
    pub text: String,
    pub font: u16,
    pub font_size: f32,
    pub color: Color,
}
