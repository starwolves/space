use bevy::prelude::{
    warn, BuildChildren, Commands, EventReader, Query, Res, ResMut, SystemLabel, Transform, Vec3,
    Visibility,
};
use entity::spawn::{ClientEntityServerEntity, PawnEntityId};
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

pub fn set_active_item(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
    map: Res<ClientEntityServerEntity>,
    mut visible_query: Query<(&mut RigidBodyStatus, &mut Visibility, &mut Transform)>,
    mut commands: Commands,
    id: Res<PawnEntityId>,
) {
    for event in net.iter() {
        match event.message {
            InventoryServerMessage::SetActiveItem(entity) => {
                match inventory.active_item {
                    Some(old_active) => match map.map.get(&old_active) {
                        Some(ent) => match visible_query.get_mut(*ent) {
                            Ok((_status, mut comp, _t)) => {
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
                        Ok((mut status, mut comp, mut transform)) => {
                            comp.is_visible = true;
                            status.enabled = false;
                            match id.option {
                                Some(pawn_entity) => match map.map.get(&pawn_entity) {
                                    Some(ent2) => {
                                        commands.entity(*ent2).add_child(*ent);
                                        transform.translation = Vec3::new(0., 0., 0.);
                                    }
                                    None => {
                                        warn!("Couldnt find client side pawn entity.");
                                    }
                                },
                                None => {
                                    warn!("Pawn id wasnt yet set!");
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
