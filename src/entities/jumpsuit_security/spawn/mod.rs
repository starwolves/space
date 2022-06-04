pub mod entity_bundle;
pub mod inventory_item_bundle;
pub mod rigidbody_bundle;

use std::collections::HashMap;

use super::components::Jumpsuit;
use crate::core::entity::{
    events::RawSpawnEvent,
    functions::{
        process_entities_json_data::{ExportData, ExportDataRaw},
        string_to_type_converters::string_transform_to_transform,
    },
    resources::SpawnData,
    spawn::{DefaultSpawnEvent, SpawnEvent},
};
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};

pub struct JumpsuitSummoner;

pub fn summon_jumpsuit(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<JumpsuitSummoner>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Jumpsuit);
    }
}

pub fn summon_raw_jumpsuit(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<JumpsuitSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != "jumpsuitSecurity" {
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
            summoner: JumpsuitSummoner,
        });
    }
}

pub fn default_summon_jumpsuit(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<JumpsuitSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != "jumpsuitSecurity" {
            continue;
        }

        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: JumpsuitSummoner,
        });
    }
}
