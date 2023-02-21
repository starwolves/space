use bevy::prelude::Entity;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UiClientMessage {
    TextTreeInput(TextTreeInput),
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UiServerMessage {
    TextTreeSelection(TextTreeSelection),
    UIAddNotice(String),
    UIRemoveNotice(String),
    UIRequestInput(String),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextTreeSelection {
    pub entity: Entity,
    pub id: String,
    pub entries: Vec<String>,
    pub text: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextTreeInput {
    pub entity: Entity,
    pub id: String,
    pub entry: String,
}
