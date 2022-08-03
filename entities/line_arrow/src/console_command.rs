use api::{
    console_commands::ConsoleCommandVariantValues, data::HandleToEntity,
    load_entity::NetUnloadEntity, sensable::Sensable,
};
use bevy::{
    math::Vec3,
    prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, Transform},
    time::Time,
};
use entity::spawn::{SpawnData, SpawnEvent};
use networking::messages::InputConsoleCommand;

use super::{
    line_arrow::PointArrow,
    spawn::{LineArrowSummoner, LINE_ARROW_ENTITY_NAME},
};

pub fn entity_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut commands: Commands,
    mut spawn_event: EventWriter<SpawnEvent<LineArrowSummoner>>,
) {
    for command in queue.iter() {
        if command.command_name == "pointArrow" {
            let x;
            let y;
            let z;

            match command.command_arguments.get(0) {
                Some(variant_val) => match variant_val {
                    ConsoleCommandVariantValues::Int(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::String(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::Float(val) => {
                        x = *val;
                    }
                    ConsoleCommandVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            match command.command_arguments.get(1) {
                Some(variant_val) => match variant_val {
                    ConsoleCommandVariantValues::Int(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::String(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::Float(val) => {
                        y = *val;
                    }
                    ConsoleCommandVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            match command.command_arguments.get(2) {
                Some(variant_val) => match variant_val {
                    ConsoleCommandVariantValues::Int(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::String(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::Float(val) => {
                        z = *val;
                    }
                    ConsoleCommandVariantValues::Bool(_) => {
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
                    ConsoleCommandVariantValues::Int(val) => {
                        duration = *val;
                    }
                    ConsoleCommandVariantValues::String(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::Float(_) => {
                        continue;
                    }
                    ConsoleCommandVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            let translation = Vec3::new(x, y, z);

            let mut passed_transform = Transform::identity();
            passed_transform.translation = translation;

            spawn_event.send(SpawnEvent {
                spawn_data: SpawnData {
                    entity_transform: passed_transform,
                    correct_transform: false,
                    entity_name: LINE_ARROW_ENTITY_NAME.to_string(),
                    entity: commands.spawn().id(),
                    ..Default::default()
                },
                summoner: LineArrowSummoner {
                    duration: duration as f32,
                },
            });
        }
    }
}

pub fn point_arrow(
    mut point_arrows: Query<(Entity, &mut PointArrow, &mut Sensable)>,
    time: Res<Time>,
    mut net_unload_entity: EventWriter<NetUnloadEntity>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands: Commands,
) {
    for (entity, mut point_arrow_component, mut sensable_component) in point_arrows.iter_mut() {
        if point_arrow_component
            .timer
            .tick(time.delta())
            .just_finished()
        {
            sensable_component.despawn(entity, &mut net_unload_entity, &handle_to_entity);
            commands.entity(entity).despawn();
        }
    }
}
