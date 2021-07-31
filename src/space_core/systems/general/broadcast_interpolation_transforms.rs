use std::collections::HashMap;

use bevy::{core::{FixedTimesteps, Time}, math::{Quat, Vec3}, prelude::{Entity, Local, Query, Res, ResMut, Transform, Without, info, warn}};
use bevy_networking_turbulence::NetworkResource;
use bevy_rapier3d::{prelude::{RigidBodyPosition, RigidBodyVelocity}};

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, interpolation_priority::InterpolationPriority, rigidbody_disabled::RigidBodyDisabled, sensable::Sensable, senser::Senser, static_transform::StaticTransform}, resources::{handle_to_entity::HandleToEntity, network_messages::UnreliableServerMessage}};

const INTERPOLATION_LABEL: &str = "fixed_timestep_interpolation";

pub enum InterpolationPriorityRates {
    T4,
    T8,
    T12,
    T24,
}

#[derive(Default)]
pub struct InterpolationFrame {
    pub i : u8,
}

pub const BROADCAST_INTERPOLATION_TRANSFORM_RATE : f64 = 24.;

pub fn broadcast_interpolation_transforms (
    time: Res<Time>, 
    fixed_timesteps: Res<FixedTimesteps>,
    
    mut net: ResMut<NetworkResource>,
    handle_to_entity : Res<HandleToEntity>,
    mut query_interpolated_entities : Query<(
        Entity,
        &Sensable,
        &RigidBodyPosition,
        &RigidBodyVelocity,
        &mut CachedBroadcastTransform,
        &InterpolationPriority,
    ),
    (
        Without<StaticTransform>,
        Without<RigidBodyDisabled>
    )>,
    query_sensers : Query<(Entity, &Senser)>,
    mut interpolation_frame : Local<InterpolationFrame>,
) {
    
    interpolation_frame.i +=1;

    if interpolation_frame.i > 24 {
        interpolation_frame.i = 1;
    }

    let current_time_stamp = time.time_since_startup().as_millis();

    let fixed_timestep = fixed_timesteps.get(INTERPOLATION_LABEL).unwrap().overstep_percentage();
    if fixed_timestep > 5. && current_time_stamp > 60000 {
        warn!(
            "overstep_percentage: {}",
            fixed_timestep
        );
    }

    let mut senser_priority_rates = HashMap::new();

    for (entity, senser) in query_sensers.iter() {


        let mut low_priority_count : u16 = 0;
        let mut medium_priority_count : u16 = 0;
        let mut high_priority_count : u16 = 0;

        //info!("{:?}", senser.sensing);

        for sensed_entity in senser.sensing.iter() {

            let sensed_entity_components;

            match query_interpolated_entities.get_mut(*sensed_entity) {
                Ok(components) => {
                    sensed_entity_components = components;
                },
                Err(_rr) => {
                    warn!("Couldn't find components for entity in senser.sensed.");
                    continue;
                },
            }

            if !sensed_entity_components.4.is_active {
                continue;
            }

            match sensed_entity_components.5.priority {
                crate::space_core::components::interpolation_priority::InterpolationPriorityStatus::High => {
                    high_priority_count+=1;
                },
                crate::space_core::components::interpolation_priority::InterpolationPriorityStatus::Medium => {
                    medium_priority_count+=1;
                },
                crate::space_core::components::interpolation_priority::InterpolationPriorityStatus::Low => {
                    low_priority_count+=1;
                },
            }


        }




        // With the counts, determine enum state for each priority type.

        let mut low_priority_rate;

        if low_priority_count < 8 {
            low_priority_rate = InterpolationPriorityRates::T12;
        } else if low_priority_count < 16 {
            low_priority_rate = InterpolationPriorityRates::T8;
        } else {
            low_priority_rate = InterpolationPriorityRates::T4;
        }

        if medium_priority_count > 6 {

            if medium_priority_count < 12 {
                low_priority_rate = InterpolationPriorityRates::T8;
            } else {
                low_priority_rate = InterpolationPriorityRates::T4;
            }

        }

        if high_priority_count > 5 {

            if high_priority_count < 10 {
                low_priority_rate = InterpolationPriorityRates::T8;
            } else {
                low_priority_rate = InterpolationPriorityRates::T4;
            }

        }





        let mut medium_priority_rate;

        if medium_priority_count < 10 {
            medium_priority_rate = InterpolationPriorityRates::T12;
        } else {
            medium_priority_rate = InterpolationPriorityRates::T8;
        }

        if low_priority_count > 30 {

            if low_priority_count < 50 {
                medium_priority_rate = InterpolationPriorityRates::T12;
            } else {
                medium_priority_rate = InterpolationPriorityRates::T8;
            }

        }

        if high_priority_count > 8 {
            if high_priority_count < 14 {
                medium_priority_rate = InterpolationPriorityRates::T12;
            } else {
                medium_priority_rate = InterpolationPriorityRates::T8;
            }
        }




        let mut high_priority_rate;
        if high_priority_count < 20 {
            high_priority_rate = InterpolationPriorityRates::T24;
        } else {
            high_priority_rate = InterpolationPriorityRates::T12;
        }

        if low_priority_count > 40 {
            high_priority_rate = InterpolationPriorityRates::T12;
        }

        if medium_priority_count > 20 {
            high_priority_rate = InterpolationPriorityRates::T12;
        }

        
        senser_priority_rates.insert(entity, (low_priority_rate,medium_priority_rate,high_priority_rate));

    }



    for (
        interpolated_entity,
        visible_component,
        rigid_body_position_component,
        rigid_body_velocity_component,
        mut cached_transform_component,
        interpolation_priority_component,
    ) in query_interpolated_entities.iter_mut() {

        let rigid_body_position = rigid_body_position_component.position;

        let rigid_body_translation_rapier = rigid_body_position.translation;
        let rigid_body_velocity_rapier = rigid_body_velocity_component.linvel;
        let rigid_body_rotation_rapier = rigid_body_position.rotation;

        let rigid_body_translation = Vec3::new(
            rigid_body_translation_rapier.x,
            rigid_body_translation_rapier.y,
            rigid_body_translation_rapier.z
        );

        let rigid_body_velocity = Vec3::new(
            rigid_body_velocity_rapier.x,
            rigid_body_velocity_rapier.y,
            rigid_body_velocity_rapier.z
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

        if this_transform == cached_transform_component.transform{
            cached_transform_component.is_active = false;
            continue;
        }

        cached_transform_component.is_active = true;

        cached_transform_component.transform = this_transform;

        

        for sensed_by_entity in visible_component.sensed_by.iter() {

            let low_priority_rate;
            let medium_priority_rate;
            let high_priority_rate;

            match senser_priority_rates.get(sensed_by_entity) {
                Some((low_rate,medium_rate,high_rate)) => {
                    low_priority_rate=low_rate;
                    medium_priority_rate=medium_rate;
                    high_priority_rate=high_rate;
                },
                None => {
                    warn!("Couldn't find entity id from sensable.sensed_by in senser_priority_rates.");
                    continue;
                },
            }

            let mut entity_tick_rate;
            
            match interpolation_priority_component.priority {
                crate::space_core::components::interpolation_priority::InterpolationPriorityStatus::High => {
                    entity_tick_rate = high_priority_rate;
                },
                crate::space_core::components::interpolation_priority::InterpolationPriorityStatus::Medium => {
                    entity_tick_rate = medium_priority_rate;
                },
                crate::space_core::components::interpolation_priority::InterpolationPriorityStatus::Low => {
                    entity_tick_rate = low_priority_rate;
                },
            }

            if *sensed_by_entity == interpolated_entity {
                entity_tick_rate = high_priority_rate;
            }

            if !is_interpolation_frame(
                &entity_tick_rate,
                interpolation_frame.i,
            ) {
                continue;
            }

            let rate_u : u8;

            match entity_tick_rate {
                InterpolationPriorityRates::T4 => {
                    rate_u = 4;
                },
                InterpolationPriorityRates::T8 => {
                    rate_u = 8;
                },
                InterpolationPriorityRates::T12 => {
                    rate_u = 12;
                },
                InterpolationPriorityRates::T24 => {
                    rate_u = 24;
                },
            }

            let player_handle_option = handle_to_entity.inv_map.get(&sensed_by_entity.id());

            match player_handle_option {
                Some(handle) => {

                    match net.send_message(
                        *handle,
                        UnreliableServerMessage::TransformUpdate (
                            interpolated_entity.to_bits(),
                            rigid_body_translation,
                            rigid_body_rotation,
                            rigid_body_velocity,
                            current_time_stamp as u64,
                            rate_u,
                        )
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
    entity_tick_rate : &InterpolationPriorityRates,
    current_frame : u8,
) -> bool {

    match entity_tick_rate {
        InterpolationPriorityRates::T4 => {
            if current_frame%6 == 0 {
                true
            } else {
                false
            }
        },
        InterpolationPriorityRates::T8 => {
            if current_frame%3 == 0 {
                true
            } else {
                false
            }
        },
        InterpolationPriorityRates::T12 => {
            if current_frame%2 == 0 {
                true
            } else {
                false
            }
        },
        InterpolationPriorityRates::T24 => {
            true
        },
    }

}
