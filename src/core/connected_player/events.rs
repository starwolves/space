use bevy_ecs::entity::Entity;
use bevy_math::Vec2;

use crate::core::{
    gridmap::resources::Vec3Int,
    networking::resources::{GridMapType, ReliableServerMessage, UIInputAction, UIInputNodeClass},
};

pub struct NetSendServerTime {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetSendWorldEnvironment {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUpdatePlayerCount {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct BoardingPlayer {
    pub player_handle: u64,
    pub player_character_name: String,
    pub entity: Entity,
}

pub struct InputExamineEntity {
    pub handle: u64,
    pub examine_entity_bits: u64,
    pub entity: Entity,
}

pub struct InputExamineMap {
    pub handle: u64,
    pub entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
}

pub struct TextTreeInputSelection {
    pub handle: u64,
    pub menu_id: String,
    pub menu_selection: String,
    pub tab_action_id: String,
    pub belonging_entity: Option<u64>,
}

pub struct InputSceneReady {
    pub handle: u64,
    pub scene_type: String,
}

pub struct NetDoneBoarding {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetExamineEntity {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnBoarding {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnNewPlayerConnection {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnSetupUI {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct InputUIInputTransmitText {
    pub handle: u64,
    pub ui_type: String,
    pub node_path: String,
    pub input_text: String,
}

pub struct InputUIInput {
    pub handle: u64,
    pub node_class: UIInputNodeClass,
    pub action: UIInputAction,
    pub node_name: String,
    pub ui_type: String,
}

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
    pub handle: u64,
}

#[derive(Debug)]
pub struct InputTabDataMap {
    pub player_entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
}

pub struct NetTabData {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUIInputTransmitData {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetUserName {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct NetOnSpawning {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub struct InputTabDataEntity {
    pub player_entity: Entity,
    pub examine_entity_bits: u64,
}
