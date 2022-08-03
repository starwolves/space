use api::{
    data::{PostUpdateLabels, StartupLabels, SummoningLabels, UpdateLabels},
    examinable::RichName,
    gridmap::{ExamineMapMessage, GridmapData, GridmapDetails1, GridmapMain, RemoveCell},
    pawn::SpawnPoints,
};
use bevy::{
    prelude::{App, ParallelSystemDescriptorCoercion, Plugin, SystemSet},
    time::FixedTimestep,
};
use entity::entity_data::INTERPOLATION_LABEL1;
use networking::messages::net_system;

use crate::init::{startup_build_map, startup_map_cells, startup_misc_resources};

use super::{
    events::{gridmap_updates, remove_cell},
    fov::{projectile_fov, senser_update_fov, DoryenMap},
    net::{NetGridmapUpdates, NetProjectileFOV},
    sensing_ability::gridmap_sensing_ability,
};

#[allow(dead_code)]
pub struct Details1CellProperties {
    pub id: i64,
    pub name: RichName,
    pub description: String,
}

impl Default for Details1CellProperties {
    fn default() -> Self {
        Self {
            id: 0,
            name: Default::default(),
            description: "".to_string(),
        }
    }
}

pub struct GridmapPlugin;

impl Plugin for GridmapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridmapDetails1>()
            .init_resource::<GridmapData>()
            .init_resource::<DoryenMap>()
            .init_resource::<SpawnPoints>()
            .add_event::<NetGridmapUpdates>()
            .add_system(senser_update_fov)
            .add_system(projectile_fov)
            .add_system(remove_cell.label(UpdateLabels::DeconstructCell))
            .add_event::<NetProjectileFOV>()
            .add_event::<RemoveCell>()
            .add_startup_system(startup_misc_resources.label(StartupLabels::MiscResources))
            .add_startup_system(
                startup_map_cells
                    .label(StartupLabels::InitDefaultGridmapData)
                    .label(SummoningLabels::TriggerSummon)
                    .after(StartupLabels::MiscResources),
            )
            .init_resource::<GridmapDetails1>()
            .add_startup_system(
                startup_build_map
                    .label(StartupLabels::BuildGridmap)
                    .after(StartupLabels::InitDefaultGridmapData),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 4.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(gridmap_updates),
            )
            .init_resource::<GridmapMain>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetProjectileFOV>)
                    .with_system(net_system::<NetGridmapUpdates>),
            )
            .add_system(gridmap_sensing_ability)
            .add_event::<ExamineMapMessage>();
    }
}
use bevy::app::CoreStage::PostUpdate;
