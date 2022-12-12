use crate::entity_data::RawEntity;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{info, Res};
use std::fs;
use std::path::Path;

use crate::meta::EntityDataResource;

/// Print startup entity data to console.
#[cfg(feature = "server")]
pub(crate) fn startup_entities(entity_data: Res<EntityDataResource>) {
    info!("Loaded {} different entity types.", entity_data.data.len());
}

use serde::{Deserialize, Serialize};
/// ron entity.
#[derive(Serialize, Deserialize, Clone)]
#[cfg(feature = "server")]
pub struct RawEntityRon {
    pub entity_type: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub data: String,
}
use resources::converters::string_transform_to_transform;
#[allow(dead_code)]
pub fn json_entities() {
    let entities_json = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("entities.json");
    let current_map_entities_raw_json: String = fs::read_to_string(entities_json.clone()).unwrap();
    let current_map_entities_data: Vec<RawEntity> =
        serde_json::from_str(&current_map_entities_raw_json).unwrap();

    let mut ron_data = vec![];

    for raw in current_map_entities_data {
        let transform = string_transform_to_transform(&raw.transform);
        ron_data.push(RawEntityRon {
            entity_type: raw.entity_type.to_string(),
            translation: transform.translation,
            rotation: transform.rotation,
            scale: transform.scale,
            data: raw.data.to_string(),
        })
    }
    let raw = ron::to_string::<Vec<RawEntityRon>>(&ron_data).unwrap();
    fs::write(
        Path::new("data")
            .join("maps")
            .join("bullseye")
            .join("entities.ron"),
        raw,
    )
    .unwrap();
}
