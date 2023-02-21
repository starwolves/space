use bevy::prelude::{App, IntoSystemDescriptor, Plugin, StartupStage};
use iyes_loopless::prelude::IntoConditionalSystem;
use resources::is_server::is_server;

use crate::{
    expand::{expand_hud, ExpandInventoryHud},
    hud::{create_hud, show_hud, HudLabels},
    inventory::{
        actions::{hide_actions, item_actions_button_events, slot_item_actions},
        build::{
            create_inventory_hud, inventory_hud_key_press, open_inventory_hud, InventoryHudState,
            OpenInventoryHud,
        },
        items::{
            change_active_item, requeue_hud_add_item_to_slot, right_mouse_click_item,
            slot_item_button_events, HoveringSlotItem, HudAddItemToSlot,
        },
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
            app.add_event::<ExpandInventoryHud>()
                .add_system(expand_hud)
                .add_startup_system_to_stage(StartupStage::PostStartup, create_inventory_hud)
                .add_startup_system(create_hud.label(HudLabels::CreateHud))
                .add_event::<OpenInventoryHud>()
                .add_system(inventory_hud_key_press)
                .add_system(open_inventory_hud)
                .add_system(
                    queue_inventory_updates.run_unless_resource_exists::<InventoryHudState>(),
                )
                .add_system(inventory_net_updates)
                .add_system(update_inventory_hud_slot.label(InventoryHudLabels::UpdateSlot))
                .add_event::<HudAddItemToSlot>()
                .add_event::<HudAddInventorySlot>()
                .init_resource::<InventoryUpdatesQueue>()
                .add_system(requeue_hud_add_item_to_slot.after(InventoryHudLabels::QueueUpdate))
                .add_event::<RequeueHudAddItemToSlot>()
                .add_system(scale_slots)
                .add_system(slot_item_button_events)
                .add_system(change_active_item)
                .init_resource::<HoveringSlotItem>()
                .add_system(right_mouse_click_item)
                .add_system(slot_item_actions)
                .add_system(show_hud)
                .add_system(hide_actions)
                .add_system(item_actions_button_events);
        }
    }
}
