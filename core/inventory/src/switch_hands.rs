use bevy::prelude::{Entity, EventReader, Query, Res};
use inventory_api::core::Inventory;
use networking::server::HandleToEntity;

use bevy::prelude::ResMut;
use bevy_renet::renet::RenetServer;
/// From client input change active hand.
#[cfg(feature = "server")]
pub(crate) fn switch_hands(
    mut switch_hands_events: EventReader<InputSwitchHands>,
    mut inventory_entities: Query<&mut Inventory>,
    mut server: ResMut<RenetServer>,
    handle_to_entity: Res<HandleToEntity>,
) {
    use networking::plugin::RENET_RELIABLE_CHANNEL_ID;

    use crate::networking::InventoryServerMessage;

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
                server.send_message(
                    *handle,
                    RENET_RELIABLE_CHANNEL_ID,
                    bincode::serialize(&InventoryServerMessage::SwitchHands).unwrap(),
                );
            }
            None => {}
        }
    }
}

/// Client input switch hands event.
#[cfg(feature = "server")]
pub struct InputSwitchHands {
    pub entity: Entity,
}
