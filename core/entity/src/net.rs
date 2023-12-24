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
    LinkPeer(LinkPeer),
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LinkPeer {
    pub handle: u16,
    pub server_entity: Entity,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadEntity {
    pub type_id: u16,
    pub entity: Entity,
    pub holder_entity: Option<Entity>,
    pub physics_data: PhysicsData,
    pub entity_updates_reliable: Vec<Vec<u8>>,
    pub entity_updates_unreliable: Vec<Vec<u8>>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhysicsData {
    pub rotation: Quat,
    pub velocity: Vec3,
    pub translation: Vec3,
    pub angular_velocity: Vec3,
}
