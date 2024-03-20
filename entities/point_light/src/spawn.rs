use bevy::{
    ecs::system::Res,
    prelude::{Commands, EventReader, PointLight, PointLightBundle, Transform},
};
use entity::{
    entity_data::{EntityData, EntityGroup, WorldMode, WorldModes},
    entity_macros::Identity,
    entity_types::EntityType,
    sensable::Sensable,
    spawn::{EntityBuildData, SpawnEntity},
};
use graphics::settings::{PerformanceSettings, Shadows};
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
                ..Default::default()
            },
            identifier: SF_CONTENT_PREFIX.to_string() + "point_light",
        }
    }
}

pub fn build_point_lights<T: PointLightBuilder<PointLightBuildData> + 'static>(
    mut spawn_events: EventReader<SpawnEntity<T>>,
    mut commands: Commands,
    perf_settings: Res<PerformanceSettings>,
) {
    for spawn_event in spawn_events.read() {
        spawn_event.entity_type.spawn(
            &spawn_event.spawn_data,
            PointLightBuildData {
                shadows: perf_settings.shadows.clone(),
            },
            &mut commands,
        );
    }
}

pub trait PointLightBuilder<Y>: Send + Sync {
    fn spawn(&self, spawn_data: &EntityBuildData, entity_data_option: Y, commands: &mut Commands);
}

pub struct PointLightBuildData {
    pub shadows: Shadows,
}

impl PointLightBuilder<PointLightBuildData> for PointLightType {
    fn spawn(
        &self,
        spawn_data: &EntityBuildData,
        data: PointLightBuildData,
        commands: &mut Commands,
    ) {
        let mut light = self.light.clone();
        match data.shadows {
            Shadows::Off => {
                light.shadows_enabled = false;
            }
            Shadows::Medium => {
                light.shadows_enabled = false;
            }
            Shadows::High => {
                light.shadows_enabled = true;
            }
        }
        commands.spawn((
            PointLightBundle {
                point_light: light,
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
