use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::networking::NetAction;

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ActionsServerMessage {
    TabData(Vec<NetAction>),
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ActionsClientMessage {
    TabDataEntity(u64),
    TabDataMap(i16, i16, i16),
    TabPressed(String, Option<u64>, Option<(i16, i16, i16)>, Option<u64>),
}
