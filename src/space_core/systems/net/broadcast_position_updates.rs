use bevy::{core::{FixedTimesteps, Time}, prelude::{Entity, Query, Res, ResMut, warn}};
use bevy_networking_turbulence::NetworkResource;

use crate::space_core::{components::{cached_broadcast_transform::CachedBroadcastTransform, sensable::Sensable, static_transform::StaticTransform, update_transform::UpdateTransform}, resources::handle_to_entity::HandleToEntity, structs::network_messages::UnreliableServerMessage};

const INTERPOLATION_LABEL1: &str = "fixed_timestep_interpolation1";


pub fn broadcast_position_updates (
    time: Res<Time>, 
    fixed_timesteps: Res<FixedTimesteps>,
    
    mut net: ResMut<NetworkResource>,
    handle_to_entity : Res<HandleToEntity>,
    mut query_update_transform_entities : Query<(Entity, &Sensable, &UpdateTransform, &StaticTransform, &mut CachedBroadcastTransform)>
) {

    let current_time_stamp = time.time_since_startup().as_millis();

    let fixed_timestep = fixed_timesteps.get(INTERPOLATION_LABEL1).unwrap().overstep_percentage();
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
        _update_transform_component,
        static_transform_component,
        mut cached_transform_component
    ) in query_update_transform_entities.iter_mut() {

        if cached_transform_component.transform == static_transform_component.transform {
            continue;
        }

        cached_transform_component.transform = static_transform_component.transform;

        let entity_id = entity.id();
        let new_position = static_transform_component.transform.translation;

        for sensed_by_entity in visible_component.sensed_by.iter() {

            let player_handle_option = handle_to_entity.inv_map.get(&sensed_by_entity.id());

            match player_handle_option {
                Some(handle) => {

                    match net.send_message(
                        *handle,
                        UnreliableServerMessage::PositionUpdate (
                            entity_id,
                            entity.generation(),
                            new_position,
                            current_time_stamp as u64
                        )
                    ) {
                        Ok(msg) => match msg {
                            Some(msg) => {
                                warn!("was unable to send PositionUpdate message: {:?}", msg);
                            }
                            None => {}
                        },
                        Err(err) => {
                            warn!("was unable to send PositionUpdate message (1): {:?}", err);
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