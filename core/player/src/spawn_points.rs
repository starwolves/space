use bevy::math::Vec3;
use bevy::prelude::Quat;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
#[derive(Serialize, Deserialize)]
pub struct SpawnPointRon {
    pub point_type: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}
