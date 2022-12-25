use std::collections::BTreeMap;

use super::line_arrow::{LineArrow, PointArrow};
use bevy::{
    prelude::{Commands, EventReader, Transform},
    time::Timer,
};
use entity::{
    entity_data::{WorldMode, WorldModes},
    entity_types::EntityType,
    examine::{Examinable, RichName},
    spawn::{BaseEntityBuilder, BaseEntityBundle, EntityBuildData, NoData, SpawnEntity},
};

#[cfg(any(feature = "server", feature = "client"))]
pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

#[cfg(any(feature = "server", feature = "client"))]
impl BaseEntityBuilder<NoData> for LineArrowType {
    fn get_bundle(&self, _spawn_data: &EntityBuildData, _entity_data: NoData) -> BaseEntityBundle {
        let template_examine_text =
            "A holographic arrow without additional data points.".to_string();
        let mut examine_map = BTreeMap::new();
        examine_map.insert(0, template_examine_text);

        BaseEntityBundle {
            default_transform: get_default_transform(),
            examinable: Examinable {
                assigned_texts: examine_map,
                name: RichName {
                    name: "arrow".to_string(),
                    n: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            entity_type: Box::new(LineArrowType::new()),
            ..Default::default()
        }
    }
}

#[cfg(any(feature = "server", feature = "client"))]
#[derive(Clone)]
pub struct LineArrowType {
    pub duration: f32,
    pub identifier: String,
}
impl Default for LineArrowType {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            identifier: SF_CONTENT_PREFIX.to_string() + "lineArrow",
        }
    }
}
impl EntityType for LineArrowType {
    fn to_string(&self) -> String {
        self.identifier.clone()
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        LineArrowType::default()
    }
    fn is_type(&self, identifier: String) -> bool {
        self.identifier == identifier
    }
}
#[cfg(any(feature = "server", feature = "client"))]
impl LinerArrowBuilder for LineArrowType {
    fn get_duration(&self) -> f32 {
        self.duration
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub trait LinerArrowBuilder: Send + Sync {
    fn get_duration(&self) -> f32;
}
use bevy::time::TimerMode;

#[cfg(any(feature = "server", feature = "client"))]
pub fn build_line_arrows<T: LinerArrowBuilder + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands.entity(spawn_event.spawn_data.entity).insert((
            spawn_event.spawn_data.entity_transform,
            LineArrow,
            PointArrow {
                timer: Timer::from_seconds(spawn_event.entity_type.get_duration(), TimerMode::Once),
            },
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}
use resources::content::SF_CONTENT_PREFIX;
