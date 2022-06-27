use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};
use bevy_log::warn;

use crate::core::entity::{
    events::RawSpawnEvent,
    functions::{
        process_entities_json_data::ExportProperty,
        string_to_type_converters::string_transform_to_transform,
    },
    resources::SpawnData,
    spawn::{DefaultSpawnEvent, SpawnEvent},
};

pub mod entity_bundle;
pub mod rigidbody_bundle;

use super::components::Computer;

pub struct ComputerSummoner {
    pub computer_type: String,
}

impl ComputerSummonable for ComputerSummoner {
    fn get_computer_type(&self) -> String {
        self.computer_type.clone()
    }
}

pub trait ComputerSummonable {
    fn get_computer_type(&self) -> String;
}

pub fn summon_computer<T: ComputerSummonable + Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Computer {
                computer_type: spawn_event.summoner.get_computer_type().clone(),
            });
    }
}

pub const BRIDGE_COMPUTER_ENTITY_NAME: &str = "bridgeComputer";

pub fn summon_raw_computer(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<ComputerSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != BRIDGE_COMPUTER_ENTITY_NAME {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        let data_result: Result<Vec<ExportProperty>, _> =
            serde_json::from_str(&spawn_event.raw_entity.data);

        let mut computer_name = "".to_string();

        match data_result {
            Ok(ds) => {
                for d in ds {
                    if d.key == "computerType" {
                        if d.value_type == 4 {
                            computer_name = d.value;
                        } else {
                            warn!("Entity from entities.json had unknown type!");
                            continue;
                        }
                    }
                }
            }
            Err(_rr) => {
                warn!("Invalid json!");
                warn!("{}", spawn_event.raw_entity.data);
                continue;
            }
        }

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: ComputerSummoner {
                computer_type: computer_name,
            },
        });
    }
}

pub fn default_summon_computer(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<ComputerSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != BRIDGE_COMPUTER_ENTITY_NAME {
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
