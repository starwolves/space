use resources::grid::CellFace;
use serde::{Deserialize, Serialize};
use typename::TypeName;

use crate::{core::TargetCell, networking::NetAction};

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ActionsServerMessage {
    TabData(Vec<NetAction>),
}

#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum ActionsClientMessage {
    TabDataEntity(u64),
    TabDataMap(i16, i16, i16, CellFace),
    TabPressed(String, Option<u64>, Option<TargetCell>, Option<u64>),
}
