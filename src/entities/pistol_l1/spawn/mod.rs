pub mod entity_bundle;
pub mod inventory_item_bundle;
pub mod rigidbody_bundle;

use std::collections::HashMap;

use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};

use crate::core::entity::{
    events::RawSpawnEvent,
    functions::{
        process_entities_json_data::{ExportData, ExportDataRaw},
        string_to_type_converters::string_transform_to_transform,
    },
    resources::SpawnData,
    spawn::{DefaultSpawnEvent, SpawnEvent},
};

use super::components::PistolL1;

pub struct PistolL1Summoner;

pub fn summon_pistol_l1(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<PistolL1Summoner>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(PistolL1);
    }
}

pub fn summon_raw_pistol_l1(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<PistolL1Summoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != PISTOL_L1_ENTITY_NAME {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        let data;

        if &spawn_event.raw_entity.data != "" {
            let raw_export_data: ExportDataRaw = ExportDataRaw {
                properties: serde_json::from_str(&spawn_event.raw_entity.data)
                    .expect("load_raw_map_entities.rs Error parsing standard entity data."),
            };

            data = ExportData::new(raw_export_data).properties;
        } else {
            data = HashMap::new();
        }

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                correct_transform: true,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                properties: data,
                ..Default::default()
            },
            summoner: PistolL1Summoner,
        });
    }
}

pub const PISTOL_L1_ENTITY_NAME: &str = "pistolL1";

pub fn default_summon_pistol_l1(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<PistolL1Summoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != PISTOL_L1_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: PistolL1Summoner,
        });
    }
}
