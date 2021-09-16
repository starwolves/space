use bevy::prelude::{Changed, Entity, EventWriter, Query, Res};

use crate::space_core::{components::{connected_player::ConnectedPlayer, entity_updates::EntityUpdates, sensable::Sensable, showcase::Showcase}, events::net::net_send_entity_updates::NetSendEntityUpdates, functions::entity_updates::entity_updates_personalise, resources::{handle_to_entity::HandleToEntity, network_messages::ReliableServerMessage}};

pub fn send_entity_updates(
    mut updated_entity_updates: Query<(Entity, Option<&Sensable>, &mut EntityUpdates, Option<&ConnectedPlayer>, Option<&Showcase>), Changed<EntityUpdates>>,
    mut net_send_entity_updates: EventWriter<NetSendEntityUpdates>,
    handle_to_entity: Res<HandleToEntity>,
) {

    for (
        visible_entity, 
        visible_component_option, 
        mut entity_updates_component,
        connected_player_component_option,
        showcase_component_option,
    ) in updated_entity_updates.iter_mut() {

        if entity_updates_component.changed_parameters.len() == 1 &&
        entity_updates_component.changed_parameters.contains(&"play_back_position".to_string()) {
            entity_updates_component.updates_difference.clear();
            continue;
        }

        match visible_component_option {
            Some(visible_component) => {

                for sensed_by_entity in visible_component.sensed_by.iter() {

                    let mut updates_data_vec = entity_updates_component.updates_difference.clone();
                    
                    for updates_data in updates_data_vec.iter_mut() {


                        match connected_player_component_option {
                            Some(connected_player_component) => {
                                
                                entity_updates_personalise::personalise(
                                    updates_data,
                                    connected_player_component.handle,
                                    &entity_updates_component
                                );
                    
                            },
                            None => {},
                        }

                        
            
                        if updates_data.len() == 0 {
                            continue;
                        }
    
                        match handle_to_entity.inv_map.get(&sensed_by_entity) {
                            Some(handle) => {
                                net_send_entity_updates.send(NetSendEntityUpdates {
                                    handle: *handle,
                                    message: ReliableServerMessage::EntityUpdate(visible_entity.to_bits(), updates_data.clone(), false)
                                });
                            },
                            None => {},
                        }

                    }

                    
        
                }

                entity_updates_component.updates_difference.clear();

            },
            None => {},
        }


        match showcase_component_option {
            Some(showcase_component) => {

                let mut updates_data = entity_updates_component.updates.clone();

                match connected_player_component_option {
                    Some(connected_player_component) => {
                        
                        entity_updates_personalise::personalise(
                            &mut updates_data,
                            connected_player_component.handle,
                            &entity_updates_component
                        );
            
                    },
                    None => {},
                }
    
                if updates_data.len() == 0 {
                    continue;
                }

                net_send_entity_updates.send(NetSendEntityUpdates {
                    handle: showcase_component.handle,
                    message: ReliableServerMessage::EntityUpdate(visible_entity.to_bits(), updates_data, false)
                });

            },
            None => {},
        }

    }

}
