use bevy::math::{Quat, Vec3};
use bevy::prelude::{info, Res};

use crate::meta::EntityDataResource;

/// Print startup entity data to console.
#[cfg(feature = "server")]
pub(crate) fn startup_entities(entity_data: Res<EntityDataResource>) {
    info!("Loaded {} different entity types.", entity_data.data.len());
}

use serde::{Deserialize, Serialize};
/// ron entity.
#[derive(Serialize, Deserialize, Clone)]
#[cfg(any(feature = "server", feature = "client"))]
pub struct RawEntityRon {
    pub entity_type: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub data: String,
}
