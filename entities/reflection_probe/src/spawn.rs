use bevy::prelude::{Commands, EventReader, EventWriter, Transform};
use entity::{
    entity_data::{EntityData, EntityUpdates, RawSpawnEvent},
    spawn::{SpawnData, SpawnEvent},
};

use crate::core::ReflectionProbe;

use super::process_content::{ExportData, ExportDataRaw};

#[cfg(feature = "server")]
pub struct ReflectionProbeSummoner {
    pub probe: ReflectionProbe,
}

#[cfg(feature = "server")]
pub trait ReflectionProbeSummonable {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands);
}

#[cfg(feature = "server")]
pub const REFLECTION_PROBE_ENTITY_NAME: &str = "reflection_probe";

#[cfg(feature = "server")]
impl ReflectionProbeSummonable for ReflectionProbeSummoner {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands) {
        commands.spawn((
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

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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

            let mut entity_transform = Transform::from_translation(event.raw_entity.translation);
            entity_transform.rotation = event.raw_entity.rotation;
            entity_transform.scale = event.raw_entity.scale;
            summon_reflection_probe.send(SpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: entity_transform,
                    default_map_spawn: true,
                    entity_name: event.raw_entity.entity_type.clone(),
                    entity: commands.spawn(()).id(),
                    ..Default::default()
                },
                summoner: ReflectionProbeSummoner {
                    probe: reflection_probe_component,
                },
            });
        }
    }
}
