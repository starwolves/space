use bevy::prelude::{EventWriter, Res};

use crate::core::tab_actions::tab_action::QueuedTabActions;

use super::inventory::InputUseWorldItem;

pub fn actions(
    queue: Res<QueuedTabActions>,

    mut pickup_world_item_event: EventWriter<InputUseWorldItem>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "actions::inventory/pickup" {
            if queued.target_entity_option.is_some() {
                pickup_world_item_event.send(InputUseWorldItem {
                    pickuper_entity: queued.player_entity,
                    pickupable_entity_bits: queued.target_entity_option.unwrap(),
                });
            }
        }
    }
}
