use api::{
    data::HandleToEntity,
    gridmap::{to_doryen_coordinates, world_to_cell_id},
    network::{PendingMessage, PendingNetworkMessage, ReliableServerMessage},
    senser::Senser,
};
use bevy::prelude::{warn, Entity, EventReader, EventWriter, Query, Res, SystemLabel, Transform};

/// Requested proximity message.
pub struct EntityProximityMessage {
    pub entities: Vec<Entity>,
    pub message: String,
}

pub(crate) struct NetProximityMessage {
    pub handle: u64,
    pub message: ReliableServerMessage,
}
impl PendingMessage for NetProximityMessage {
    fn get_message(&self) -> PendingNetworkMessage {
        PendingNetworkMessage {
            handle: self.handle,
            message: self.message.clone(),
        }
    }
}

/// Requested entity proximity messages systems ordering label.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum EntityProximityMessages {
    Send,
}

/// Manage entity proximity messages.
pub(crate) fn send_entity_proximity_messages(
    mut entity_proximity_messages: EventReader<EntityProximityMessage>,
    sensers: Query<(Entity, &Senser)>,
    positions: Query<&Transform>,
    handle_to_entity: Res<HandleToEntity>,
    mut net: EventWriter<NetProximityMessage>,
) {
    for entity_proximity_message in entity_proximity_messages.iter() {
        for proximity_entity in entity_proximity_message.entities.iter() {
            let entity_transform;

            match positions.get(*proximity_entity) {
                Ok(t) => {
                    entity_transform = t;
                }
                Err(_rr) => {
                    warn!("Couldnt find transform of entity");
                    continue;
                }
            }

            let entity_gridmap_coords = world_to_cell_id(entity_transform.translation);
            let entity_cell_id_doryen =
                to_doryen_coordinates(entity_gridmap_coords.x, entity_gridmap_coords.z);

            for (entity, senser_component) in sensers.iter() {
                if senser_component.fov.is_in_fov(
                    entity_cell_id_doryen.0 as usize,
                    entity_cell_id_doryen.1 as usize,
                ) {
                    match handle_to_entity.inv_map.get(&entity) {
                        Some(handle) => {
                            net.send(NetProximityMessage {
                                handle: *handle,
                                message: ReliableServerMessage::ChatMessage(
                                    entity_proximity_message.message.clone(),
                                ),
                            });
                        }
                        None => {}
                    }
                }
            }
        }
    }
}
