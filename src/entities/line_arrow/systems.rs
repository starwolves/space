use std::collections::HashMap;

use bevy_core::Time;
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, Query, Res},
};
use bevy_math::Vec3;
use bevy_transform::components::Transform;

use crate::core::{
    connected_player::resources::HandleToEntity,
    console_commands::events::InputConsoleCommand,
    entity::{events::NetUnloadEntity, resources::SpawnData},
    networking::resources::ConsoleCommandVariantValues,
    sensable::components::Sensable,
};

use super::{components::PointArrow, spawn::LineArrowBundle};

pub fn entity_console_commands(
    mut queue: EventReader<InputConsoleCommand>,
    mut commands: Commands,
) {
    for command in queue.iter() {
        if command.command_name == "pointArrow" {
            let x;
            let y;
            let z;

            match command.command_arguments.get(0) {
                Some(variant_val) => match variant_val {
                    crate::core::networking::resources::ConsoleCommandVariantValues::Int(_) => {
                        continue;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::String(_) => {
                        continue;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::Float(val) => {
                        x = *val;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            match command.command_arguments.get(1) {
                Some(variant_val) => match variant_val {
                    crate::core::networking::resources::ConsoleCommandVariantValues::Int(_) => {
                        continue;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::String(_) => {
                        continue;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::Float(val) => {
                        y = *val;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::Bool(_) => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            }

            match command.command_arguments.get(2) {
                Some(variant_val) => match variant_val {
                    crate::core::networking::resources::ConsoleCommandVariantValues::Int(_) => {
                        continue;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::String(_) => {
                        continue;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::Float(val) => {
                        z = *val;
                    }
                    crate::core::networking::resources::ConsoleCommandVariantValues::Bool(_) => {
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

            let mut properties = HashMap::new();
            properties.insert(
                "duration".to_string(),
                ConsoleCommandVariantValues::Int(duration),
            );

            let translation = Vec3::new(x, y, z);

            let mut passed_transform = Transform::identity();
            passed_transform.translation = translation;

            LineArrowBundle::spawn(SpawnData {
                entity_transform: passed_transform,
                commands: &mut commands,
                correct_transform: false,
                pawn_data_option: None,
                held_data_option: None,
                default_map_spawn: false,
                properties,
                showcase_data_option: &mut None,
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
