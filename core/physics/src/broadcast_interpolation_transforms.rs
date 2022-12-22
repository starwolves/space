use bevy::{
    math::Vec3,
    prelude::{Entity, Local, Query, Res, Transform, With, Without},
    time::Time,
};
use networking::server::UnreliableServerMessage;

/// All transform interpolation rates.
#[derive(Debug)]
#[allow(dead_code)]
#[cfg(feature = "server")]
enum InterpolationPriorityRates {
    T4,
    T8,
    T12,
    T24,
}

/// Resource with interpolation frame.
#[derive(Default)]
#[cfg(feature = "server")]
pub(crate) struct InterpolationFrame {
    pub i: u8,
}
use bevy_rapier3d::prelude::Velocity;
use networking::server::HandleToEntity;

use crate::physics::RigidBodyDisabled;
use bevy::prelude::EventWriter;
use entity::entity_data::CachedBroadcastTransform;
use entity::sensable::Sensable;
use networking::server::OutgoingUnreliableServerMessage;

use bevy_rapier3d::prelude::RigidBody;
use networking::server::ConnectedPlayer;
/// Broadcast transforms.
#[cfg(feature = "server")]
pub(crate) fn broadcast_interpolation_transforms(
    time: Res<Time>,

    mut net: EventWriter<OutgoingUnreliableServerMessage<UnreliableServerMessage>>,
    handle_to_entity: Res<HandleToEntity>,
    mut query_interpolated_entities: Query<
        (
            Entity,
            &Sensable,
            &Transform,
            &Velocity,
            &mut CachedBroadcastTransform,
            Option<&ConnectedPlayer>,
        ),
        (With<RigidBody>, Without<RigidBodyDisabled>),
    >,
    mut interpolation_frame: Local<InterpolationFrame>,
) {
    interpolation_frame.i += 1;

    if interpolation_frame.i > 24 {
        interpolation_frame.i = 1;
    }

    let current_time_stamp = time.elapsed().as_millis();

    for (
        interpolated_entity,
        visible_component,
        rigid_body_position_component,
        rigid_body_velocity_component,
        mut cached_transform_component,
        connected_player_component_option,
    ) in query_interpolated_entities.iter_mut()
    {
        let rigid_body_position = rigid_body_position_component;

        let rigid_body_translation = rigid_body_position.translation;
        let rigid_body_velocity = rigid_body_velocity_component.linvel;
        let rigid_body_rotation = rigid_body_position.rotation;

        let this_transform = Transform {
            translation: rigid_body_translation,
            rotation: rigid_body_rotation,
            scale: Vec3::ONE,
        };

        if this_transform == cached_transform_component.transform
            || this_transform
                .translation
                .distance(cached_transform_component.transform.translation)
                < 0.01
        {
            cached_transform_component.is_active = false;
            continue;
        }
        cached_transform_component.is_active = true;

        cached_transform_component.transform = this_transform;

        for sensed_by_entity in visible_component.sensed_by.iter() {
            let player_handle_option = handle_to_entity.inv_map.get(&sensed_by_entity);

            if player_handle_option.is_none() {
                continue;
            }

            let mut entity_tick_rate = &InterpolationPriorityRates::T4;

            if *sensed_by_entity == interpolated_entity {
                entity_tick_rate = &InterpolationPriorityRates::T24;
            } else if connected_player_component_option.is_some() {
                entity_tick_rate = &InterpolationPriorityRates::T12;
            }

            if !is_interpolation_frame(&entity_tick_rate, interpolation_frame.i) {
                continue;
            }

            let rate_u: u8;
            let send_vel;

            match entity_tick_rate {
                InterpolationPriorityRates::T4 => {
                    rate_u = 4;
                    //send_vel = true;
                    send_vel = false;
                }
                InterpolationPriorityRates::T8 => {
                    rate_u = 8;
                    //send_vel = true;
                    send_vel = false;
                }
                InterpolationPriorityRates::T12 => {
                    rate_u = 12;
                    send_vel = false;
                }
                InterpolationPriorityRates::T24 => {
                    rate_u = 24;
                    send_vel = false;
                }
            }

            let velocity_option;

            if send_vel {
                velocity_option = Some(rigid_body_velocity);
            } else {
                velocity_option = None;
            }

            match player_handle_option {
                Some(handle) => {
                    net.send(OutgoingUnreliableServerMessage {
                        handle: *handle,
                        message: UnreliableServerMessage::TransformUpdate(
                            interpolated_entity.to_bits(),
                            rigid_body_translation,
                            rigid_body_rotation,
                            velocity_option,
                            current_time_stamp as u64,
                            rate_u,
                        ),
                    });
                }
                None => {
                    continue;
                }
            }
        }
    }
}

/// Check if this is the frame for interpolation as a function.
#[cfg(feature = "server")]
fn is_interpolation_frame(
    entity_tick_rate: &InterpolationPriorityRates,
    current_frame: u8,
) -> bool {
    match entity_tick_rate {
        InterpolationPriorityRates::T4 => {
            if current_frame % 6 == 0 {
                true
            } else {
                false
            }
        }
        InterpolationPriorityRates::T8 => {
            if current_frame % 3 == 0 {
                true
            } else {
                false
            }
        }
        InterpolationPriorityRates::T12 => {
            if current_frame % 2 == 0 {
                true
            } else {
                false
            }
        }
        InterpolationPriorityRates::T24 => true,
    }
}
