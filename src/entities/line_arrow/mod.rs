use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::core::{
    console_commands::{resources::AllConsoleCommands, ConsoleCommandsLabels},
    entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
        spawn::{summon_base_entity, SpawnEvent},
    },
    networking::resources::ConsoleCommandVariant,
    StartupLabels, SummoningLabels,
};

use self::{
    spawn::{default_line_arrow, summon_line_arrow, LineArrowSummoner, LINE_ARROW_ENTITY_NAME},
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
        .add_system(entity_console_commands.label(SummoningLabels::TriggerSummon))
        .add_startup_system(content_initialization.before(StartupLabels::InitEntities))
        .add_system((summon_base_entity::<LineArrowSummoner>).after(SummoningLabels::TriggerSummon))
        .add_system(summon_line_arrow::<LineArrowSummoner>.after(SummoningLabels::TriggerSummon))
        .add_event::<SpawnEvent<LineArrowSummoner>>()
        .add_system(
            (default_line_arrow)
                .label(SummoningLabels::DefaultSummon)
                .after(SummoningLabels::NormalSummon),
        );
    }
}

pub struct PointArrowPlugin;

impl Plugin for PointArrowPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(point_arrow).add_system(
            (summon_base_entity::<LineArrowSummoner>).after(SummoningLabels::TriggerSummon),
        );
    }
}

pub fn initialize_console_commands(mut commands: ResMut<AllConsoleCommands>) {
    commands.list.push((
        "pointArrow".to_string(),
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
        name: LINE_ARROW_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
