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
    entity::{events::NetUnloadEntity, resources::SpawnData, spawn::SpawnEvent},
    networking::resources::ConsoleCommandVariantValues,
    sensable::components::Sensable,
};

use super::{
    components::PointArrow,
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
