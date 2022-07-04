use bevy::prelude::{Commands, EventReader, EventWriter};

use crate::core::entity::{
    entity_data::{string_transform_to_transform, EntityData, RawSpawnEvent},
    entity_updates::EntityUpdates,
    spawn::{SpawnData, SpawnEvent},
};

use super::{
    process_content::{ExportData, ExportDataRaw},
    reflection_probe::ReflectionProbe,
};

pub struct ReflectionProbeSummoner {
    pub probe: ReflectionProbe,
}

pub trait ReflectionProbeSummonable {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands);
}

pub const REFLECTION_PROBE_ENTITY_NAME: &str = "reflection_probe";

impl ReflectionProbeSummonable for ReflectionProbeSummoner {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands) {
        commands.spawn_bundle((
            self.probe.clone(),
            spawn_data.entity_transform,
            EntityData {
                entity_class: REFLECTION_PROBE_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
        ));
    }
}

pub fn summon_reflection_probe<T: ReflectionProbeSummonable + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        spawn_event
            .summoner
            .spawn(&spawn_event.spawn_data, &mut commands);
    }
}

pub fn spawn_raw_reflection_probe(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_reflection_probe: EventWriter<SpawnEvent<ReflectionProbeSummoner>>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        if event.raw_entity.entity_type == "ReflectionProbe" {
            let reflection_probe_data_raw: ExportDataRaw =
                serde_json::from_str(&event.raw_entity.data)
                    .expect("load_raw_map_entities.rs Error parsing entity ReflectionProbe data.");
            let reflection_probe_component =
                ExportData::new(reflection_probe_data_raw).to_component();

            let entity_transform = string_transform_to_transform(&event.raw_entity.transform);

            summon_reflection_probe.send(SpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: entity_transform,
                    default_map_spawn: true,
                    entity_name: event.raw_entity.entity_type.clone(),
                    entity: commands.spawn().id(),
                    ..Default::default()
                },
                summoner: ReflectionProbeSummoner {
                    probe: reflection_probe_component,
                },
            });
        }
    }
}
