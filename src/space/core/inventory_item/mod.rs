use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::SystemSet;

use crate::space::PostUpdateLabels;

use self::entity_update::inventory_item_update;
use self::systems::inventory_item_console_commands;

pub mod components;
pub mod entity_update;
mod systems;

pub struct InventoryItemPlugin;

impl Plugin for InventoryItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(inventory_item_update),
        )
        .add_system(inventory_item_console_commands);
    }
}
