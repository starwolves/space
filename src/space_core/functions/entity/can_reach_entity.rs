use bevy::{math::Vec3, prelude::{Entity, Query, Res, warn}};
use bevy_rapier3d::prelude::{InteractionGroups, IntoEntity, QueryPipeline, QueryPipelineColliderComponentsQuery, QueryPipelineColliderComponentsSet, Ray};

use crate::space_core::{components::{cell::Cell, health::Health}, resources::{doryen_fov::Vec3Int, gridmap_data::GridmapData, gridmap_main::GridmapMain}};

use super::collider_interaction_groups::{ColliderGroup, get_bit_masks};

pub const REACH_DISTANCE : f32 = 3.;

struct ReachResult {
    distance : f32,
    hit_entity : Option<(Entity, bool)>,
    hit_cell : Option<Vec3Int>,
}

pub fn can_reach_entity(
    query_pipeline: &Res<QueryPipeline>,
    collider_query: &QueryPipelineColliderComponentsQuery,
    mut start_point: Vec3,
    end_point : Vec3,
    target_entity : &Entity,
    reacher_entity : &Entity,
    health_entities_query : &Query<&Health>,
    cells_query : &Query<&Cell>,
    _world_cells : &Res<GridmapMain>,
    _gridmap_data : &Res<GridmapData>,
    no_result_is_valid : bool,
) -> bool {

    start_point.y = 1.8;

    let direction = (end_point - start_point).normalize();

    let collider_set = QueryPipelineColliderComponentsSet(&collider_query);

    let ray = Ray::new(start_point.into(), direction.into());
    let max_toi = REACH_DISTANCE;
    let solid = true;

    let collider_groups = get_bit_masks(ColliderGroup::Standard);
    let interaction_groups = InteractionGroups::new(collider_groups.0,collider_groups.1);

    let mut collided_entities = vec![];

    query_pipeline.intersections_with_ray(
        &collider_set, &ray, max_toi, solid, interaction_groups, None,
        |handle, ray_intersection| {
            let collided_entity = handle.entity(); 

            if collided_entity == *reacher_entity {
                return true;
            }

            let hit_cell;

            match cells_query.get(collided_entity) {
                Ok(cell_id) => {
                    hit_cell = Some(cell_id.id);
                },
                Err(_rr) => {
                    hit_cell = None;
                },
            }

            let hit_entity;

            match health_entities_query.get(collided_entity) {
                Ok( h) => {
                    hit_entity = Some((collided_entity, h.is_reach_obstacle));
                },
                Err(_rr) => {
                    hit_entity = None;
                },
            }

            if hit_entity.is_none() && hit_cell.is_none() {
                return true;
            }

            collided_entities.push(
                ReachResult {
                    distance: ray_intersection.toi,
                    hit_entity: hit_entity,
                    hit_cell: hit_cell,
                }
            );

            true
        }
    );

    collided_entities.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
    collided_entities.reverse();
    
    let mut in_reach = false;
    let collided_entities_length = collided_entities.len() as i64;
    let mut this_i : i64 = -1;
    for reach_result in collided_entities.iter() {
        this_i+=1;
        match reach_result.hit_entity {
            Some((hit_entity, is_reach_obstacle)) => {

                if hit_entity == *target_entity {
                    in_reach = true;
                    break;
                } else if is_reach_obstacle {
                    break;
                }

            },
            None => {

                match reach_result.hit_cell {
                    Some(cell_id) => {

                        // Assume all gridmap main wall items are blockers, work with _world_cells and _gridmap_data if you want to change this.
                        if cell_id.y == 0 {
                            
                            if no_result_is_valid && collided_entities_length-1 == this_i {
                                in_reach=true;
                            }
                            break;
                        }
                    },
                    None => {
                        warn!("ReachResult only contained empty options.");
                    },
                }

            },
        }

    }    

    in_reach

}
