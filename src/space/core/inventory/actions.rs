use bevy_app::EventWriter;
use bevy_ecs::system::Res;

use crate::space::core::tab_actions::resources::QueuedTabActions;

use super::events::InputUseWorldItem;

pub fn inventory_actions(
    queue: Res<QueuedTabActions>,

    mut pickup_world_item_event: EventWriter<InputUseWorldItem>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "pickup" {
            if queued.target_entity_option.is_some() {
                pickup_world_item_event.send(InputUseWorldItem {
                    pickuper_entity: queued.player_entity,
                    pickupable_entity_bits: queued.target_entity_option.unwrap(),
                });
            }
        }
    }
}
