use crate::{connection_events::send_server_configuration, health_ui::ClientHealthUICache};
use bevy::{
    math::Vec2,
    prelude::{info, Commands, Component, EventReader, EventWriter, Query, Res, ResMut},
};
use bevy_renet::renet::ServerEvent;
use console_commands::commands::{AllConsoleCommands, GiveAllRCON};
use gridmap::grid::GridmapData;
use map::map_input::MapData;
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;
use pawn::pawn::{ControllerInput, PawnDesignation, PersistentPlayerData, UsedNames};
use resources::core::{ConnectedPlayer, HandleToEntity, ServerId, TickRate};

#[cfg(feature = "server")]
#[derive(NetMessage)]
pub struct NetPlayerConn {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

/// The component for players that are requesting boarding.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Boarding;

/// The component for entities int he boarding phase.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct SetupPhase;

/// The component for entities that are done boarding and about to spawn in on the ship. A stage after [Boarding].
#[derive(Component)]
#[cfg(feature = "server")]
pub struct OnBoard;

/// Data for spawning.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct SpawnPawnData {
    pub persistent_player_data: PersistentPlayerData,
    pub connected_player_option: Option<ConnectedPlayer>,
    pub inventory_setup: Vec<(String, String)>,
    pub designation: PawnDesignation,
}

/// Manage client connection events.
#[cfg(feature = "server")]
pub(crate) fn connections(
    tick_rate: Res<TickRate>,
    mut auth_id_i: ResMut<AuthidI>,
    server_id: Res<ServerId>,
    mut handle_to_entity: ResMut<HandleToEntity>,
    mut commands: Commands,
    mut reader: EventReader<ServerEvent>,
    mut net_on_new_player_connection: EventWriter<NetPlayerConn>,
    mut connected_players: Query<(
        &mut PersistentPlayerData,
        &mut ConnectedPlayer,
        &mut ControllerInput,
    )>,
    mut used_names: ResMut<UsedNames>,
    mut client_health_ui_cache: ResMut<ClientHealthUICache>,
    gridmap_data: Res<GridmapData>,
    map_data: Res<MapData>,
    console_commands: Res<AllConsoleCommands>,
    give_all_rcon: Res<GiveAllRCON>,
) {
    for event in reader.iter() {
        match event {
            ServerEvent::ClientConnected(handle, _) => {
                info!("Incoming connection on [{}]", handle,);

                send_server_configuration(
                    &mut net_on_new_player_connection,
                    handle,
                    &tick_rate,
                    &mut auth_id_i,
                    &server_id,
                    &mut handle_to_entity,
                    &mut commands,
                    &mut used_names,
                    &gridmap_data,
                    &map_data,
                    &console_commands,
                    &give_all_rcon,
                );
            }
            ServerEvent::ClientDisconnected(handle) => {
                on_player_disconnect(
                    *handle,
                    &mut handle_to_entity,
                    &mut connected_players,
                    &mut used_names,
                    &mut client_health_ui_cache,
                );
            }
        }
    }
}

/// On player disconnect as a function.
#[cfg(feature = "server")]
pub fn on_player_disconnect(
    handle: u64,
    handle_to_entity: &mut ResMut<HandleToEntity>,
    connected_players: &mut Query<(
        &mut PersistentPlayerData,
        &mut ConnectedPlayer,
        &mut ControllerInput,
    )>,
    used_names: &mut ResMut<UsedNames>,
    client_health_ui_cache: &mut ResMut<ClientHealthUICache>,
) {
    info!("[{}] disconnected!", handle);

    let mut entity = None;

    match handle_to_entity.map.get(&handle) {
        Some(ent) => {
            entity = Some(*ent);
            match connected_players.get_mut(*ent) {
                Ok((
                    mut persistent_player_data,
                    mut connected_player_component,
                    mut player_input_component,
                )) => {
                    connected_player_component.connected = false;
                    player_input_component.movement_vector = Vec2::ZERO;
                    player_input_component.sprinting = false;
                    player_input_component.is_mouse_action_pressed = false;
                    player_input_component.auto_move_enabled = false;

                    // When reconnecting into the old pawn works remove this.
                    used_names
                        .account_name
                        .remove(&persistent_player_data.account_name);
                    persistent_player_data.account_name = "disconnectedUser".to_string();
                }
                Err(_rr) => {}
            }
        }
        None => {}
    }

    match entity {
        Some(ent) => {
            handle_to_entity.inv_map.remove(&ent);
            client_health_ui_cache.cache.remove(&ent);
        }
        None => {}
    }

    handle_to_entity.map.remove(&handle);
}

/// Resource with the current incremented authentication ID.
#[derive(Default)]
#[cfg(feature = "server")]
pub(crate) struct AuthidI {
    pub i: u16,
}
