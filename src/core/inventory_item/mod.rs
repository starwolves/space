use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};
use bevy_ecs::system::ResMut;

use self::entity_update::inventory_item_update;
use self::systems::inventory_item_console_commands;

use super::console_commands::resources::AllConsoleCommands;
use super::console_commands::ConsoleCommandsLabels;
use super::networking::resources::ConsoleCommandVariant;
use super::{PostUpdateLabels, SummoningLabels};

pub mod components;
pub mod entity_update;
pub mod spawn;
pub mod systems;

pub struct InventoryItemPlugin;

impl Plugin for InventoryItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            PostUpdate,
            SystemSet::new()
                .label(PostUpdateLabels::EntityUpdate)
                .with_system(inventory_item_update),
        )
        .add_system(inventory_item_console_commands.label(SummoningLabels::TriggerSummon))
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
