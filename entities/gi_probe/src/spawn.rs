use super::process_content::ExportData;

pub struct GIProbeSummoner {
    pub probe: GIProbe,
}

pub trait GIProbeSummonable {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands);
}

impl GIProbeSummonable for GIProbeSummoner {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands) {
        commands.spawn_bundle((
            self.probe.clone(),
            spawn_data.entity_transform,
            EntityData {
                entity_class: "gi_probe".to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
        ));
    }
}

pub fn summon_gi_probe<T: GIProbeSummonable + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        spawn_event
            .summoner
            .spawn(&spawn_event.spawn_data, &mut commands);
    }
}

pub const GI_PROBE_ENTITY_NAME: &str = "GIProbe";

pub fn summon_raw_gi_probe(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_gi_probe: EventWriter<SpawnEvent<GIProbeSummoner>>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        if event.raw_entity.entity_type == GI_PROBE_ENTITY_NAME {
            let gi_probe_data: ExportData = serde_json::from_str(&event.raw_entity.data)
                .expect("load_raw_map_entities.rs Error parsing entity raw GIProbe data.");
            let gi_probe_component = gi_probe_data.to_component();

            let entity_transform = string_transform_to_transform(&event.raw_entity.transform);

            summon_gi_probe.send(SpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: entity_transform,
                    default_map_spawn: true,
                    entity_name: event.raw_entity.entity_type.clone(),
                    entity: commands.spawn().id(),
                    ..Default::default()
                },
                summoner: GIProbeSummoner {
                    probe: gi_probe_component,
                },
            });
        }
    }
}
use api::{
    converters::string_transform_to_transform,
    data::GIProbe,
    entity_updates::{EntityData, EntityUpdates},
};
use bevy::prelude::{Commands, EventReader, EventWriter};
use entity::{
    entity_data::RawSpawnEvent,
    spawn::{SpawnData, SpawnEvent},
};
