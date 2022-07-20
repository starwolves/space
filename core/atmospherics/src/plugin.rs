use networking::messages::net_system;
use shared::data::{MapLabels, PostUpdateLabels, StartupLabels, UpdateLabels};
use shared::examinable::ExamineLabels;

use crate::diffusion::AtmosphericsResource;
use crate::examine_events::{examine_map_atmos, NetAtmosphericsMapExamine};
use crate::init::startup_atmospherics;
use crate::remove_cell_atmos_event::remove_cell_atmos_event;

use super::{
    diffusion::{atmos_diffusion, RigidBodyForcesAccumulation, DIFFUSION_STEP},
    effects::atmos_effects,
    map_events::{atmospherics_map, atmospherics_map_hover},
    net::{NetAtmosphericsNotices, NetMapDisplayAtmospherics, NetMapHoverAtmospherics},
    notices::atmospherics_notices,
    rigidbody_forces::{rigidbody_forces_accumulation, rigidbody_forces_physics},
    sensing_ability::atmospherics_sensing_ability,
    zero_gravity::zero_gravity,
};

pub const ATMOS_LABEL: &str = "fixed_timestep_map_atmos";
pub const ATMOS_DIFFUSION_LABEL: &str = "fixed_timestep_atmos";

pub struct AtmosphericsPlugin;

impl Plugin for AtmosphericsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtmosphericsResource>()
            .add_system(atmospherics_map_hover.after(MapLabels::ChangeMode))
            .add_system(atmospherics_sensing_ability)
            .add_system(remove_cell_atmos_event.label(UpdateLabels::DeconstructCell))
            .add_system_to_stage(CoreStage::Update, rigidbody_forces_physics)
            .add_system_to_stage(CoreStage::Update, zero_gravity)
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 4.).with_label(ATMOS_LABEL))
                    .with_system(atmospherics_notices)
                    .with_system(atmospherics_map.after(MapLabels::ChangeMode)),
            )
            .add_event::<NetMapDisplayAtmospherics>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / DIFFUSION_STEP).with_label(ATMOS_DIFFUSION_LABEL),
                    )
                    .with_system(atmos_diffusion.label(AtmosphericsLabels::Diffusion))
                    .with_system(
                        atmos_effects
                            .after(AtmosphericsLabels::Diffusion)
                            .label(AtmosphericsLabels::Effects),
                    )
                    .with_system(rigidbody_forces_accumulation.after(AtmosphericsLabels::Effects)),
            )
            .init_resource::<RigidBodyForcesAccumulation>()
            .add_event::<NetMapHoverAtmospherics>()
            .add_startup_system(
                startup_atmospherics
                    .label(StartupLabels::InitAtmospherics)
                    .after(StartupLabels::BuildGridmap),
            )
            .add_event::<NetAtmosphericsNotices>()
            .add_event::<NetAtmosphericsMapExamine>()
            .add_system_set_to_stage(
                PostUpdate,
                SystemSet::new()
                    .after(PostUpdateLabels::VisibleChecker)
                    .label(PostUpdateLabels::Net)
                    .with_system(net_system::<NetMapDisplayAtmospherics>)
                    .with_system(net_system::<NetMapHoverAtmospherics>)
                    .with_system(net_system::<NetAtmosphericsNotices>)
                    .with_system(net_system::<NetAtmosphericsMapExamine>),
            )
            .add_system(examine_map_atmos.after(ExamineLabels::Default));
    }
}

use bevy::app::CoreStage::PostUpdate;
use bevy::{
    core::FixedTimestep,
    prelude::{App, CoreStage, ParallelSystemDescriptorCoercion, Plugin, SystemLabel, SystemSet},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum AtmosphericsLabels {
    Diffusion,
    Effects,
}
