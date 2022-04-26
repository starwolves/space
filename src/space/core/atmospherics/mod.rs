pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

use bevy_app::{App, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::{
    schedule::{ParallelSystemDescriptorCoercion, SystemSet},
    system::{Res, ResMut},
};
use bevy_log::info;
use bevy_rapier3d::physics::{PhysicsStages, PhysicsSystems};

use crate::space::{
    core::{
        atmospherics::{
            functions::get_atmos_index,
            resources::{Atmospherics, AtmosphericsResource, DEFAULT_INTERNAL_AMOUNT},
        },
        gridmap::resources::{GridmapMain, Vec2Int, Vec3Int, FOV_MAP_WIDTH},
    },
    AtmosphericsLabels, MapLabels, PostUpdateLabels, StartupLabels, ATMOS_DIFFUSION_LABEL,
    ATMOS_LABEL,
};

use self::{
    events::{
        net_system, NetAtmosphericsNotices, NetMapDisplayAtmospherics, NetMapHoverAtmospherics,
    },
    resources::{MapHolders, RigidBodyForcesAccumulation},
    systems::{
        diffusion::{atmos_diffusion, DIFFUSION_STEP},
        effects::atmos_effects,
        map::atmospherics_map,
        map_hover::atmospherics_map_hover,
        notices::atmospherics_notices,
        rigidbody_forces_atmospherics::rigidbody_forces_accumulation,
        rigidbody_forces_physics::rigidbody_forces_physics,
        sensing_ability::atmospherics_sensing_ability,
        zero_gravity::zero_gravity,
    },
};

use super::gridmap::resources::GridmapData;

pub fn startup_atmospherics(
    gridmap_main: Res<GridmapMain>,
    mut atmospherics: ResMut<AtmosphericsResource>,
    gridmap_main_data: Res<GridmapData>,
) {
    // Setup atmospherics.
    let default_x = FOV_MAP_WIDTH as i16 / 2;
    let default_z = FOV_MAP_WIDTH as i16 / 2;

    let mut current_cell_id = Vec2Int {
        x: -default_x - 1,
        y: -default_z,
    };

    let mut vacuum_cells: u32 = 0;

    for _i in 0..FOV_MAP_WIDTH * FOV_MAP_WIDTH {
        current_cell_id.x += 1;

        if current_cell_id.x > default_x {
            current_cell_id.x = -default_x;
            current_cell_id.y += 1;
        }

        let blocked;
        let push_up;

        match gridmap_main.grid_data.get(&Vec3Int {
            x: current_cell_id.x,
            y: 0,
            z: current_cell_id.y,
        }) {
            Some(cell_data) => {
                let properties = gridmap_main_data
                    .main_cell_properties
                    .get(&cell_data.item)
                    .unwrap();
                blocked = properties.atmospherics_blocker;
                push_up = properties.atmospherics_pushes_up;
            }
            None => {
                blocked = false;
                push_up = false;
            }
        }

        let internal;

        match gridmap_main.grid_data.get(&Vec3Int {
            x: current_cell_id.x,
            y: -1,
            z: current_cell_id.y,
        }) {
            Some(_cell_data) => {
                internal = true;
            }
            None => {
                internal = false;
            }
        }

        if internal {
            atmospherics.atmospherics[get_atmos_index(current_cell_id)] =
                Atmospherics::new_internal(blocked, push_up);
        } else {
            let flags = vec!["default_vacuum".to_string()];
            atmospherics.atmospherics[get_atmos_index(current_cell_id)] = Atmospherics {
                blocked,
                flags,
                forces_push_up: push_up,
                ..Default::default()
            };
            vacuum_cells += 1;
        }
    }

    let internal_cells_count = (FOV_MAP_WIDTH * FOV_MAP_WIDTH - vacuum_cells as usize) as f32;

    let internal_m3 = internal_cells_count / 2.;

    let internal_mol = internal_cells_count * DEFAULT_INTERNAL_AMOUNT;
    let internal_mega_mol = internal_mol * 1e-6;
    let internal_liter = internal_m3 * 1000.;
    let internal_kilo_liter = internal_liter * 0.001;

    info!(
        "Loaded {:.1}Mmol atmosphere into {:.1}kl ship.",
        internal_mega_mol, internal_kilo_liter
    );
}

pub struct AtmosphericsPlugin;

impl Plugin for AtmosphericsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AtmosphericsResource>()
            .add_system(atmospherics_map_hover.after(MapLabels::ChangeMode))
            .add_system(atmospherics_sensing_ability)
            .add_system_to_stage(
                PhysicsStages::SyncTransforms,
                rigidbody_forces_physics.after(PhysicsSystems::SyncTransforms),
            )
            .add_system_to_stage(
                PhysicsStages::SyncTransforms,
                zero_gravity.after(PhysicsSystems::SyncTransforms),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(1. / 4.).with_label(ATMOS_LABEL))
                    .with_system(atmospherics_notices)
                    .with_system(atmospherics_map.after(MapLabels::ChangeMode)),
            )
            .add_event::<NetMapDisplayAtmospherics>()
            .init_resource::<MapHolders>()
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
            .add_system_to_stage(
                PostUpdate,
                net_system.after(PostUpdateLabels::VisibleChecker),
            );
    }
}
use bevy_app::CoreStage::PostUpdate;
