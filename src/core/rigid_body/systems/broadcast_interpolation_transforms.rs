use bevy_core::Time;
use bevy_ecs::{
    entity::Entity,
    prelude::Without,
    system::{Local, Query, Res, ResMut},
};
use bevy_log::warn;
use bevy_math::{Quat, Vec3};
use bevy_networking_turbulence::NetworkResource;
use bevy_rapier3d::prelude::{RigidBodyPositionComponent, RigidBodyVelocityComponent};
use bevy_transform::components::Transform;

use crate::core::{
    connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
    networking::resources::UnreliableServerMessage,
    rigid_body::components::{CachedBroadcastTransform, RigidBodyDisabled},
    sensable::components::Sensable,
    static_body::components::StaticTransform,
};

#[derive(Debug)]
pub enum InterpolationPriorityRates {
    T4,
    T8,
    T12,
    T24,
}

#[derive(Default)]
pub struct InterpolationFrame {
    pub i: u8,
}

pub const BROADCAST_INTERPOLATION_TRANSFORM_RATE: f64 = 24.;

pub fn broadcast_interpolation_transforms(
    time: Res<Time>,

    mut net: ResMut<NetworkResource>,
    handle_to_entity: Res<HandleToEntity>,
    mut query_interpolated_entities: Query<
        (
            Entity,
            &Sensable,
            &RigidBodyPositionComponent,
            &RigidBodyVelocityComponent,
            &mut CachedBroadcastTransform,
            Option<&ConnectedPlayer>,
        ),
        (Without<StaticTransform>, Without<RigidBodyDisabled>),
    >,
    mut interpolation_frame: Local<InterpolationFrame>,
) {
    interpolation_frame.i += 1;

    if interpolation_frame.i > 24 {
        interpolation_frame.i = 1;
    }

    let current_time_stamp = time.time_since_startup().as_millis();

    for (
        interpolated_entity,
        visible_component,
        rigid_body_position_component,
        rigid_body_velocity_component,
        mut cached_transform_component,
        connected_player_component_option,
    ) in query_interpolated_entities.iter_mut()
    {
        let rigid_body_position = rigid_body_position_component.position;

        let rigid_body_translation_rapier = rigid_body_position.translation;
        let rigid_body_velocity_rapier = rigid_body_velocity_component.linvel;
        let rigid_body_rotation_rapier = rigid_body_position.rotation;

        let rigid_body_translation = Vec3::new(
            rigid_body_translation_rapier.x,
            rigid_body_translation_rapier.y,
            rigid_body_translation_rapier.z,
        );

        let rigid_body_velocity = Vec3::new(
            rigid_body_velocity_rapier.x,
            rigid_body_velocity_rapier.y,
            rigid_body_velocity_rapier.z,
        );
        let rigid_body_rotation = Quat::from_xyzw(
            rigid_body_rotation_rapier.i,
            rigid_body_rotation_rapier.j,
            rigid_body_rotation_rapier.k,
            rigid_body_rotation_rapier.w,
        );

        let this_transform = Transform {
            translation: rigid_body_translation,
            rotation: rigid_body_rotation,
            scale: Vec3::ONE,
        };

        if this_transform == cached_transform_component.transform {
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
                    match net.send_message(
                        *handle,
                        UnreliableServerMessage::TransformUpdate(
                            interpolated_entity.to_bits(),
                            rigid_body_translation,
                            rigid_body_rotation,
                            velocity_option,
                            current_time_stamp as u64,
                            rate_u,
                        ),
                    ) {
                        Ok(msg) => match msg {
                            Some(msg) => {
                                warn!("was unable to send TransformUpdate message: {:?}", msg);
                            }
                            None => {}
                        },
                        Err(err) => {
                            warn!("was unable to send TransformUpdate message (1): {:?}", err);
                        }
                    };
                }
                None => {
                    continue;
                }
            }
        }
    }
}

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
