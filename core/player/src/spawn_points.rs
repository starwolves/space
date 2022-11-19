use bevy::prelude::{Component, Quat, Resource, Transform};
use data_converters::converters::string_transform_to_transform;
use networking::server::ConnectedPlayer;
use serde::Deserialize;

use crate::boarding::PersistentPlayerData;

/// Raw json.
#[derive(Deserialize)]
#[cfg(feature = "server")]
pub struct SpawnPointRaw {
    pub point_type: String,
    pub transform: String,
}
/// Resource containing all available spawn points for players.
#[derive(Default, Resource)]
#[cfg(feature = "server")]
pub struct SpawnPoints {
    pub list: Vec<SpawnPoint>,
    pub i: usize,
}
/// Component that contains the spawn data of a to-be-spawned entity.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Spawning {
    pub transform: Transform,
}

/// A spawn point in which players will spawn.
#[cfg(feature = "server")]
pub struct SpawnPoint {
    pub point_type: String,
    pub transform: Transform,
}

#[cfg(feature = "server")]
impl SpawnPoint {
    pub fn new(raw: &SpawnPointRaw) -> SpawnPoint {
        let mut this_transform = string_transform_to_transform(&raw.transform);

        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;

        SpawnPoint {
            point_type: raw.point_type.clone(),
            transform: this_transform,
        }
    }
}

/// Data for spawning.
#[derive(Clone)]
#[cfg(feature = "server")]
pub struct SpawnPawnData {
    pub persistent_player_data: PersistentPlayerData,
    pub connected_player_option: Option<ConnectedPlayer>,
    pub inventory_setup: Vec<(String, String)>,
    pub designation: PawnDesignation,
}

#[derive(Clone)]
#[cfg(feature = "server")]
pub enum PawnDesignation {
    Showcase,
    Player,
    Dummy,
    Ai,
}
