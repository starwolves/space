
use bevy::{math::Vec3, prelude::{Entity, EventWriter, Mut, Query, ResMut, Transform}};
use bevy_rapier3d::{prelude::RigidBodyPosition};

use crate::space_core::{components::{connected_player::ConnectedPlayer, entity_data::EntityData, entity_updates::EntityUpdates, static_transform::StaticTransform, sensable::Sensable, visible_checker::VisibleChecker}, events::net::{net_load_entity::NetLoadEntity, net_unload_entity::NetUnloadEntity}, functions::{gridmap_functions::{world_to_cell_id}, isometry_to_transform::isometry_to_transform, load_entity_for_player::load_entity, unload_entity_for_player::unload_entity}, resources::{precalculated_fov_data::Vec2Int, world_fov::WorldFOV}};

pub fn visible_checker(
    mut query_visible_entities: Query<(
        Entity,
        &mut Sensable,
        Option<&StaticTransform>,
        Option<&RigidBodyPosition>,
        &EntityData,
        &EntityUpdates
    )>,
    query_visible_checker_entities_rigid : Query<(Entity, &VisibleChecker,  &RigidBodyPosition, &ConnectedPlayer)>,
    mut net_load_entity: EventWriter<NetLoadEntity>,
    mut net_unload_entity: EventWriter<NetUnloadEntity>,
    mut world_fov : ResMut<WorldFOV>,
) {
    
    for (
        entity,
        _visible_checker_component,
        visible_checker_rigid_body_position_component,
        visible_checker_connected_player_component
    ) in query_visible_checker_entities_rigid.iter() {
        let visible_checker_translation = visible_checker_rigid_body_position_component.position.translation;

        let visible_checker_translation_vec = Vec3::new(
            visible_checker_translation.x,
            visible_checker_translation.y,
            visible_checker_translation.z
        );


        for (
            visible_entity_id,
            mut visible_component,
            static_transform_component_option,
            rigid_body_position_component_option,
            entity_data_component,
            entity_updates_component
        ) in query_visible_entities.iter_mut() {

            let visible_entity_transform;

            let mut is_interpolated = false;

            match static_transform_component_option {
                Some(static_transform) => {
                    visible_entity_transform = static_transform.transform;
                }
                None => {

                    

                    is_interpolated=true;
                    let visible_entity_isometry =  rigid_body_position_component_option.unwrap().position;

                    visible_entity_transform = isometry_to_transform(visible_entity_isometry);

                }
            }

            visible_check(
                &mut visible_component,
                visible_entity_transform,
                visible_checker_translation_vec,
                entity,
                &mut net_load_entity,
                &mut net_unload_entity,
                visible_checker_connected_player_component.handle,
                entity_data_component,
                visible_entity_id.id(),
                is_interpolated,
                &entity_updates_component,
                &mut world_fov,
            );

            

        }


    }




}

const VIEW_DISTANCE : f32 = 90.;
const HEAR_DISTANCE : f32 = 60.;
const LIGHT_DISTANCE : f32 = 180.;

fn visible_check(
    visible_component : &mut Mut<Sensable>,
    visible_entity_transform : Transform,
    visible_checker_translation: Vec3,
    visible_checker_entity_id : Entity,
    net_load_entity : &mut EventWriter<NetLoadEntity>,
    net_unload_entity : &mut EventWriter<NetUnloadEntity>,
    visible_checker_handle : u32,
    visible_entity_data : &EntityData,
    visible_entity_id : u32,
    interpolated_transform : bool,
    visible_entity_updates_component : &EntityUpdates,
    world_fov : &mut ResMut<WorldFOV>,
) {

    let distance = visible_checker_translation.distance(visible_entity_transform.translation);
    let is_cached = distance < VIEW_DISTANCE;
    let can_cache;

    if visible_component.is_light ||
    visible_component.is_audible ||
    visible_component.always_sensed {
        can_cache = false;
    } else {
        can_cache = true;
    }

    let mut is_sensed = false;
    

    if visible_component.is_light == false &&
    visible_component.is_audible == false &&
    visible_component.always_sensed == false &&
    is_cached {

        let visible_checker_cell_id = world_to_cell_id(visible_checker_translation);

        let visible_checker_cell_id_2d = &Vec2Int{ x: visible_checker_cell_id.x, y: visible_checker_cell_id.z };

        let this_cell_fov_option = world_fov.data.get(visible_checker_cell_id_2d);

        match this_cell_fov_option {
            Some(this_cell_fov) => {

                let visible_entity_cell_id = world_to_cell_id(visible_entity_transform.translation);

                let visible_entity_cell_id_2d = &Vec2Int{ x: visible_entity_cell_id.x, y: visible_entity_cell_id.z };

                is_sensed = this_cell_fov.contains(visible_entity_cell_id_2d);

            },
            None => {
                if !world_fov.to_be_recalculated_priority.contains(visible_checker_cell_id_2d) {
                    world_fov.to_be_recalculated_priority.push(*visible_checker_cell_id_2d);
                }
                //is_visible = false;
                return;
            },
        }

    }

    if visible_component.is_light {
        is_sensed = distance < LIGHT_DISTANCE;
    }
    else if visible_component.is_audible {
        is_sensed = distance < HEAR_DISTANCE;
    }

    if visible_component.always_sensed == true {
        is_sensed = true;
    }

    let sensed_by_contains = visible_component.sensed_by.contains(&visible_checker_entity_id);
    let sensed_by_cached_contains = visible_component.sensed_by_cached.contains(&visible_checker_entity_id);

    if is_sensed == false {

        let unload_entirely;

        if can_cache {
            unload_entirely = !is_cached;
        } else {
            unload_entirely = true;
        }


        if sensed_by_contains {

            unload_entity(
                visible_checker_handle,
                visible_entity_id,
                net_unload_entity,
                unload_entirely
            );

            let index = visible_component.sensed_by.iter().position(|x| x == &visible_checker_entity_id).unwrap();
            visible_component.sensed_by.remove(index);

            if can_cache && !unload_entirely {
                if !sensed_by_cached_contains {
                    visible_component.sensed_by_cached.push(visible_checker_entity_id);
                }
            }
            
        } else if sensed_by_cached_contains && unload_entirely {
            unload_entity(
                visible_checker_handle,
                visible_entity_id,
                net_unload_entity,
                unload_entirely
            );
            let index = visible_component.sensed_by_cached.iter().position(|x| x == &visible_checker_entity_id).unwrap();
            visible_component.sensed_by_cached.remove(index);
        } else if !sensed_by_contains && !sensed_by_cached_contains {
            if can_cache && !unload_entirely {
                unload_entity(
                    visible_checker_handle,
                    visible_entity_id,
                    net_unload_entity,
                    unload_entirely
                );
                visible_component.sensed_by_cached.push(visible_checker_entity_id);
            }
        }



        



    } else {

        if !sensed_by_contains {
            visible_component.sensed_by.push(visible_checker_entity_id);
            load_entity(
                &visible_entity_updates_component.updates,
                visible_entity_transform,
                interpolated_transform,
                net_load_entity,
                visible_checker_handle,
                visible_entity_data,
                visible_entity_updates_component,
                visible_entity_id,
                true
            );
    
        }
        
        if sensed_by_cached_contains {
            let index = visible_component.sensed_by_cached.iter().position(|x| x == &visible_checker_entity_id).unwrap();
            visible_component.sensed_by_cached.remove(index);
        }


    }


}
