use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Transform},
    time::Time,
};
use entity::spawn::{EntityBuildData, SpawnEntity};

use super::{line_arrow::PointArrow, spawn::LineArrowType};
use console_commands::commands::InputConsoleCommand;

/// Perform "pointArrow" command.

pub(crate) fn entity_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut commands: Commands,
    mut spawn_event: EventWriter<SpawnEntity<LineArrowType>>,
) {
    for command in queue.read() {
        if command.input.command == "pointArrow" && command.input.args.len() == 4 {
            let x;
            let y;
            let z;

            match command.input.args[0].parse::<f32>() {
                Ok(v) => x = v,
                Err(_) => continue,
            }

            match command.input.args[1].parse::<f32>() {
                Ok(v) => y = v,
                Err(_) => continue,
            }

            match command.input.args[2].parse::<f32>() {
                Ok(v) => z = v,
                Err(_) => continue,
            }

            let duration;

            match command.input.args[3].parse::<i64>() {
                Ok(v) => duration = v,
                Err(_) => continue,
            }

            let translation = Vec3::new(x, y, z);

            let mut passed_transform = Transform::IDENTITY;
            passed_transform.translation = translation;

            spawn_event.send(SpawnEntity {
                spawn_data: EntityBuildData {
                    entity_transform: passed_transform,
                    correct_transform: false,
                    entity: commands.spawn(()).id(),
                    ..Default::default()
                },
                entity_type: LineArrowType {
                    duration: duration as f32,
                    ..Default::default()
                },
            });
        }
    }
}
use entity::spawning_events::DespawnClientEntity;

/// Despawn point arrows after duration.

pub(crate) fn expire_point_arrow(
    mut point_arrows: Query<(Entity, &mut PointArrow)>,
    time: Res<Time>,
    mut net_unload_entity: EventWriter<DespawnClientEntity>,
) {
    for (entity, mut point_arrow_component) in point_arrows.iter_mut() {
        if point_arrow_component
            .timer
            .tick(time.delta())
            .just_finished()
        {
            net_unload_entity.send(DespawnClientEntity { entity });
        }
    }
}
