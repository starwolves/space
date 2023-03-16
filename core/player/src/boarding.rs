use bevy::math::Quat;
use bevy::{
    prelude::{Commands, Component, Entity, EventReader, ResMut, Resource},
    time::Timer,
};

/// Component with boarding data.

pub struct BoardingPlayer {
    pub player_handle: u64,
    pub player_character_name: String,
    pub entity: Entity,
}
/// Resource for slightly delayed boarding announcements.
#[derive(Default, Resource)]

pub struct BoardingAnnouncements {
    pub announcements: Vec<(String, Timer)>,
}
use crate::net::PlayerServerMessage;
use crate::spawn_points::SpawnPointRon;

use crate::connections::{OnBoard, SetupPhase};
use bevy::prelude::{EventWriter, Transform};
use bevy::time::TimerMode;
use networking::server::OutgoingReliableServerMessage;
use pawn::pawn::Spawning;

/// Event that fires when a player has successfully boarded.
pub struct PlayerBoarded {
    pub handle: u64,
    pub entity: Entity,
    pub character_name: String,
    pub account_name: String,
}
use bevy::prelude::info;

/// Do some logic when a player has successfully boarded.

pub(crate) fn player_boarded(
    mut events: EventReader<PlayerBoarded>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
) {
    for boarded_player in events.iter() {
        info!(
            "{} has boarded as \"{}\". [{}][{:?}]",
            boarded_player.account_name,
            boarded_player.character_name,
            boarded_player.handle,
            boarded_player.entity
        );

        server.send(OutgoingReliableServerMessage {
            handle: boarded_player.handle,
            message: PlayerServerMessage::PawnId(boarded_player.entity),
        });
        server.send(OutgoingReliableServerMessage {
            handle: boarded_player.handle,
            message: PlayerServerMessage::Boarded,
        });
    }
}

/// Perform initialization of spawning player.

pub(crate) fn done_boarding(
    mut spawn_points: ResMut<SpawnPoints>,
    mut server: EventWriter<OutgoingReliableServerMessage<PlayerServerMessage>>,
    mut boarding_player_event: EventReader<BoardingPlayer>,
    mut commands: Commands,

    mut asana_boarding_announcements: ResMut<BoardingAnnouncements>,
) {
    for boarding_player in boarding_player_event.iter() {
        let player_character_name = boarding_player.player_character_name.clone();
        let player_handle = boarding_player.player_handle;
        let entity_id = boarding_player.entity;

        //let mut assigned_spawn_transform = spawn_points.list[spawn_points.i].transform;
        let mut assigned_spawn_transform = Transform::IDENTITY;
        assigned_spawn_transform.translation.y = 0.;
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
        server.send(OutgoingReliableServerMessage {
            handle: player_handle,
            message: PlayerServerMessage::InitGame,
        });

        asana_boarding_announcements.announcements.push((
            player_character_name + " is now on board.",
            Timer::from_seconds(2., TimerMode::Once),
        ));
    }
}

/// The component for players that haven't yet boarded.
#[derive(Component)]

pub struct SoftPlayer;

/// Client input submitting text event.

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

/// A spawn point in which players will spawn.

pub struct SpawnPoint {
    pub point_type: String,
    pub transform: Transform,
}

impl SpawnPoint {
    pub fn new(&self) -> SpawnPoint {
        let mut this_transform = self.transform.clone();

        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;

        SpawnPoint {
            point_type: self.point_type.clone(),
            transform: this_transform,
        }
    }
}

impl SpawnPointRon {
    pub fn new(&self) -> SpawnPoint {
        let mut this_transform = Transform::from_translation(self.translation);

        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;

        SpawnPoint {
            point_type: self.point_type.clone(),
            transform: this_transform,
        }
    }
}

/// Resource containing all available spawn points for players.
#[derive(Default, Resource)]

pub struct SpawnPoints {
    pub list: Vec<SpawnPoint>,
    pub i: usize,
}
use serde::Deserialize;
/// Raw json.
#[derive(Deserialize)]

pub struct SpawnPointRaw {
    pub point_type: String,
    pub transform: String,
}
