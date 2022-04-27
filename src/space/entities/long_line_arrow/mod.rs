use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::space::{
    core::{
        console_commands::{resources::ConsoleCommands, ConsoleCommandsLabels},
        networking::resources::ConsoleCommandVariant,
    },
    StartupLabels,
};

use self::systems::{entity_console_commands, point_arrow};

pub mod components;
pub mod events;
pub mod spawn;
pub mod systems;

pub struct LongLineArrowPlugin;

impl Plugin for LongLineArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            initialize_console_commands
                .before(ConsoleCommandsLabels::Finalize)
                .label(StartupLabels::ConsoleCommands),
        )
        .add_system(entity_console_commands);
    }
}

pub struct PointArrowPlugin;

impl Plugin for PointArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(point_arrow);
    }
}

pub fn initialize_console_commands(mut commands: ResMut<ConsoleCommands>) {
    commands.list.push((
        "arrow_point".to_string(),
        "Spawn an arrow with a specified duration and world position to point at.".to_string(),
        vec![
            ("x".to_string(), ConsoleCommandVariant::Float),
            ("y".to_string(), ConsoleCommandVariant::Float),
            ("z".to_string(), ConsoleCommandVariant::Float),
            ("duration".to_string(), ConsoleCommandVariant::Int),
        ],
    ));
}
