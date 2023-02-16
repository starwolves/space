use bevy::prelude::{Plugin, App};
use iyes_loopless::prelude::IntoConditionalSystem;
use resources::is_server::is_server;

use crate::{expand::{ExpandHud, expand_hud}, hud::{create_hud, HudState}, inventory::{OpenInventoryHud, inventory_hud_key_press, open_inventory_hud, create_inventory_hud, InventoryState}};


pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_event::<ExpandHud>()
            .add_system(expand_hud.run_if_resource_exists::<HudState>())
            .add_system(create_inventory_hud.run_if_resource_exists::<HudState>())
            .add_system( create_hud)
            .add_event::<OpenInventoryHud>()
            .add_system(inventory_hud_key_press.run_if_resource_exists::<InventoryState>()) 
            .add_system(open_inventory_hud.run_if_resource_exists::<InventoryState>());
        }
    }
}
