use bevy::{core::{FixedTimesteps, Time}, math::{Quat, Vec3}, prelude::{Entity, Query, Res, ResMut, Transform, Without, warn}};
use bevy_networking_turbulence::NetworkResource;
use bevy_rapier3d::{prelude::{RigidBodyPosition, RigidBodyVelocity}};

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, rigidbody_disabled::RigidBodyDisabled, sensable::Sensable, static_transform::StaticTransform}, resources::handle_to_entity::HandleToEntity, structs::network_messages::UnreliableServerMessage};

const INTERPOLATION_LABEL: &str = "fixed_timestep_interpolation";


pub fn broadcast_interpolation_transforms (
    time: Res<Time>, 
    fixed_timesteps: Res<FixedTimesteps>,
    
    mut net: ResMut<NetworkResource>,
    handle_to_entity : Res<HandleToEntity>,
    mut query_interpolated_entities : Query<(Entity, &Sensable, &RigidBodyPosition, &RigidBodyVelocity, &mut CachedBroadcastTransform), (Without<StaticTransform>, Without<RigidBodyDisabled>)>,
) {
    
    let current_time_stamp = time.time_since_startup().as_millis();

    let fixed_timestep = fixed_timesteps.get(INTERPOLATION_LABEL).unwrap().overstep_percentage();
    if fixed_timestep > 5. {

        if current_time_stamp > 60000 {
            warn!(
                "overstep_percentage: {}",
                fixed_timestep
            );
        }

    }

    for (
        entity,
        visible_component,
        rigid_body_position_component,
        rigid_body_velocity_component,
        mut cached_transform_component
    ) in query_interpolated_entities.iter_mut() {

        let entity_id = entity.id();


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
            continue;
        }


        cached_transform_component.transform = this_transform;

        for sensed_by_entity in visible_component.sensed_by.iter() {

            let player_handle_option = handle_to_entity.inv_map.get(&sensed_by_entity.id());

            match player_handle_option {
                Some(handle) => {

                    match net.send_message(
                        *handle,
                        UnreliableServerMessage::TransformUpdate (
                            entity_id,
                            entity.generation(),
                            rigid_body_translation,
                            rigid_body_rotation,
                            rigid_body_velocity,
                            current_time_stamp as u64
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