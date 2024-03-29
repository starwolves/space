use bevy::log::warn;
use bevy::prelude::{
    BuildChildren, Commands, Entity, Event, EventReader, EventWriter, Query, Res, ResMut,
    SystemSet, Transform, Vec3, Visibility,
};

use bevy_xpbd_3d::components::RigidBody;
use cameras::controllers::fps::ActiveCamera;
use entity::{entity_data::EntityData, entity_types::EntityType, spawn::ServerEntityClientEntity};
use networking::client::IncomingReliableServerMessage;
use physics::entity::RigidBodies;

use crate::{
    net::InventoryServerMessage,
    server::inventory::{Inventory, SlotItem},
};

pub(crate) fn client_item_added_to_slot(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
) {
    for message in net.read() {
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
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum ClientBuildInventoryLabel {
    AddSlot,
    Net,
}

/// Event that fires when an item becomes acive selected and displayed in front of first person camera.
#[derive(Event)]
pub struct ActiveItemCamera {
    pub client_entity: Entity,
}

pub fn active_item_display_camera<T: Send + Sync + EntityType + Default + 'static>(
    mut events: EventReader<ActiveItemCamera>,
    mut entity_type_query: Query<(&EntityData, &mut Transform)>,
) {
    for event in events.read() {
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum ClientActiveCameraItem {
    ActivateItem,
}

pub fn set_active_item(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut inventory: ResMut<Inventory>,
    map: Res<ServerEntityClientEntity>,
    mut visible_query: Query<(Entity, &mut Visibility)>,
    state: Res<ActiveCamera>,
    mut commands: Commands,

    mut events: EventWriter<ActiveItemCamera>,
    rigidbodies: Res<RigidBodies>,
) {
    for event in net.read() {
        match event.message {
            InventoryServerMessage::SetActiveItem(entity) => {
                match inventory.active_item {
                    Some(old_active) => match map.map.get(&old_active) {
                        Some(ent) => match visible_query.get_mut(*ent) {
                            Ok((_status, mut comp)) => {
                                *comp = Visibility::Hidden;
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
                        Ok((entt, mut comp)) => {
                            *comp = Visibility::Inherited;
                            match rigidbodies.get_entity_rigidbody(&entt) {
                                Some(rb) => {
                                    commands.entity(*rb).remove::<RigidBody>();
                                }
                                None => {
                                    warn!("Couldnt find rigidbody.");
                                }
                            }
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
