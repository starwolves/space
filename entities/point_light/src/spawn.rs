use bevy::prelude::{Commands, EventReader, EventWriter, PointLight, PointLightBundle, Transform};
use entity::{
    entity_data::{EntityData, EntityUpdates, RawSpawnEvent, WorldMode, WorldModes},
    sensable::Sensable,
    spawn::{EntityBuildData, SpawnEntity},
};

#[cfg(any(feature = "server", feature = "client"))]
pub struct PointLightSummonBundle;

#[cfg(any(feature = "server", feature = "client"))]
pub const POINT_LIGHT_ENTITY_NAME: &str = "point_light";

#[cfg(any(feature = "server", feature = "client"))]
impl PointLightSummonBundle {
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
                entity_class: POINT_LIGHT_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub struct PointLightBuilder {
    pub light: PointLight,
}

#[cfg(any(feature = "server", feature = "client"))]
pub fn build_point_lights<T: PointLightBuildable + Send + Sync + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        spawn_event
            .summoner
            .spawn(&spawn_event.spawn_data, &mut commands);
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub trait PointLightBuildable {
    fn spawn(&self, spawn_data: &EntityBuildData, commands: &mut Commands);
}

#[cfg(any(feature = "server", feature = "client"))]
impl PointLightBuildable for PointLightBuilder {
    fn spawn(&self, spawn_data: &EntityBuildData, commands: &mut Commands) {
        commands.spawn((
            PointLightBundle {
                point_light: self.light.clone(),
                transform: spawn_data.entity_transform,
                ..Default::default()
            },
            Sensable {
                is_light: true,
                ..Default::default()
            },
            EntityData {
                entity_class: POINT_LIGHT_ENTITY_NAME.to_string(),
                ..Default::default()
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub fn build_raw_point_lights(
    mut spawn_events: EventReader<RawSpawnEvent>,
    mut summon_point_light: EventWriter<SpawnEntity<PointLightBuilder>>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        if event.raw_entity.entity_type == "OmniLight" {
            let mut entity_transform = Transform::from_translation(event.raw_entity.translation);
            entity_transform.rotation = event.raw_entity.rotation;
            entity_transform.scale = event.raw_entity.scale;

            summon_point_light.send(SpawnEntity {
                spawn_data: EntityBuildData {
                    entity_transform: entity_transform,
                    default_map_spawn: true,
                    entity_name: event.raw_entity.entity_type.clone(),
                    entity: commands.spawn(()).id(),
                    ..Default::default()
                },
                summoner: PointLightBuilder {
                    light: get_default_point_light(),
                },
            });
        }
    }
}

pub fn get_default_point_light() -> PointLight {
    PointLight {
        intensity: 1500.,
        range: 10.,
        radius: 5.,
        shadows_enabled: true,
        ..Default::default()
    }
}
