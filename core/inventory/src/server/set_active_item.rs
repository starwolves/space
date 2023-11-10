use bevy::log::warn;
use bevy::prelude::{EventReader, EventWriter, Query, Res};
use networking::server::{
    HandleToEntity, IncomingReliableClientMessage, OutgoingReliableServerMessage,
};

use crate::net::{InventoryClientMessage, InventoryServerMessage};

use super::inventory::Inventory;

pub(crate) fn process_request_set_active_item(
    mut net: EventReader<IncomingReliableClientMessage<InventoryClientMessage>>,
    mut o_net: EventWriter<OutgoingReliableServerMessage<InventoryServerMessage>>,
    mut inventory_query: Query<&mut Inventory>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in net.read() {
        match event.message {
            InventoryClientMessage::RequestSetActiveItem(requested_active_item) => {
                match handle_to_entity.map.get(&event.handle) {
                    Some(pawn_entity) => match inventory_query.get_mut(*pawn_entity) {
                        Ok(mut inventory_component) => {
                            inventory_component.active_item = Some(requested_active_item);
                            o_net.send(OutgoingReliableServerMessage {
                                handle: event.handle,
                                message: InventoryServerMessage::SetActiveItem(
                                    requested_active_item,
                                ),
                            });
                        }
                        Err(_) => {
                            warn!("Couldnt find pawn entity.");
                        }
                    },
                    None => {
                        warn!("Couldnt find handle entity.");
                    }
                }
            }
        }
    }
}
