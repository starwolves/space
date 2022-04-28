use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::core::{
    console_commands::{resources::ConsoleCommands, ConsoleCommandsLabels},
    entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
    },
    networking::resources::ConsoleCommandVariant,
    space_plugin::StartupLabels,
};

use self::{
    spawn::LineArrowBundle,
    systems::{entity_console_commands, point_arrow},
};

pub mod components;
pub mod spawn;
pub mod systems;

pub struct LineArrowPlugin;

impl Plugin for LineArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            initialize_console_commands
                .before(ConsoleCommandsLabels::Finalize)
                .label(StartupLabels::ConsoleCommands),
        )
        .add_system(entity_console_commands)
        .add_startup_system(content_initialization.before(StartupLabels::InitEntities));
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
        "point_arrow".to_string(),
        "Spawn an arrow with a specified duration and world position to point at.".to_string(),
        vec![
            ("x".to_string(), ConsoleCommandVariant::Float),
            ("y".to_string(), ConsoleCommandVariant::Float),
            ("z".to_string(), ConsoleCommandVariant::Float),
            ("duration".to_string(), ConsoleCommandVariant::Int),
        ],
    ));
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: "lineArrow".to_string(),
        id: entity_data.get_id_inc(),
        spawn_function: Box::new(LineArrowBundle::spawn),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
