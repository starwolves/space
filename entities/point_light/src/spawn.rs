use bevy::prelude::{Commands, EventReader, EventWriter, PointLight, PointLightBundle, Transform};
use entity::{
    entity_data::{EntityData, EntityGroup, EntityUpdates, RawSpawnEvent, WorldMode, WorldModes},
    entity_types::{BoxedEntityType, EntityType},
    sensable::Sensable,
    spawn::{EntityBuildData, SpawnEntity},
};
use resources::content::SF_CONTENT_PREFIX;

#[cfg(any(feature = "server", feature = "client"))]
pub struct PointLightBuilderBundle;

#[cfg(any(feature = "server", feature = "client"))]
impl PointLightBuilderBundle {
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
                entity_type: Box::new(PointLightType::new()),
                entity_group: EntityGroup::default(),
            },
            EntityUpdates::default(),
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

#[cfg(any(feature = "server", feature = "client"))]
#[derive(Clone)]
pub struct PointLightType {
    pub light: PointLight,
    pub identifier: String,
}
impl Default for PointLightType {
    fn default() -> Self {
        Self {
            light: Default::default(),
            identifier: SF_CONTENT_PREFIX.to_string() + "PointLight",
        }
    }
}
impl EntityType for PointLightType {
    fn to_string(&self) -> String {
        self.identifier.clone()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        PointLightType::default()
    }
    fn is_type(&self, other_type: BoxedEntityType) -> bool {
        other_type.to_string() == self.identifier
    }
}
#[cfg(any(feature = "server", feature = "client"))]
pub fn build_point_lights<T: PointLightBuilder + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.iter() {
        spawn_event
            .builder
            .spawn(&spawn_event.spawn_data, &mut commands);
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub trait PointLightBuilder: Send + Sync {
    fn spawn(&self, spawn_data: &EntityBuildData, commands: &mut Commands);
}

#[cfg(any(feature = "server", feature = "client"))]
impl PointLightBuilder for PointLightType {
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
                entity_type: Box::new(PointLightType::new()),
                entity_group: EntityGroup::default(),
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
    mut build_point_light: EventWriter<SpawnEntity<PointLightType>>,
    mut commands: Commands,
) {
    for event in spawn_events.iter() {
        if event.raw_entity.entity_type == PointLightType::new().to_string() {
            let mut entity_transform = Transform::from_translation(event.raw_entity.translation);
            entity_transform.rotation = event.raw_entity.rotation;
            entity_transform.scale = event.raw_entity.scale;
            build_point_light.send(SpawnEntity {
                spawn_data: EntityBuildData {
                    entity_transform: entity_transform,
                    default_map_spawn: true,
                    entity: commands.spawn(()).id(),
                    ..Default::default()
                },
                builder: PointLightType {
                    light: get_default_point_light(),
                    ..Default::default()
                },
            });
        }
    }
}

pub fn get_default_point_light() -> PointLight {
    PointLight {
        intensity: 1500.,
        range: 10.,
        shadows_enabled: true,
        ..Default::default()
    }
}
