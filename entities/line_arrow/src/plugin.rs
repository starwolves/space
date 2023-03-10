use basic_console_commands::register::register_basic_console_commands_for_type;
use bevy::prelude::{App, IntoSystemConfig, Plugin, ResMut};
use console_commands::commands::{AllConsoleCommands, ConsoleCommandsLabels};
use entity::{entity_types::register_entity_type, spawn::build_base_entities};
use networking::server::GodotVariant;
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
            app.add_startup_system(
                initialize_console_commands
                    .before(ConsoleCommandsLabels::Finalize)
                    .in_set(StartupLabels::ConsoleCommands),
            )
            .add_system(entity_console_commands.in_set(BuildingLabels::TriggerBuild));
        }
        register_entity_type::<LineArrowType>(app);
        register_basic_console_commands_for_type::<LineArrowType>(app);
        app.add_system((build_base_entities::<LineArrowType>).after(BuildingLabels::TriggerBuild))
            .add_system(build_line_arrows::<LineArrowType>.after(BuildingLabels::TriggerBuild));
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
    commands.list.push((
        "pointArrow".to_string(),
        "Spawn an arrow with a specified duration and world position to point at.".to_string(),
        vec![
            ("x".to_string(), GodotVariant::Float),
            ("y".to_string(), GodotVariant::Float),
            ("z".to_string(), GodotVariant::Float),
            ("duration".to_string(), GodotVariant::Int),
        ],
    ));
}
