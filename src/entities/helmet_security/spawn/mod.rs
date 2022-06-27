use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};

use crate::core::entity::{
    events::RawSpawnEvent,
    functions::string_to_type_converters::string_transform_to_transform,
    resources::SpawnData,
    spawn::{DefaultSpawnEvent, SpawnEvent},
};

use super::components::Helmet;

pub mod entity_bundle;
pub mod inventory_item_bundle;
pub mod rigidbody_bundle;

pub struct HelmetSummoner;

pub fn summon_helmet<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(Helmet);
    }
}

pub fn summon_raw_helmet(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_computer: EventWriter<SpawnEvent<HelmetSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != HELMET_SECURITY_ENTITY_NAME {
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
            summoner: HelmetSummoner,
        });
    }
}

pub const HELMET_SECURITY_ENTITY_NAME: &str = "helmetSecurity";

pub fn default_summon_helmet_security(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<HelmetSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != HELMET_SECURITY_ENTITY_NAME {
            continue;
        }
        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: HelmetSummoner,
        });
    }
}
