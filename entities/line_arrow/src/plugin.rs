use basic_console_commands::register::register_basic_console_commands_for_type;
use bevy::prelude::{App, IntoSystemConfigs, Plugin, ResMut, Startup};
use console_commands::commands::{AllConsoleCommands, ConsoleCommand, ConsoleCommandsSet};
use entity::{entity_types::register_entity_type, spawn::build_base_entities};
use networking::server::ConsoleArgVariant;
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, PreUpdate, StartupSet, Update},
};

use crate::console_command::entity_console_commands;

use super::{
    console_command::expire_point_arrow,
    spawn::{build_line_arrows, LineArrowType},
};

pub struct LineArrowPlugin;

impl Plugin for LineArrowPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                entity_console_commands
                    .after(ConsoleCommandsSet::Input)
                    .in_set(BuildingSet::TriggerBuild),
            );
        }
        register_entity_type::<LineArrowType>(app);
        register_basic_console_commands_for_type::<LineArrowType>(app);
        app.add_systems(
            PreUpdate,
            (
                (build_base_entities::<LineArrowType>).in_set(BuildingSet::NormalBuild),
                build_line_arrows::<LineArrowType>.in_set(BuildingSet::NormalBuild),
            ),
        )
        .add_systems(
            Startup,
            initialize_console_commands
                .before(ConsoleCommandsSet::Finalize)
                .in_set(StartupSet::ConsoleCommands),
        );
    }
}

pub struct PointArrowPlugin;

impl Plugin for PointArrowPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                PreUpdate,
                (
                    expire_point_arrow,
                    (build_base_entities::<LineArrowType>).in_set(BuildingSet::NormalBuild),
                ),
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
