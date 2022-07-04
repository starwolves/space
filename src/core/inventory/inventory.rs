#[derive(Component)]
pub struct Inventory {
    pub slots: Vec<Slot>,
    pub active_slot: String,
    pub entity_tab_action_option: Option<Entity>,
}

#[derive(Debug)]
pub struct Slot {
    pub slot_type: SlotType,
    pub slot_name: String,
    pub slot_item: Option<Entity>,
    pub slot_attachment: Option<String>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            slots: vec![],
            active_slot: "".to_string(),
            entity_tab_action_option: None,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SlotType {
    Generic,
    Helmet,
    Jumpsuit,
    Holster,
}

impl Inventory {
    pub fn has_item(&self, entity_id: Entity) -> bool {
        let mut has = false;

        for slot in self.slots.iter() {
            match slot.slot_item {
                Some(item_entity) => {
                    if item_entity == entity_id {
                        has = true;
                        break;
                    }
                }
                None => {}
            }
        }

        has
    }

    pub fn get_active_slot_entity(&self) -> Option<Entity> {
        let mut return_slot_option = None;

        for slot in self.slots.iter() {
            if slot.slot_name == self.active_slot {
                return_slot_option = Some(slot);
                break;
            }
        }

        return_slot_option
            .expect("inventory.rs get_active_entity() couldn't find slot")
            .slot_item
    }

    pub fn get_slot_mut(&mut self, slot_name: &str) -> &mut Slot {
        let mut return_slot_option = None;

        for slot in self.slots.iter_mut() {
            if slot.slot_name == slot_name {
                return_slot_option = Some(slot);
                break;
            }
        }

        return_slot_option.expect("inventory.rs get_slot_mut() couldn't find slot")
    }

    pub fn get_slot(&self, slot_name: &str) -> &Slot {
        let mut return_slot_option = None;

        for slot in self.slots.iter() {
            if slot.slot_name == slot_name {
                return_slot_option = Some(slot);
                break;
            }
        }

        return_slot_option.expect("inventory.rs get_slot() couldn't find slot")
    }
}

use bevy::{
    math::Vec3,
    prelude::{Component, Entity, EventReader, EventWriter, Query, Res},
};

pub struct InputDropCurrentItem {
    pub pickuper_entity: Entity,
    pub input_position_option: Option<Vec3>,
}

pub struct InputThrowItem {
    pub entity: Entity,
    pub position: Vec3,
    pub angle: f32,
}

pub struct InputSwitchHands {
    pub entity: Entity,
}

pub struct InputTakeOffItem {
    pub entity: Entity,
    pub slot_name: String,
}

pub struct InputUseWorldItem {
    pub pickuper_entity: Entity,
    pub pickupable_entity_bits: u64,
}

pub struct InputWearItem {
    pub wearer_entity: Entity,
    pub wearable_id_bits: u64,
    pub wear_slot: String,
}

use crate::core::{
    connected_player::plugin::HandleToEntity, networking::networking::ReliableServerMessage,
};

use super::net::NetSwitchHands;

pub fn switch_hands(
    mut switch_hands_events: EventReader<InputSwitchHands>,
    mut inventory_entities: Query<&mut Inventory>,
    mut net_switch_hands: EventWriter<NetSwitchHands>,
    handle_to_entity: Res<HandleToEntity>,
) {
    for event in switch_hands_events.iter() {
        let hand_switcher_components_option = inventory_entities.get_mut(event.entity);
        let hand_switcher_components;

        match hand_switcher_components_option {
            Ok(components) => {
                hand_switcher_components = components;
            }
            Err(_rr) => {
                continue;
            }
        }

        let mut hand_switcher_inventory = hand_switcher_components;

        if hand_switcher_inventory.active_slot == "left_hand" {
            hand_switcher_inventory.active_slot = "right_hand".to_string();
        } else {
            hand_switcher_inventory.active_slot = "left_hand".to_string();
        }

        match handle_to_entity.inv_map.get(&event.entity) {
            Some(handle) => {
                net_switch_hands.send(NetSwitchHands {
                    handle: *handle,
                    message: ReliableServerMessage::SwitchHands,
                });
            }
            None => {}
        }
    }
}
