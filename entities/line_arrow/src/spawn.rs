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
        BaseEntityBundle, BaseEntitySummonable, DefaultSpawnEvent, NoData, SpawnData, SpawnEvent,
    },
};

#[cfg(any(feature = "server", feature = "client"))]
pub fn get_default_transform() -> Transform {
    Transform::IDENTITY
}

#[cfg(any(feature = "server", feature = "client"))]
impl BaseEntitySummonable<NoData> for LineArrowSummoner {
    fn get_bundle(&self, _spawn_data: &SpawnData, _entity_data: NoData) -> BaseEntityBundle {
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
            entity_name: LINE_ARROW_ENTITY_NAME.to_string(),
            ..Default::default()
        }
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub struct LineArrowSummoner {
    pub duration: f32,
}

#[cfg(any(feature = "server", feature = "client"))]
impl LinerArrowSummonable for LineArrowSummoner {
    fn get_duration(&self) -> f32 {
        self.duration
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub trait LinerArrowSummonable {
    fn get_duration(&self) -> f32;
}
use bevy::time::TimerMode;

#[cfg(any(feature = "server", feature = "client"))]
pub fn summon_line_arrow<T: LinerArrowSummonable + Send + Sync + 'static>(
    mut commands: Commands,
    mut spawn_events: EventReader<SpawnEvent<T>>,
) {
    for spawn_event in spawn_events.iter() {
        commands.entity(spawn_event.spawn_data.entity).insert((
            spawn_event.spawn_data.entity_transform,
            LineArrow,
            PointArrow {
                timer: Timer::from_seconds(spawn_event.summoner.get_duration(), TimerMode::Once),
            },
            WorldMode {
                mode: WorldModes::Static,
            },
        ));
    }
}

#[cfg(any(feature = "server", feature = "client"))]
pub const LINE_ARROW_ENTITY_NAME: &str = "lineArrow";

#[cfg(any(feature = "server", feature = "client"))]
pub fn default_line_arrow(
    mut default_spawner: EventReader<DefaultSpawnEvent>,
    mut spawner: EventWriter<SpawnEvent<LineArrowSummoner>>,
) {
    for spawn_event in default_spawner.iter() {
        if spawn_event.spawn_data.entity_name == LINE_ARROW_ENTITY_NAME {
            spawner.send(SpawnEvent {
                spawn_data: spawn_event.spawn_data.clone(),
                summoner: LineArrowSummoner { duration: 6000.0 },
            });
        }
    }
}
