use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use console_commands::commands::{AllConsoleCommands, ConsoleCommandsLabels};
use entity::{
    entity_data::initialize_entity_data,
    entity_types::init_entity_type,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::build_base_entities,
};
use networking::server::GodotVariant;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, StartupLabels},
};

use crate::console_command::entity_console_commands;

use super::{
    console_command::expire_point_arrow,
    spawn::{build_line_arrows, default_build_line_arrows, LineArrowType, LINE_ARROW_ENTITY_NAME},
};

pub struct LineArrowPlugin;

impl Plugin for LineArrowPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_startup_system(
                initialize_console_commands
                    .before(ConsoleCommandsLabels::Finalize)
                    .label(StartupLabels::ConsoleCommands),
            )
            .add_system(entity_console_commands.label(BuildingLabels::TriggerBuild));
        }
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system((build_base_entities::<LineArrowType>).after(BuildingLabels::TriggerBuild))
            .add_system(build_line_arrows::<LineArrowType>.after(BuildingLabels::TriggerBuild))
            .add_system(
                (default_build_line_arrows)
                    .label(BuildingLabels::DefaultBuild)
                    .after(BuildingLabels::NormalBuild),
            );
        init_entity_type::<LineArrowType>(app);
    }
}

pub struct PointArrowPlugin;

impl Plugin for PointArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(expire_point_arrow)
            .add_system((build_base_entities::<LineArrowType>).after(BuildingLabels::TriggerBuild));
    }
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: LINE_ARROW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
