use std::collections::HashMap;

use networking::server::TextTreeBit;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UiClientMessage {
    TextTreeInput(Option<u64>, String, String, String),
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum UiServerMessage {
    TextTreeSelection(
        Option<u64>,
        String,
        String,
        String,
        HashMap<String, TextTreeBit>,
    ),
    UIAddNotice(String),
    UIRemoveNotice(String),
    UIRequestInput(String),
}
