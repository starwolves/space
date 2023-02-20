use bevy::prelude::{
    warn, BuildChildren, Commands, Entity, EventReader, EventWriter, Query, Res, ResMut,
    SystemLabel, Transform, Vec3, Visibility,
};
use cameras::controllers::fps::ActiveCamera;
use entity::{entity_data::EntityData, entity_types::EntityType, spawn::ClientEntityServerEntity};
use networking::client::IncomingReliableServerMessage;
use physics::rigid_body::RigidBodyStatus;

use crate::{
    net::InventoryServerMessage,
    server::inventory::{Inventory, SlotItem},
};

pub(crate) fn client_item_added_to_slot(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
) {
    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(event) => {
                match inventory.slots.get_mut(&event.slot_id) {
                    Some(slot) => {
                        slot.items.push(SlotItem {
                            entity: event.item_entity,
                            position: event.position,
                        });
                    }
                    None => {
                        warn!("couldnt find slot to add to.");
                    }
                }
            }
            _ => (),
        }
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum ClientBuildInventoryLabel {
    AddSlot,
}

/// Event that fires when an item becomes acive selected and displayed in front of first person camera.
pub struct ActiveItemCamera {
    pub client_entity: Entity,
}

pub fn active_item_display_camera<T: Send + Sync + EntityType + Default + 'static>(
    mut events: EventReader<ActiveItemCamera>,
    mut entity_type_query: Query<(&EntityData, &mut Transform)>,
) {
    for event in events.iter() {
        match entity_type_query.get_mut(event.client_entity) {
            Ok((data, mut transform)) => {
                if data.entity_type.get_identity() != T::default().get_identity() {
                    continue;
                }
                transform.translation = Vec3::new(0.4, -0.4, -1.);
                let mut look_at_translation = transform.translation.clone();
                look_at_translation.x += 1.;
                look_at_translation.z -= 0.1;
                *transform = transform.looking_at(look_at_translation, Vec3::Y);
            }
            Err(_) => {
                warn!("Couldnt find entitytype.");
            }
        }
    }
}

pub fn set_active_item(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
    map: Res<ClientEntityServerEntity>,
    mut visible_query: Query<(&mut RigidBodyStatus, &mut Visibility)>,
    state: Res<ActiveCamera>,
    mut commands: Commands,

    mut events: EventWriter<ActiveItemCamera>,
) {
    for event in net.iter() {
        match event.message {
            InventoryServerMessage::SetActiveItem(entity) => {
                match inventory.active_item {
                    Some(old_active) => match map.map.get(&old_active) {
                        Some(ent) => match visible_query.get_mut(*ent) {
                            Ok((_status, mut comp)) => {
                                comp.is_visible = false;
                            }
                            Err(_) => {
                                warn!("Couldnt find old visible component.");
                            }
                        },
                        None => {
                            warn!("Coudlnt find old client entity.");
                        }
                    },
                    None => {}
                }
                inventory.active_item = Some(entity);
                match map.map.get(&entity) {
                    Some(ent) => match visible_query.get_mut(*ent) {
                        Ok((mut status, mut comp)) => {
                            comp.is_visible = true;
                            status.enabled = false;
                            match state.option {
                                Some(camera_entity) => {
                                    commands.entity(camera_entity).add_child(*ent);
                                    events.send(ActiveItemCamera {
                                        client_entity: *ent,
                                    });
                                }
                                None => {
                                    warn!("No active cam found.");
                                }
                            }
                        }
                        Err(_) => {
                            warn!("Couldnt find visible component.");
                        }
                    },
                    None => {
                        warn!("Coudlnt find client entity.");
                    }
                }
            }
            _ => (),
        }
    }
}
