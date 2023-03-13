use basic_console_commands::register::register_basic_console_commands_for_type;
use bevy::prelude::{App, IntoSystemConfig, Plugin, ResMut};
use console_commands::commands::{AllConsoleCommands, ConsoleCommand, ConsoleCommandsLabels};
use entity::{entity_types::register_entity_type, spawn::build_base_entities};
use networking::server::ConsoleArgVariant;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, StartupLabels},
};

use crate::console_command::entity_console_commands;

use super::{
    console_command::expire_point_arrow,
    spawn::{build_line_arrows, LineArrowType},
};

pub struct LineArrowPlugin;

impl Plugin for LineArrowPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(entity_console_commands.in_set(BuildingLabels::TriggerBuild));
        }
        register_entity_type::<LineArrowType>(app);
        register_basic_console_commands_for_type::<LineArrowType>(app);
        app.add_system((build_base_entities::<LineArrowType>).after(BuildingLabels::TriggerBuild))
            .add_system(build_line_arrows::<LineArrowType>.after(BuildingLabels::TriggerBuild))
            .add_startup_system(
                initialize_console_commands
                    .before(ConsoleCommandsLabels::Finalize)
                    .in_set(StartupLabels::ConsoleCommands),
            );
    }
}

pub struct PointArrowPlugin;

impl Plugin for PointArrowPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(expire_point_arrow).add_system(
                (build_base_entities::<LineArrowType>).after(BuildingLabels::TriggerBuild),
            );
        }
    }
}

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push(ConsoleCommand {
        base: "pointArrow".to_string(),
        description: "Spawn an arrow with a specified duration and world position to point at."
            .to_string(),

        args: vec![
            ("x".to_string(), ConsoleArgVariant::Float),
            ("y".to_string(), ConsoleArgVariant::Float),
            ("z".to_string(), ConsoleArgVariant::Float),
            ("duration".to_string(), ConsoleArgVariant::Int),
        ],
    });
}
