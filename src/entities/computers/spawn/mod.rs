use std::collections::HashMap;

use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};
use bevy_log::warn;

use crate::core::{
    entity::{
        events::RawSpawnEvent,
        functions::{
            process_entities_json_data::{ExportData, ExportDataRaw},
            string_to_type_converters::string_transform_to_transform,
        },
        resources::SpawnData,
        spawn::{DefaultSpawnEvent, SpawnEvent},
    },
    networking::resources::ConsoleCommandVariantValues,
};

pub mod entity_bundle;
pub mod rigidbody_bundle;

use super::components::Computer;

pub struct ComputerSummoner {
    pub computer_type: String,
}

pub fn summon_computer(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<ComputerSummoner>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Computer {
                computer_type: spawn_event.summoner.computer_type.clone(),
            });
    }
}

pub fn summon_raw_computer(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<ComputerSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != "bridgeComputer" {
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

        let computer_type;

        match data.get("computerType") {
            Some(x) => match x {
                ConsoleCommandVariantValues::String(s) => {
                    computer_type = s.to_string();
                }
                _ => {
                    warn!("computerType had incorrect variable type!");
                    computer_type = "".to_string();
                }
            },
            None => {
                warn!("computerType not found.");
                computer_type = "".to_string();
            }
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
            summoner: ComputerSummoner { computer_type },
        });
    }
}

pub fn default_summon_computer(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<ComputerSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != "bridgeComputer" {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: ComputerSummoner {
                computer_type: "".to_string(),
            },
        });
    }
}
