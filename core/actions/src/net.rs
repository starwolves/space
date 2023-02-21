use bevy::prelude::Entity;
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
    TabDataEntity(Entity),
    TabDataMap(i16, i16, i16, CellFace),
    TabPressed(TabPressed),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TabPressed {
    pub id: String,
    pub action_taker: Entity,
    pub action_taker_item: Option<Entity>,
    pub target_cell_option: Option<TargetCell>,
    pub target_entity_option: Option<Entity>,
}
