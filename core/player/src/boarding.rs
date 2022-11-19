use crate::connection::{Boarding, OnBoard, SetupPhase};
use bevy::{
    prelude::{
        info, Added, Commands, Component, Entity, EventReader, EventWriter, Query, ResMut, Resource,
    },
    time::Timer,
};
use networking::server::ReliableServerMessage;
use networking_macros::NetMessage;

/// Component with boarding data.
#[cfg(feature = "server")]
pub struct BoardingPlayer {
    pub player_handle: u64,
    pub player_character_name: String,
    pub entity: Entity,
}
/// Resource for slightly delayed boarding announcements.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub struct BoardingAnnouncements {
    pub announcements: Vec<(String, Timer)>,
}
use crate::networking::NetDoneBoarding;
use crate::spawn_points::SpawnPoints;
use crate::spawn_points::Spawning;
use networking::server::ServerConfigMessage;

/// Perform initialization of spawning player.
#[cfg(feature = "server")]
pub(crate) fn done_boarding(
    mut spawn_points: ResMut<SpawnPoints>,
    mut net_done_boarding: EventWriter<NetDoneBoarding>,
    mut boarding_player_event: EventReader<BoardingPlayer>,
    mut commands: Commands,

    mut asana_boarding_announcements: ResMut<BoardingAnnouncements>,
) {
    use bevy::time::TimerMode;
    use text_api::core::get_talk_spaces;

    for boarding_player in boarding_player_event.iter() {
        let player_character_name = boarding_player.player_character_name.clone();
        let player_handle = boarding_player.player_handle;
        let entity_id = boarding_player.entity;

        info!(
            "{} [{}] has boarded the spaceship.",
            player_character_name, player_handle
        );

        let assigned_spawn_transform = spawn_points.list[spawn_points.i].transform;

        commands
            .entity(entity_id)
            .insert((
                OnBoard,
                Spawning {
                    transform: assigned_spawn_transform,
                },
            ))
            .remove::<(SetupPhase, SoftPlayer)>();

        spawn_points.i += 1;

        if spawn_points.i >= spawn_points.list.len() {
            spawn_points.i = 0;
        }

        // Queue net_code message for client so he goes back to the main scene and ditches setupUI.
        net_done_boarding.send(NetDoneBoarding {
            handle: player_handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::ChangeScene(
                true,
                "main".to_string(),
            )),
        });

        let talk_spaces = get_talk_spaces();

        net_done_boarding.send(NetDoneBoarding {
            handle: player_handle,
            message: ReliableServerMessage::ConfigMessage(ServerConfigMessage::TalkSpaces(
                talk_spaces,
            )),
        });

        asana_boarding_announcements.announcements.push((
            ";Security Officer ".to_owned() + &player_character_name + " is now on board.",
            Timer::from_seconds(2., TimerMode::Once),
        ));
    }
}

/// Persistent player data component.
#[derive(Clone, Component)]
#[cfg(feature = "server")]
pub struct PersistentPlayerData {
    pub account_name_is_set: bool,
    pub character_name: String,
    pub account_name: String,
}
#[cfg(feature = "server")]
impl Default for PersistentPlayerData {
    fn default() -> Self {
        Self {
            account_name_is_set: false,
            character_name: "".to_string(),
            account_name: "".to_string(),
        }
    }
}
use networking::server::ConnectedPlayer;

/// Manage client boarding.
#[cfg(feature = "server")]
pub(crate) fn on_boarding(
    query: Query<&ConnectedPlayer, Added<Boarding>>,
    mut net_on_boarding: EventWriter<NetOnBoarding>,
) {
    for connected_player_component in query.iter() {
        net_on_boarding.send(NetOnBoarding {
            handle: connected_player_component.handle,
            message: ReliableServerMessage::UIRequestInput("setupUI".to_string()),
        });
    }
}
use networking::server::PendingMessage;
use networking::server::PendingNetworkMessage;

/// The component for players that haven't yet boarded.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct SoftPlayer;

/// Client input submitting text event.
#[cfg(feature = "server")]
pub struct InputUIInputTransmitText {
    /// Handle of the connection that input this.
    pub handle: u64,
    /// The UI this input was submitted from.
    pub ui_type: String,
    /// The Godot node path of the input element.
    pub node_path: String,
    /// The input text from the client.
    pub input_text: String,
}

#[derive(NetMessage)]
#[cfg(feature = "server")]
pub(crate) struct NetOnBoarding {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
