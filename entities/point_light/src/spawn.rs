use bevy::prelude::{Commands, EventReader, PointLight, PointLightBundle, Transform};
use entity::{
    entity_data::{EntityData, EntityGroup, WorldMode, WorldModes},
    entity_macros::Identity,
    entity_types::EntityType,
    sensable::Sensable,
    spawn::{EntityBuildData, SpawnEntity},
};
use resources::core::SF_CONTENT_PREFIX;

pub struct PointLightBuilderBundle;

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
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

#[derive(Clone, Identity)]
pub struct PointLightType {
    pub light: PointLight,
    pub identifier: String,
}
impl Default for PointLightType {
    fn default() -> Self {
        Self {
            light: PointLight {
                shadows_enabled: true,
                intensity: 1800.,
                ..Default::default()
            },
            identifier: SF_CONTENT_PREFIX.to_string() + "point_light",
        }
    }
}

pub fn build_point_lights<T: PointLightBuilder + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
) {
    for spawn_event in spawn_events.read() {
        spawn_event
            .entity_type
            .spawn(&spawn_event.spawn_data, &mut commands);
    }
}

pub trait PointLightBuilder: Send + Sync {
    fn spawn(&self, spawn_data: &EntityBuildData, commands: &mut Commands);
}

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
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

pub fn get_default_point_light() -> PointLight {
    PointLight {
        intensity: 1.,
        range: 16.,
        shadows_enabled: true,
        ..Default::default()
    }
}
