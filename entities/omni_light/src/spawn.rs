use bevy::prelude::{Commands, EventReader, EventWriter, Transform};
use entity::{
    entity_data::RawSpawnEvent,
    spawn::{SpawnData, SpawnEvent},
};
use physics::world_mode::{WorldMode, WorldModes};
use api::{
    converters::string_transform_to_transform,
    entity_updates::{EntityData, EntityUpdates},
    sensable::Sensable,
};

use super::{
    omni_light::OmniLight,
    process_content::{ExportData, ExportDataRaw},
};

pub struct OmniLightBundle;

pub const OMNI_LIGHT_ENTITY_NAME: &str = "omni_light";

impl OmniLightBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        omni_light_component: OmniLight,
    ) {
        let static_transform_component = entity_transform;

        commands.spawn_bundle((
            omni_light_component,
            Sensable {
                is_light: true,
                ..Default::default()
            },
            static_transform_component,
            EntityData {
                entity_class: OMNI_LIGHT_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

pub struct OmniLightSummoner {
    pub light: OmniLight,
}

pub fn summon_omni_light<T: OmniLightSummonable + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        spawn_event
            .summoner
            .spawn(&spawn_event.spawn_data, &mut commands);
    }
}

pub trait OmniLightSummonable {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands);
}

impl OmniLightSummonable for OmniLightSummoner {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands) {
        commands.spawn_bundle((
            self.light.clone(),
            Sensable {
                is_light: true,
                ..Default::default()
            },
            spawn_data.entity_transform,
            EntityData {
                entity_class: OMNI_LIGHT_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

pub fn summon_raw_omni_light(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_gi_probe: EventWriter<SpawnEvent<OmniLightSummoner>>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        if event.raw_entity.entity_type == "OmniLight" {
            let omni_light_data_raw: ExportDataRaw = serde_json::from_str(&event.raw_entity.data)
                .expect("load_raw_map_entities.rs Error parsing entity OmniLight data.");
            let omni_light_component = ExportData::new(omni_light_data_raw).to_component();

            let entity_transform = string_transform_to_transform(&event.raw_entity.transform);

            summon_gi_probe.send(SpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: entity_transform,
                    default_map_spawn: true,
                    entity_name: event.raw_entity.entity_type.clone(),
                    entity: commands.spawn().id(),
                    ..Default::default()
                },
                summoner: OmniLightSummoner {
                    light: omni_light_component,
                },
            });
        }
    }
}
