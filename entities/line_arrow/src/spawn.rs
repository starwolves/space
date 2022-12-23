use std::collections::BTreeMap;

use super::line_arrow::{LineArrow, PointArrow};
use bevy::{
    prelude::{Commands, EventReader, EventWriter, Transform},
    time::Timer,
};
use entity::{
    entity_data::{WorldMode, WorldModes},
    examine::{Examinable, RichName},
    spawn::{
        BaseEntityBuildable, BaseEntityBundle, DefaultSpawnEvent, EntityBuildData, NoData,
        SpawnEntity,
    },
};

#[cfg(any(feature = "server", feature = "client"))]
pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

#[cfg(any(feature = "server", feature = "client"))]
impl BaseEntityBuildable<NoData> for LineArrowBuilder {
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
            entity_type: LINE_ARROW_ENTITY_NAME.to_string(),
            ..Default::default()
        }
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub struct LineArrowBuilder {
    pub duration: f32,
}

#[cfg(any(feature = "server", feature = "client"))]
impl LinerArrowBuildable for LineArrowBuilder {
    fn get_duration(&self) -> f32 {
        self.duration
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub trait LinerArrowBuildable {
    fn get_duration(&self) -> f32;
}
use bevy::time::TimerMode;

#[cfg(any(feature = "server", feature = "client"))]
pub fn build_line_arrows<T: LinerArrowBuildable + Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEntity<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands.entity(spawn_event.spawn_data.entity).insert((
            spawn_event.spawn_data.entity_transform,
            LineArrow,
            PointArrow {
                timer: Timer::from_seconds(spawn_event.builder.get_duration(), TimerMode::Once),
            },
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}
use resources::content::SF_CONTENT_PREFIX;
#[cfg(any(feature = "server", feature = "client"))]
pub const LINE_ARROW_ENTITY_NAME: &str = concatcp!(SF_CONTENT_PREFIX, "lineArrow");

#[cfg(any(feature = "server", feature = "client"))]
pub fn default_build_line_arrows(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEntity<LineArrowBuilder>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_type == LINE_ARROW_ENTITY_NAME {
            spawner.send(SpawnEntity {
                spawn_data: spawn_event.spawn_data.clone(),
                builder: LineArrowBuilder { duration: 6000.0 },
            });
        }
    }
}
use const_format::concatcp;
