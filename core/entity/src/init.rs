use bevy::math::{Quat, Vec3};
use bevy::prelude::info;

use serde::{Deserialize, Serialize};
/// ron entity.
#[derive(Serialize, Deserialize, Clone)]

pub struct RawEntityRon {
    pub entity_type: String,
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    pub data: String,
}
use std::path::Path;

use bevy::prelude::EventWriter;
use std::fs;

use crate::entity_data::RawSpawnEvent;

/// Build the entities from ron.

pub(crate) fn load_ron_entities(mut raw_spawner: EventWriter<RawSpawnEvent>) {
    let entities_ron = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("entities.ron");
    let current_map_entities_raw_ron: String =
        fs::read_to_string(entities_ron).expect("Error reading map entities.ron file from drive.");
    let current_map_entities_data: Vec<RawEntityRon> = ron::from_str(&current_map_entities_raw_ron)
        .expect("Error parsing map entities.ron String.");

    for raw_entity in current_map_entities_data.iter() {
        raw_spawner.send(RawSpawnEvent {
            raw_entity: raw_entity.clone(),
        });
    }
    info!("Spawned {} entities.", current_map_entities_data.len());
}
