use std::collections::HashMap;

use bevy::{
    math::Vec3,
    prelude::{Entity, Query, Res, ResMut},
};
use bevy_rapier3d::prelude::{
    RigidBodyForcesComponent, RigidBodyPositionComponent, RigidBodyVelocityComponent,
};

use crate::space::core::{
    atmospherics::{
        functions::get_atmos_index,
        resources::{AtmosphericsResource, RigidBodyForcesAccumulation},
    },
    gridmap::{
        functions::gridmap_functions::world_to_cell_id,
        resources::{GridmapMain, Vec2Int, FOV_MAP_WIDTH},
    },
    pawn::components::Pawn,
};

use super::diffusion::DIFFUSION_STEP;

const ATMOSPHERICS_FORCES_SENSITIVITY_PAWN: f32 = 50.;
const ATMOSPHERICS_FORCES_ACCELERATION_MAX_PAWN: f32 = 300.;

const ATMOSPHERICS_FORCES_SENSITIVITY: f32 = 1.;
const ATMOSPHERICS_FORCES_ACCELERATION_MAX: f32 = 8.;

const ATMOSHPERICS_MAX_VELOCITY: f32 = 10.;

const ATMOSPHERICS_PUSHING_UP_FORCE: f32 = 2.;

// Now this system must instead read from a shared resource or event reader of rigidbody_forces_accumulation.

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum AdjacentTileDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn rigidbody_forces_accumulation(
    mut rigid_bodies: Query<(
        Entity,
        &RigidBodyPositionComponent,
        &RigidBodyForcesComponent,
        Option<&Pawn>,
        &RigidBodyVelocityComponent,
    )>,
    atmospherics_resource: Res<AtmosphericsResource>,
    mut forces_accumulation: ResMut<RigidBodyForcesAccumulation>,
    gridmap_main: Res<GridmapMain>,
) {
    for (
        rigidbody_entity,
        rigid_body_position_component,
        rigid_body_forces_component,
        pawn_component_option,
        rigidbody_velocity_component,
    ) in rigid_bodies.iter_mut()
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
            sensitivity = ATMOSPHERICS_FORCES_SENSITIVITY * (64. / DIFFUSION_STEP as f32);
            forces_max = ATMOSPHERICS_FORCES_ACCELERATION_MAX;
        } else {
            sensitivity = ATMOSPHERICS_FORCES_SENSITIVITY_PAWN * (64. / DIFFUSION_STEP as f32);
            forces_max = ATMOSPHERICS_FORCES_ACCELERATION_MAX_PAWN;
        }

        let mut to_be_applied_forces: HashMap<AdjacentTileDirection, Vec3> = HashMap::new();

        let mut push_up = self_atmospherics.forces_push_up;

        for j in 0..4 {
            let mut adjacent_cell_id = cell_id.clone();

            let mut force_direction = Vec3::ZERO;

            let tile_direction;

            if j == 0 {
                adjacent_cell_id.x += 1;
                force_direction.x += 1.;
                tile_direction = AdjacentTileDirection::Right;
            } else if j == 1 {
                adjacent_cell_id.x -= 1;
                force_direction.x -= 1.;
                tile_direction = AdjacentTileDirection::Left;
            } else if j == 2 {
                adjacent_cell_id.z += 1;
                force_direction.z += 1.;
                tile_direction = AdjacentTileDirection::Up;
            } else {
                adjacent_cell_id.z -= 1;
                force_direction.z -= 1.;
                tile_direction = AdjacentTileDirection::Down;
            }

            to_be_applied_forces.insert(tile_direction.clone(), Vec3::ZERO);

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

            if adjacent_atmospherics.forces_push_up {
                push_up = true;
            }

            // Foreach adjacent cell build up a netto force.

            to_be_applied_forces.insert(
                tile_direction,
                force_direction
                    * ((adjacent_atmospherics.get_pressure() - self_atmospherics.get_pressure())
                        * sensitivity),
            );
        }

        let mut floor_tile = cell_id.clone();
        floor_tile.y = -1;

        match gridmap_main.data.get(&floor_tile) {
            Some(_) => {}
            None => {
                push_up = false;
            }
        }

        // Limit max velocity, bleed speed in a smart way, essentially we only ever move into two different directions at once. Two of the four available directions.
        let body_linear_velocity: Vec3 = rigidbody_velocity_component.linvel.into();
        let body_velocity = body_linear_velocity.length();

        let mut is_moving_up = false;
        let mut is_moving_down = false;
        let mut is_moving_left = false;
        let mut is_moving_right = false;

        if body_linear_velocity.y > 0. {
            is_moving_up = true;
        } else if body_linear_velocity.y < 0. {
            is_moving_down = true;
        }

        if body_linear_velocity.x > 0. {
            is_moving_right = true;
        } else if body_linear_velocity.x < 0. {
            is_moving_left = true;
        }

        let over_max_speed = body_velocity > ATMOSHPERICS_MAX_VELOCITY;

        if over_max_speed {
            if is_moving_left {
                let net_x_axis = to_be_applied_forces
                    .get(&AdjacentTileDirection::Left)
                    .unwrap()
                    .length();
                to_be_applied_forces.insert(AdjacentTileDirection::Left, Vec3::ZERO);

                let opposite_dir = to_be_applied_forces
                    .get_mut(&AdjacentTileDirection::Right)
                    .unwrap();
                opposite_dir.x -= net_x_axis;
                opposite_dir.x = opposite_dir.x.clamp(0., 100000000.);
            } else if is_moving_right {
                let net_x_axis = to_be_applied_forces
                    .get(&AdjacentTileDirection::Right)
                    .unwrap()
                    .length();
                to_be_applied_forces.insert(AdjacentTileDirection::Right, Vec3::ZERO);

                let opposite_dir = to_be_applied_forces
                    .get_mut(&AdjacentTileDirection::Left)
                    .unwrap();
                opposite_dir.x -= net_x_axis;
                opposite_dir.x = opposite_dir.x.clamp(0., 100000000.);
            }
            if is_moving_up {
                let net_y_axis = to_be_applied_forces
                    .get(&AdjacentTileDirection::Up)
                    .unwrap()
                    .length();
                to_be_applied_forces.insert(AdjacentTileDirection::Up, Vec3::ZERO);

                let opposite_dir = to_be_applied_forces
                    .get_mut(&AdjacentTileDirection::Down)
                    .unwrap();
                opposite_dir.z -= net_y_axis;
                opposite_dir.z = opposite_dir.z.clamp(0., 100000000.);
            } else if is_moving_down {
                let net_y_axis = to_be_applied_forces
                    .get(&AdjacentTileDirection::Down)
                    .unwrap()
                    .length();
                to_be_applied_forces.insert(AdjacentTileDirection::Down, Vec3::ZERO);

                let opposite_dir = to_be_applied_forces
                    .get_mut(&AdjacentTileDirection::Up)
                    .unwrap();
                opposite_dir.z -= net_y_axis;
                opposite_dir.z = opposite_dir.z.clamp(0., 100000000.);
            }
        }

        for force in to_be_applied_forces.values() {
            atmos_force -= *force;
        }

        // Limit acceleration
        atmos_force = atmos_force.clamp(
            Vec3::new(-forces_max, -forces_max, -forces_max),
            Vec3::new(forces_max, forces_max, forces_max),
        );

        if push_up && !over_max_speed {
            atmos_force.y = ATMOSPHERICS_PUSHING_UP_FORCE * (64. / DIFFUSION_STEP as f32);
        }

        let mut bevy_vec: Vec3 = rigid_body_forces_component.force.into();

        bevy_vec += atmos_force;

        match forces_accumulation.data.get_mut(&rigidbody_entity) {
            Some(accumulation) => {
                accumulation.push(bevy_vec);
            }
            None => {
                forces_accumulation.data.insert(rigidbody_entity, vec![]);
                forces_accumulation
                    .data
                    .get_mut(&rigidbody_entity)
                    .unwrap()
                    .push(bevy_vec);
            }
        }
    }
}
