pub mod entity_bundle;
pub mod inventory_item_bundle;
pub mod rigidbody_bundle;

use super::components::Jumpsuit;
use crate::core::entity::{
    events::RawSpawnEvent,
    functions::string_to_type_converters::string_transform_to_transform,
    resources::SpawnData,
    spawn::{DefaultSpawnEvent, SpawnEvent},
};
use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};

pub struct JumpsuitSummoner;

pub fn summon_jumpsuit<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Jumpsuit);
    }
}

pub const JUMPSUIT_SECURITY_ENTITY_NAME: &str = "jumpsuitSecurity";

pub fn summon_raw_jumpsuit(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<JumpsuitSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != JUMPSUIT_SECURITY_ENTITY_NAME {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        summon_computer.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
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
        if spawn_event.spawn_data.entity_name != JUMPSUIT_SECURITY_ENTITY_NAME {
            continue;
        }

        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: JumpsuitSummoner,
        });
    }
}
