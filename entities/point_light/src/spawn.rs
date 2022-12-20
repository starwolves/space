use bevy::prelude::{Commands, EventReader, EventWriter, PointLight, Transform};
use entity::{
    entity_data::{EntityData, EntityUpdates, RawSpawnEvent, WorldMode, WorldModes},
    sensable::Sensable,
    spawn::{SpawnData, SpawnEvent},
};

#[cfg(feature = "client")]
pub struct PointLightBundle;

#[cfg(feature = "client")]
pub const OMNI_LIGHT_ENTITY_NAME: &str = "omni_light";

#[cfg(feature = "client")]
impl PointLightBundle {
    pub fn spawn(
        entity_transform: Transform,
        commands: &mut Commands,
        _correct_transform: bool,
        point_light_component: PointLight,
    ) {
        let static_transform_component = entity_transform;

        commands.spawn((
            point_light_component,
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

#[cfg(feature = "client")]
pub struct PointLightSummoner {
    pub light: PointLight,
}

#[cfg(feature = "client")]
pub fn summon_point_light<T: OmniLightSummonable + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEvent<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        spawn_event
            .summoner
            .spawn(&spawn_event.spawn_data, &mut commands);
    }
}

#[cfg(feature = "client")]
pub trait OmniLightSummonable {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands);
}

#[cfg(feature = "client")]
impl OmniLightSummonable for PointLightSummoner {
    fn spawn(&self, spawn_data: &SpawnData, commands: &mut Commands) {
        commands.spawn((
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

#[cfg(feature = "client")]
pub fn summon_raw_point_light(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_point_light: EventWriter<SpawnEvent<PointLightSummoner>>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        if event.raw_entity.entity_type == "OmniLight" {
            let mut entity_transform = Transform::from_translation(event.raw_entity.translation);
            entity_transform.rotation = event.raw_entity.rotation;
            entity_transform.scale = event.raw_entity.scale;

            summon_point_light.send(SpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: entity_transform,
                    default_map_spawn: true,
                    entity_name: event.raw_entity.entity_type.clone(),
                    entity: commands.spawn(()).id(),
                    ..Default::default()
                },
                summoner: PointLightSummoner {
                    light: PointLight {
                        intensity: 1500.,
                        range: 10.,
                        radius: 5.,
                        shadows_enabled: true,
                        ..Default::default()
                    },
                },
            });
        }
    }
}
