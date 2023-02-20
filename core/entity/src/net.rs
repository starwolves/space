use bevy::prelude::Entity;
use bevy::prelude::Quat;
use bevy::prelude::Vec3;
use serde::Deserialize;
use serde::Serialize;
use typename::TypeName;

use std::collections::HashMap;

use networking::server::EntityUpdateData;

use crate::entity_data::EntityWorldType;

/// Gets serialized and sent over the net, this is the client message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum EntityClientMessage {
    ExamineEntity(u64),
}

/// Gets serialized and sent over the net, this is the server message.
#[derive(Serialize, Deserialize, Debug, Clone, TypeName)]

pub enum EntityServerMessage {
    EntityUpdate(
        u64,
        HashMap<String, HashMap<String, EntityUpdateData>>,
        bool,
        EntityWorldType,
    ),
    LoadEntity(LoadEntity),
    UnloadEntity(Entity),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadEntity {
    pub type_id: u16,
    pub entity: Entity,
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Quat,
    pub holder_entity: Option<Entity>,
}
