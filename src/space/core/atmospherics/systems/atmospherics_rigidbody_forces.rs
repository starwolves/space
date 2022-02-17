use bevy::{
    math::Vec3,
    prelude::{Query, Res},
};
use bevy_rapier3d::prelude::{RigidBodyForcesComponent, RigidBodyPositionComponent};

use crate::space::core::{
    atmospherics::{functions::get_atmos_index, resources::AtmosphericsResource},
    gridmap::{
        functions::gridmap_functions::world_to_cell_id,
        resources::{Vec2Int, FOV_MAP_WIDTH},
    },
    pawn::components::Pawn,
};

const ATMOSPHERICS_FORCES_SENSITIVITY_PAWN: f32 = 120.;
const ATMOSPHERICS_FORCES_MAX_PAWN: f32 = 110.;

const ATMOSPHERICS_FORCES_SENSITIVITY: f32 = 20.;
const ATMOSPHERICS_FORCES_MAX: f32 = 11.;

pub fn atmospherics_rigidbody_forces(
    mut rigid_bodies: Query<(
        &RigidBodyPositionComponent,
        &mut RigidBodyForcesComponent,
        Option<&Pawn>,
    )>,
    atmospherics_resource: Res<AtmosphericsResource>,
) {
    for (rigid_body_position_component, mut rigid_body_forces_component, pawn_component_option) in
        rigid_bodies.iter_mut()
    {
        let cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());

        if cell_id.x >= FOV_MAP_WIDTH as i16 / 2
            || cell_id.x <= -(FOV_MAP_WIDTH as i16 / 2)
            || cell_id.z >= FOV_MAP_WIDTH as i16 / 2
            || cell_id.z <= -(FOV_MAP_WIDTH as i16 / 2)
        {
            continue;
        }

        let cell_id2 = Vec2Int {
            x: cell_id.x,
            y: cell_id.z,
        };
        let self_atmospherics = atmospherics_resource
            .atmospherics
            .get(get_atmos_index(cell_id2))
            .unwrap();

        let mut atmos_force = Vec3::ZERO;

        let sensitivity;
        let forces_max;

        if pawn_component_option.is_none() {
            sensitivity = ATMOSPHERICS_FORCES_SENSITIVITY;
            forces_max = ATMOSPHERICS_FORCES_MAX;
        } else {
            sensitivity = ATMOSPHERICS_FORCES_SENSITIVITY_PAWN;
            forces_max = ATMOSPHERICS_FORCES_MAX_PAWN;
        }

        for j in 0..4 {
            let mut adjacent_cell_id = cell_id.clone();

            let mut force_direction = Vec3::ZERO;

            if j == 0 {
                adjacent_cell_id.x += 1;
                force_direction.x += 1.;
            } else if j == 1 {
                adjacent_cell_id.x -= 1;
                force_direction.x -= 1.;
            } else if j == 2 {
                adjacent_cell_id.z += 1;
                force_direction.z += 1.;
            } else {
                adjacent_cell_id.z -= 1;
                force_direction.z -= 1.;
            }

            if adjacent_cell_id.x >= FOV_MAP_WIDTH as i16 / 2
                || adjacent_cell_id.x <= -(FOV_MAP_WIDTH as i16 / 2)
                || adjacent_cell_id.z >= FOV_MAP_WIDTH as i16 / 2
                || adjacent_cell_id.z <= -(FOV_MAP_WIDTH as i16 / 2)
            {
                continue;
            }

            let adjacent_atmospherics = atmospherics_resource
                .atmospherics
                .get(get_atmos_index(Vec2Int {
                    x: adjacent_cell_id.x,
                    y: adjacent_cell_id.z,
                }))
                .unwrap();

            if adjacent_atmospherics.blocked {
                continue;
            }

            // Foreach adjacent cell build up a netto force.

            atmos_force -= force_direction
                * ((adjacent_atmospherics.get_pressure() - self_atmospherics.get_pressure())
                    * sensitivity);
        }

        atmos_force = atmos_force.clamp(
            Vec3::new(-forces_max, -forces_max, -forces_max),
            Vec3::new(forces_max, forces_max, forces_max),
        );

        let mut bevy_vec: Vec3 = rigid_body_forces_component.force.into();
        bevy_vec += atmos_force;
        rigid_body_forces_component.force = bevy_vec.into();
    }
}
