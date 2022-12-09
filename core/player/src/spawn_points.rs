use bevy::math::Vec3;
use bevy::prelude::{Component, Quat, Transform};
use networking::server::ConnectedPlayer;
use serde::{Deserialize, Serialize};

use crate::boarding::PersistentPlayerData;

/// Component that contains the spawn data of a to-be-spawned entity.
#[derive(Component)]
#[cfg(feature = "server")]
pub struct Spawning {
    pub transform: Transform,
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
#[cfg(feature = "server")]
#[derive(Serialize, Deserialize)]
pub struct SpawnPointRon {
    pub point_type: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
