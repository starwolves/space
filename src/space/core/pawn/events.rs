use bevy_ecs::entity::Entity;
use bevy_math::Vec2;

use crate::space::core::{
    gridmap::resources::Vec3Int,
    networking::resources::{GridMapType, ReliableServerMessage},
};

pub struct InputAltItemAttack {
    pub entity: Entity,
}

pub struct InputAttackCell {
    pub entity: Entity,
    pub id: Vec3Int,
}

pub struct InputAttackEntity {
    pub entity: Entity,
    pub target_entity_bits: u64,
}

pub struct InputMouseAction {
    pub entity: Entity,
    pub pressed: bool,
}

pub struct InputSelectBodyPart {
    pub entity: Entity,
    pub body_part: String,
}

pub struct InputSprinting {
    pub entity: Entity,
    pub is_sprinting: bool,
}

pub struct InputToggleAutoMove {
    pub entity: Entity,
}

pub struct InputToggleCombatMode {
    pub entity: Entity,
}

pub struct InputUserName {
    pub entity: Entity,
    pub input_name: String,
}

pub struct InputMouseDirectionUpdate {
    pub entity: Entity,
    pub direction: f32,
    pub time_stamp: u64,
}

pub struct InputMovementInput {
    pub player_entity: Entity,
    pub vector: Vec2,
}

pub struct InputBuildGraphics {
    pub handle: u32,
}

pub struct InputTabDataEntity {
    pub player_entity: Entity,
    pub examine_entity_bits: u64,
}

#[derive(Debug)]
pub struct InputTabDataMap {
    pub player_entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
}

pub struct NetTabData {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetUIInputTransmitData {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetUserName {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetOnSpawning {
    pub handle: u32,
    pub message: ReliableServerMessage,
}
