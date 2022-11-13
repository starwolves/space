use bevy::{
    hierarchy::Parent,
    math::Vec3,
    prelude::{warn, Entity, Query, Res, With},
};
use bevy_rapier3d::{
    pipeline::QueryFilter,
    plugin::RapierContext,
    prelude::{Collider, InteractionGroups},
    rapier::prelude::Ray,
};
use entity::health::HealthComponent;
use pawn::pawn::REACH_DISTANCE;
use physics::physics::{get_bit_masks, ColliderGroup, ReachResult};

use crate::{
    events::Cell,
    grid::{GridmapData, GridmapMain},
};

use bevy_rapier3d::rapier::geometry::Group;

/// Check if entity can be reached by another entity with nothing in between to block it as a function.
#[cfg(feature = "server")]
pub fn can_reach_entity(
    query_pipeline: &bevy::prelude::Res<RapierContext>,

    mut start_point: Vec3,
    end_point: Vec3,
    target_entity: &Entity,
    reacher_entity: &Entity,
    health_entities_query: &Query<&HealthComponent>,
    cells_query: &Query<&Cell>,
    _world_cells: &Res<GridmapMain>,
    _gridmap_data: &Res<GridmapData>,
    no_result_is_valid: bool,
    collider_parents: &Query<&Parent, With<Collider>>,
) -> bool {
    start_point.y = 1.8;

    let direction = (end_point - start_point).normalize();

    let ray = Ray::new(start_point.into(), direction.into());
    let max_toi = REACH_DISTANCE;
    let solid = true;

    let collider_groups = get_bit_masks(ColliderGroup::Standard);
    let query_filter = QueryFilter::new().groups(InteractionGroups::new(
        Group::from_bits(collider_groups.0).unwrap(),
        Group::from_bits(collider_groups.1).unwrap(),
    ));
    let mut collided_entities = vec![];

    query_pipeline.intersections_with_ray(
        ray.origin.into(),
        ray.dir.into(),
        max_toi,
        solid,
        query_filter,
        |collided_entity, ray_intersection| {
            let parent_entity;
            match collider_parents.get(collided_entity) {
                Ok(s) => {
                    parent_entity = s.get();
                }
                Err(_rr) => {
                    parent_entity = collided_entity;
                }
            }

            if parent_entity == *reacher_entity {
                return true;
            }

            let hit_cell;

            match cells_query.get(parent_entity) {
                Ok(cell_id) => {
                    hit_cell = Some(cell_id.id);
                }
                Err(_rr) => {
                    hit_cell = None;
                }
            }

            let hit_entity;

            match health_entities_query.get(parent_entity) {
                Ok(h) => {
                    hit_entity = Some((parent_entity, h.health.is_reach_obstacle));
                }
                Err(_rr) => {
                    hit_entity = None;
                }
            }

            if hit_entity.is_none() && hit_cell.is_none() {
                return true;
            }

            collided_entities.push(ReachResult {
                distance: ray_intersection.toi,
                hit_entity: hit_entity,
                hit_cell: hit_cell,
            });

            true
        },
    );

    collided_entities.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
    collided_entities.reverse();

    let mut in_reach = false;
    let collided_entities_length = collided_entities.len() as i64;
    let mut this_i: i64 = -1;
    for reach_result in collided_entities.iter() {
        this_i += 1;
        match reach_result.hit_entity {
            Some((hit_entity, is_reach_obstacle)) => {
                if hit_entity == *target_entity {
                    in_reach = true;
                    break;
                } else if is_reach_obstacle {
                    break;
                }
            }
            None => {
                match reach_result.hit_cell {
                    Some(cell_id) => {
                        // Assume all gridmap main wall items are blockers, work with _world_cells and _gridmap_data if you want to change this.
                        if cell_id.y == 0 {
                            if no_result_is_valid && collided_entities_length - 1 == this_i {
                                in_reach = true;
                            }
                            break;
                        }
                    }
                    None => {
                        warn!("ReachResult only contained empty options.");
                    }
                }
            }
        }
    }

    in_reach
}
