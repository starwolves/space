use bevy::{
    prelude::{Commands, Entity, EventReader, ResMut, Resource},
    time::Timer,
};
use bevy_renet::renet::ClientId;
use networking::client::IncomingReliableServerMessage;
use resources::player::SoftPlayer;
use ui::cursor::GrabCursor;

/// Component with boarding data.
#[derive(Event)]
pub struct BoardingPlayer {
    pub player_handle: ClientId,
    pub player_character_name: String,
    pub entity: Entity,
}
/// Resource for slightly delayed boarding announcements.
#[derive(Default, Resource)]

pub struct BoardingAnnouncements {
    pub announcements: Vec<(String, Timer)>,
}
use crate::net::PlayerServerMessage;

use crate::connections::SetupPhase;
use bevy::prelude::{Event, EventWriter, Transform};
use bevy::time::TimerMode;
use networking::server::OutgoingReliableServerMessage;
use pawn::pawn::SpawningPlayer;

/// Event that fires when a player has successfully boarded.
#[derive(Event)]
pub struct PlayerBoarded {
    pub handle: ClientId,
    pub entity: Entity,
    pub character_name: String,
    pub account_name: String,
}
use bevy::log::info;

/// Do some logic when a player has successfully boarded.

pub fn player_boarded(
    mut events: EventReader<PlayerBoarded>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
) {
    for boarded_player in events.read() {
        info!(
            "{} has boarded as \"{}\" [{}] {:?}.",
            boarded_player.account_name,
            boarded_player.character_name,
            boarded_player.handle,
            boarded_player.entity
        );
        server.send(OutgoingReliableServerMessage {
            handle: boarded_player.handle,
            message: PlayerServerMessage::Boarded,
        });
    }
}

/// Perform initialization of spawning player.

pub fn done_boarding(
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    mut boarding_player_event: EventReader<BoardingPlayer>,
    mut commands: Commands,
    mut spawning: EventWriter<SpawningPlayer>,
    mut asana_boarding_announcements: ResMut<BoardingAnnouncements>,
) {
    for boarding_player in boarding_player_event.read() {
        let player_character_name = boarding_player.player_character_name.clone();
        let player_handle = boarding_player.player_handle;
        let entity_id = boarding_player.entity;

        //let mut assigned_spawn_transform = spawn_points.list[spawn_points.i].transform;
        let mut assigned_spawn_transform = Transform::IDENTITY;
        assigned_spawn_transform.translation.y = 1.8 - 0.5 - 0.5 + 0.2;
        commands
            .entity(entity_id)
            .remove::<(SetupPhase, SoftPlayer)>();
        spawning.send(SpawningPlayer {
            transform: assigned_spawn_transform,
            entity: entity_id,
        });

        server.send(OutgoingReliableServerMessage {
            handle: player_handle,
            message: PlayerServerMessage::InitGame,
        });

        asana_boarding_announcements.announcements.push((
            player_character_name + " has boarded.",
            Timer::from_seconds(2., TimerMode::Once),
        ));
    }
}

pub(crate) fn grab_mouse_on_board(
    mut net: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut grab: EventWriter<GrabCursor>,
) {
    for message in net.read() {
        match &message.message {
            PlayerServerMessage::Boarded => {
                grab.send(GrabCursor);
            }
            _ => (),
        }
    }
}
