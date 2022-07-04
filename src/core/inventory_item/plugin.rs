use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut, SystemSet};

use crate::core::{
    console_commands::{commands::AllConsoleCommands, plugins::ConsoleCommandsLabels},
    networking::networking::ConsoleCommandVariant,
    space_plugin::plugin::{PostUpdateLabels, SummoningLabels},
};

use super::{
    console_commands::inventory_item_console_commands, entity_update::inventory_item_update,
};
use bevy::app::CoreStage::PostUpdate;

pub struct InventoryItemPlugin;

impl Plugin for InventoryItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(inventory_item_update),
        )
        .add_system(
            inventory_item_console_commands
                .before(SummoningLabels::TriggerSummon)
                .label(SummoningLabels::NormalSummon),
        )
        .add_startup_system(initialize_console_commands.before(ConsoleCommandsLabels::Finalize));
    }
}

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "spawnHeld".to_string(),
        "For server administrators only. Spawn in held entities in hands or in proximity."
            .to_string(),
        vec![
            ("entity_name".to_string(), ConsoleCommandVariant::String),
            ("player_selector".to_string(), ConsoleCommandVariant::String),
        ],
    ));
}
