use std::env;

use bevy::time::FixedTimestep;
use resources::labels::{ActionsLabels, MapLabels, StartupLabels, UpdateLabels};

use crate::diffusion::AtmosphericsResource;
use crate::examine_events::examine_map_atmos;
use crate::init::startup_atmospherics;
use crate::remove_cell_atmos_event::remove_cell_atmos_event;

use super::{
    diffusion::{atmos_diffusion, RigidBodyForcesAccumulation, DIFFUSION_STEP},
    effects::atmos_effects,
    map_events::{atmospherics_map, atmospherics_map_hover},
    notices::atmospherics_notices,
    rigidbody_forces::{rigidbody_forces_physics, rigidbody_pawn_forces_accumulation},
    sensing_ability::atmospherics_sensing_ability,
    zero_gravity::zero_gravity,
};

use bevy::prelude::{App, CoreStage, IntoSystemDescriptor, Plugin, SystemLabel, SystemSet};

pub const ATMOS_LABEL: &str = "fixed_timestep_map_atmos";
pub const ATMOS_DIFFUSION_LABEL: &str = "fixed_timestep_atmos";

pub struct AtmosphericsPlugin;

impl Plugin for AtmosphericsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
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
                .add_system_set(
                    SystemSet::new()
                        .with_run_criteria(
                            FixedTimestep::step(1. / DIFFUSION_STEP)
                                .with_label(ATMOS_DIFFUSION_LABEL),
                        )
                        .with_system(atmos_diffusion.label(AtmosphericsLabels::Diffusion))
                        .with_system(
                            atmos_effects
                                .after(AtmosphericsLabels::Diffusion)
                                .label(AtmosphericsLabels::Effects),
                        )
                        .with_system(
                            rigidbody_pawn_forces_accumulation.after(AtmosphericsLabels::Effects),
                        ),
                )
                .init_resource::<RigidBodyForcesAccumulation>()
                .add_startup_system(
                    startup_atmospherics
                        .label(StartupLabels::InitAtmospherics)
                        .after(StartupLabels::BuildGridmap),
                )
                .add_system(examine_map_atmos.after(ActionsLabels::Action));
        }
    }
}

/// Atmospherics systems ordering label.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum AtmosphericsLabels {
    Diffusion,
    Effects,
}
