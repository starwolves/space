use bevy_app::EventReader;
use bevy_ecs::{
    entity::Entity,
    system::{Query, Res, ResMut},
};
use bevy_math::Vec2;
use bevy_networking_turbulence::NetworkResource;

use crate::space::core::{
    gridmap::resources::Vec3Int,
    networking::{
        resources::{GridMapType, ReliableServerMessage, UIInputAction, UIInputNodeClass},
        send_net, NetEvent,
    },
};

use super::{components::ConnectedPlayer, resources::HandleToEntity};

pub struct NetSendServerTime {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetSendWorldEnvironment {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetUpdatePlayerCount {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct BoardingPlayer {
    pub player_handle: u32,
    pub player_character_name: String,
    pub entity: Entity,
}

pub struct InputExamineEntity {
    pub handle: u32,
    pub examine_entity_bits: u64,
    pub entity: Entity,
}

pub struct InputExamineMap {
    pub handle: u32,
    pub entity: Entity,
    pub gridmap_type: GridMapType,
    pub gridmap_cell_id: Vec3Int,
}

pub struct TextTreeInputSelection {
    pub handle: u32,
    pub menu_id: String,
    pub menu_selection: String,
    pub tab_action_id: String,
    pub belonging_entity: Option<u64>,
}

pub struct InputSceneReady {
    pub handle: u32,
    pub scene_type: String,
}

pub struct NetDoneBoarding {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetExamineEntity {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetOnBoarding {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetOnNewPlayerConnection {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct NetOnSetupUI {
    pub handle: u32,
    pub message: ReliableServerMessage,
}

pub struct InputUIInputTransmitText {
    pub handle: u32,
    pub ui_type: String,
    pub node_path: String,
    pub input_text: String,
}

pub struct InputUIInput {
    pub handle: u32,
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
    pub handle: u32,
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

pub struct InputTabDataEntity {
    pub player_entity: Entity,
    pub examine_entity_bits: u64,
}

pub fn net_system(
    mut net: ResMut<NetworkResource>,
    connected_players: Query<&ConnectedPlayer>,
    handle_to_entity: Res<HandleToEntity>,

    mut net1: EventReader<NetOnBoarding>,
    mut net2: EventReader<NetOnNewPlayerConnection>,
    mut net3: EventReader<NetOnSetupUI>,
    mut net4: EventReader<NetDoneBoarding>,
    mut net5: EventReader<NetSendWorldEnvironment>,
    mut net6: EventReader<NetOnSpawning>,
    mut net7: EventReader<NetUserName>,
    mut net8: EventReader<NetUIInputTransmitData>,
    mut net9: EventReader<NetExamineEntity>,
    mut net10: EventReader<NetTabData>,
    mut net11: EventReader<NetSendServerTime>,
    mut net12: EventReader<NetUpdatePlayerCount>,
) {
    for new_event in net1.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net2.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net3.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net4.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net5.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net6.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net7.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net8.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net9.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net10.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net11.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
    for new_event in net12.iter() {
        send_net(
            &mut net,
            &connected_players,
            &handle_to_entity,
            &NetEvent {
                handle: new_event.handle,
                message: new_event.message.clone(),
            },
        );
    }
}
