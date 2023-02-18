use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::IntoConditionalSystem;
use resources::is_server::is_server;

use crate::{
    expand::{expand_hud, ExpandHud},
    hud::{create_hud, HudState},
    inventory::{
        create_inventory_hud, inventory_hud_key_press, inventory_net_updates, open_inventory_hud,
        queue_inventory_updates, update_inventory_hud, HudAddInventorySlot, HudAddItemToSlot,
        InventoryState, InventoryUpdatesQueue, OpenInventoryHud,
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
                .add_system(inventory_hud_key_press.run_if_resource_exists::<InventoryState>())
                .add_system(open_inventory_hud.run_if_resource_exists::<InventoryState>())
                .add_system(queue_inventory_updates.run_unless_resource_exists::<InventoryState>())
                .add_system(inventory_net_updates.run_if_resource_exists::<InventoryState>())
                .add_system(update_inventory_hud.run_if_resource_exists::<InventoryState>())
                .add_event::<HudAddItemToSlot>()
                .add_event::<HudAddInventorySlot>()
                .init_resource::<InventoryUpdatesQueue>();
        }
    }
}
