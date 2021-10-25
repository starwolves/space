use bevy::prelude::{EventReader, EventWriter, Query};

use crate::space_core::{components::inventory::Inventory, events::{general::switch_hands::InputSwitchHands, net::net_switch_hands::NetSwitchHands}, resources::network_messages::ReliableServerMessage};

pub fn switch_hands(
    mut switch_hands_events : EventReader<InputSwitchHands>,
    mut inventory_entities : Query<
        &mut Inventory,
    >,
    mut net_switch_hands : EventWriter<NetSwitchHands>,
) {

    for event in switch_hands_events.iter() {

        let hand_switcher_components_option = inventory_entities.get_mut(event.entity);
        let hand_switcher_components;


        match hand_switcher_components_option {
            Ok(components) => {
                hand_switcher_components =components;
            },
            Err(_rr) => {
                continue;
            },
        }


        let mut hand_switcher_inventory = hand_switcher_components;

        if hand_switcher_inventory.active_slot == "left_hand" {
            hand_switcher_inventory.active_slot = "right_hand".to_string();
        } else {
            hand_switcher_inventory.active_slot = "left_hand".to_string();
        }
        

        net_switch_hands.send(NetSwitchHands {
            handle: event.handle,
            message: ReliableServerMessage::SwitchHands,
        });


    }

}
