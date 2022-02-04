use bevy::{prelude::Entity, math::Vec2};

use crate::space_core::{generics::{gridmap::resources::Vec3Int, networking::resources::{GridMapType, UIInputNodeClass, UIInputAction, ConsoleCommandVariantValues, ReliableServerMessage}}};


pub struct BoardingPlayer {
    pub player_handle : u32,
    pub player_character_name : String,
    pub entity : Entity,
}

pub struct InputExamineEntity{
    pub handle : u32,
    pub examine_entity_bits : u64,
    pub entity : Entity,
}

pub struct InputExamineMap{
    pub handle : u32,
    pub entity : Entity,
    pub gridmap_type : GridMapType,
    pub gridmap_cell_id : Vec3Int,
}

pub struct InputAltItemAttack {
    pub handle : u32,
    pub entity : Entity,
}

pub struct InputAttackCell {
    pub handle : u32,
    pub entity : Entity,
    pub id : Vec3Int,
}

pub struct InputAttackEntity {
    pub handle : u32,
    pub entity : Entity,
    pub target_entity_bits : u64,
}

pub struct InputChatMessage {
    pub handle : u32,
    pub message : String
}

pub struct InputMouseAction {
    pub handle : u32,
    pub entity : Entity,
    pub pressed : bool,
}

pub struct TextTreeInputSelection {
    pub handle : u32,
    pub menu_id : String,
    pub menu_selection : String,
    pub tab_action_id : String,
    pub belonging_entity : Option<u64>,
}

pub struct InputSelectBodyPart {
    pub handle : u32,
    pub entity : Entity,
    pub body_part : String,
}

pub struct InputSprinting {
    pub handle : u32,
    pub is_sprinting : bool
}

pub struct InputTabAction {
    pub handle : u32,
    pub tab_id : String,
    pub player_entity : Entity,
    pub target_entity_option: Option<u64>,
    pub belonging_entity : Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16,i16,i16)>,
}

pub struct InputToggleAutoMove {
    pub handle : u32,
    pub entity : Entity,
}

pub struct InputToggleCombatMode {
    pub handle : u32,
    pub entity : Entity,
}

pub struct InputUserName {
    pub handle : u32,
    pub entity : Entity,
    pub input_name : String,
}

pub struct InputMouseDirectionUpdate {
    pub handle : u32,
    pub entity : Entity,
    pub direction : f32,
    pub time_stamp : u64,
}

pub struct InputMovementInput {
    pub handle : u32,
    pub vector : Vec2
}

pub struct InputBuildGraphics {
    pub handle : u32
}

pub struct InputSceneReady {
    pub handle : u32,
    pub scene_type : String
}

pub struct InputTabDataEntity {
    pub handle : u32,
    pub player_entity: Entity,
    pub examine_entity_bits : u64,
}

#[derive(Debug)]
pub struct InputTabDataMap {
    pub handle : u32,
    pub player_entity : Entity,
    pub gridmap_type : GridMapType,
    pub gridmap_cell_id : Vec3Int,
}

pub struct InputUIInputTransmitText {
    pub handle: u32,
    pub ui_type: String,
    pub node_path: String,
    pub input_text : String
}

pub struct InputUIInput {
    pub handle : u32,
    pub node_class : UIInputNodeClass,
    pub action : UIInputAction,
    pub node_name : String,
    pub ui_type : String
}

pub struct InputConsoleCommand {
    pub handle : u32,
    pub entity : Entity,
    pub command_name : String,
    pub command_arguments : Vec<ConsoleCommandVariantValues>,
}

pub struct NetChatMessage {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetConsoleCommands {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetDoneBoarding {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetExamineEntity {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetOnBoarding {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetOnNewPlayerConnection {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetOnSetupUI {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetTabData {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetUIInputTransmitData {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetUserName {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetOnSpawning {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetSendServerTime {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetSendWorldEnvironment {
    pub handle : u32,
    pub message : ReliableServerMessage
}

pub struct NetUpdatePlayerCount {
    pub handle : u32,
    pub message : ReliableServerMessage
}
