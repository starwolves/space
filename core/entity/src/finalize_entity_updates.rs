use bevy::prelude::{Changed, Entity, Query, Res};

use crate::showcase::Showcase;

use networking::server::{ConnectedPlayer, HandleToEntity};

use crate::{entity_data::EntityUpdates, sensable::Sensable};

use crate::entity_data::personalise;
use bevy::prelude::EventWriter;

use networking::typenames::OutgoingReliableServerMessage;

use crate::networking::{EntityServerMessage, EntityWorldType};
/// Finalize entity updates of this frame and send them to Godot clients.
#[cfg(feature = "server")]
pub(crate) fn finalize_entity_updates(
    mut updated_entity_updates: Query<
        (
            Entity,
            Option<&Sensable>,
            &mut EntityUpdates,
            Option<&ConnectedPlayer>,
            Option<&Showcase>,
        ),
        Changed<EntityUpdates>,
    >,
    handle_to_entity: Res<HandleToEntity>,
    mut server: EventWriter<OutgoingReliableServerMessage<EntityServerMessage>>,
) {
    for (
        visible_entity,
        visible_component_option,
        mut entity_updates_component,
        connected_player_component_option,
        showcase_component_option,
    ) in updated_entity_updates.iter_mut()
    {
        if entity_updates_component.changed_parameters.len() == 1
            && entity_updates_component
                .changed_parameters
                .contains(&"play_back_position".to_string())
        {
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
                                personalise(
                                    updates_data,
                                    connected_player_component.handle,
                                    &entity_updates_component,
                                );
                            }
                            None => {}
                        }

                        if updates_data.len() == 0 {
                            continue;
                        }

                        match handle_to_entity.inv_map.get(&sensed_by_entity) {
                            Some(handle) => {
                                server.send(OutgoingReliableServerMessage {
                                    handle: *handle,
                                    message: EntityServerMessage::EntityUpdate(
                                        visible_entity.to_bits(),
                                        updates_data.clone(),
                                        false,
                                        EntityWorldType::Main,
                                    ),
                                });
                            }
                            None => {}
                        }
                    }
                }

                entity_updates_component.updates_difference.clear();
            }
            None => {}
        }

        match showcase_component_option {
            Some(showcase_component) => {
                let mut updates_data = entity_updates_component.updates.clone();

                match connected_player_component_option {
                    Some(connected_player_component) => {
                        personalise(
                            &mut updates_data,
                            connected_player_component.handle,
                            &entity_updates_component,
                        );
                    }
                    None => {}
                }

                if updates_data.len() == 0 {
                    continue;
                }
                server.send(OutgoingReliableServerMessage {
                    handle: showcase_component.handle,
                    message: EntityServerMessage::EntityUpdate(
                        visible_entity.to_bits(),
                        updates_data,
                        false,
                        EntityWorldType::Main,
                    ),
                })
            }
            None => {}
        }
    }
}
