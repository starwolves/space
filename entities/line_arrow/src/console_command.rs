use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Transform},
    time::Time,
};
use entity::spawn::{EntityBuildData, SpawnEntity};
use networking::server::GodotVariantValues;

use super::{
    line_arrow::PointArrow,
    spawn::{LineArrowBuilder, LINE_ARROW_ENTITY_NAME},
};
use console_commands::commands::InputConsoleCommand;

/// Perform "pointArrow" command.
#[cfg(feature = "server")]
pub(crate) fn entity_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut commands: Commands,
    mut spawn_event: EventWriter<SpawnEntity<LineArrowBuilder>>,
) {
    for command in queue.iter() {
        if command.command_name == "pointArrow" {
            let x;
            let y;
            let z;

            match command.command_arguments.get(0) {
                Some(variant_val) => match variant_val {
                    GodotVariantValues::Int(_) => {
                        continue;
                    }
                    GodotVariantValues::String(_) => {
                        continue;
                    }
                    GodotVariantValues::Float(val) => {
                        x = *val;
                    }
                    GodotVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            match command.command_arguments.get(1) {
                Some(variant_val) => match variant_val {
                    GodotVariantValues::Int(_) => {
                        continue;
                    }
                    GodotVariantValues::String(_) => {
                        continue;
                    }
                    GodotVariantValues::Float(val) => {
                        y = *val;
                    }
                    GodotVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            match command.command_arguments.get(2) {
                Some(variant_val) => match variant_val {
                    GodotVariantValues::Int(_) => {
                        continue;
                    }
                    GodotVariantValues::String(_) => {
                        continue;
                    }
                    GodotVariantValues::Float(val) => {
                        z = *val;
                    }
                    GodotVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            let duration;

            match command.command_arguments.get(3) {
                Some(variant_val) => match variant_val {
                    GodotVariantValues::Int(val) => {
                        duration = *val;
                    }
                    GodotVariantValues::String(_) => {
                        continue;
                    }
                    GodotVariantValues::Float(_) => {
                        continue;
                    }
                    GodotVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            let translation = Vec3::new(x, y, z);

            let mut passed_transform = Transform::IDENTITY;
            passed_transform.translation = translation;

            spawn_event.send(SpawnEntity {
                spawn_data: EntityBuildData {
                    entity_transform: passed_transform,
                    correct_transform: false,
                    entity_name: LINE_ARROW_ENTITY_NAME.to_string(),
                    entity: commands.spawn(()).id(),
                    ..Default::default()
                },
                builder: LineArrowBuilder {
                    duration: duration as f32,
                },
            });
        }
    }
}
use entity::spawning_events::DespawnClientEntity;

/// Despawn point arrows after duration.
#[cfg(feature = "server")]
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
