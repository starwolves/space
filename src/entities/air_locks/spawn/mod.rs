use bevy_ecs::{
    event::{EventReader, EventWriter},
    system::Commands,
};

use crate::core::{
    entity::{
        events::RawSpawnEvent,
        functions::string_to_type_converters::string_transform_to_transform,
        resources::SpawnData,
        spawn::{DefaultSpawnEvent, SpawnEvent},
    },
    pawn::components::ShipAuthorizationEnum,
};

pub mod entity_bundle;
pub mod rigidbody_bundle;

use super::components::AirLock;

pub struct AirlockSummoner;

pub fn summon_air_lock<T: Send + Sync + 'static>(
    mut commands: Commands,
    mut airlock_spawns: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in airlock_spawns.iter() {
        commands
            .entity(spawn_event.spawn_data.entity)
            .insert(AirLock {
                access_permissions: vec![ShipAuthorizationEnum::Security],
                ..Default::default()
            });
    }
}

pub const SECURITY_AIRLOCK_ENTITY_NAME: &str = "securityAirLock1";
pub const BRIDGE_AIRLOCK_ENTITY_NAME: &str = "bridgeAirLock";
pub const GOVERNMENT_AIRLOCK_ENTITY_NAME: &str = "governmentAirLock";
pub const VACUUM_AIRLOCK_ENTITY_NAME: &str = "vacuumAirLock";

pub fn default_summon_air_lock(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<AirlockSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name != SECURITY_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != BRIDGE_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != GOVERNMENT_AIRLOCK_ENTITY_NAME
            || spawn_event.spawn_data.entity_name != VACUUM_AIRLOCK_ENTITY_NAME
        {
            continue;
        }

        spawner.send(SpawnEvent {
            spawn_data: spawn_event.spawn_data.clone(),
            summoner: AirlockSummoner,
        });
    }
}

pub fn summon_raw_air_lock(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_air_lock: EventWriter<SpawnEvent<AirlockSummoner>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        if spawn_event.raw_entity.entity_type != SECURITY_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != BRIDGE_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != GOVERNMENT_AIRLOCK_ENTITY_NAME
            && spawn_event.raw_entity.entity_type != VACUUM_AIRLOCK_ENTITY_NAME
        {
            continue;
        }

        let entity_transform = string_transform_to_transform(&spawn_event.raw_entity.transform);

        summon_air_lock.send(SpawnEvent {
            spawn_data: SpawnData {
                entity_transform: entity_transform,
                default_map_spawn: true,
                entity_name: spawn_event.raw_entity.entity_type.clone(),
                entity: commands.spawn().id(),
                raw_entity_option: Some(spawn_event.raw_entity.clone()),
                ..Default::default()
            },
            summoner: AirlockSummoner,
        });
    }
}
