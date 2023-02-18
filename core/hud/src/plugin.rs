use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use iyes_loopless::prelude::IntoConditionalSystem;
use resources::is_server::is_server;

use crate::{
    expand::{expand_hud, ExpandHud},
    hud::{create_hud, HudState},
    inventory::{
        build::{
            create_inventory_hud, inventory_hud_key_press, open_inventory_hud, InventoryHudState,
            OpenInventoryHud,
        },
        items::{requeue_hud_add_item_to_slot, HudAddItemToSlot},
        queue::{
            inventory_net_updates, queue_inventory_updates, InventoryUpdatesQueue,
            RequeueHudAddItemToSlot,
        },
        slots::{scale_slots, update_inventory_hud_slot, HudAddInventorySlot, InventoryHudLabels},
    },
};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_event::<ExpandHud>()
                .add_system(expand_hud.run_if_resource_exists::<HudState>())
                .add_system(create_inventory_hud.run_if_resource_exists::<HudState>())
                .add_system(create_hud)
                .add_event::<OpenInventoryHud>()
                .add_system(inventory_hud_key_press.run_if_resource_exists::<InventoryHudState>())
                .add_system(open_inventory_hud.run_if_resource_exists::<InventoryHudState>())
                .add_system(
                    queue_inventory_updates.run_unless_resource_exists::<InventoryHudState>(),
                )
                .add_system(inventory_net_updates.run_if_resource_exists::<InventoryHudState>())
                .add_system(
                    update_inventory_hud_slot
                        .run_if_resource_exists::<InventoryHudState>()
                        .label(InventoryHudLabels::UpdateSlot),
                )
                .add_event::<HudAddItemToSlot>()
                .add_event::<HudAddInventorySlot>()
                .init_resource::<InventoryUpdatesQueue>()
                .add_system(requeue_hud_add_item_to_slot.after(InventoryHudLabels::QueueUpdate))
                .add_event::<RequeueHudAddItemToSlot>()
                .add_system(scale_slots.run_if_resource_exists::<InventoryHudState>());
        }
    }
}
