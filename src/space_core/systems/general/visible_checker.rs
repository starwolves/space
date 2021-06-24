
use bevy::{math::Vec3, prelude::{Entity, EventWriter, Mut, Query, Res, Transform, warn}};
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
    world_fov : Res<WorldFOV>,
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
                &world_fov,
            );

        }


    }




}


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
    world_fov : &Res<WorldFOV>,
) {

    let mut is_visible = !(visible_checker_translation.distance(visible_entity_transform.translation) > 90.);

    if is_visible {

        let visible_checker_cell_id = world_to_cell_id(visible_checker_translation);

        //is_visible = world_fov.data.contains_key( &Vec2Int{ x: this_cell_id.x, y: this_cell_id.z }  );

        let this_cell_fov_option = world_fov.data.get(&Vec2Int{ x: visible_checker_cell_id.x, y: visible_checker_cell_id.z });

        match this_cell_fov_option {
            Some(this_cell_fov) => {

                let visible_entity_cell_id = world_to_cell_id(visible_entity_transform.translation);

                is_visible = this_cell_fov.contains(&Vec2Int{ x: visible_entity_cell_id.x, y: visible_entity_cell_id.z });

            },
            None => {
                warn!("Requested world_fov out of range data.");
                is_visible = false;
            },
        }

    }

    if visible_component.always_sensed == true {
        is_visible = true;
    }

    if is_visible == false {


        if visible_component.sensed_by.contains(&visible_checker_entity_id) == true {

            unload_entity(
                visible_checker_handle,
                visible_entity_id,
                net_unload_entity
            );

            let index = visible_component.sensed_by.iter().position(|x| x == &visible_checker_entity_id).unwrap();

            visible_component.sensed_by.remove(index);

        }



        



    } else {


        if visible_component.sensed_by.contains(&visible_checker_entity_id) == false {
        
            visible_component.sensed_by.push(visible_checker_entity_id);
            
            load_entity(
                &visible_entity_updates_component.updates,
                visible_entity_transform,
                interpolated_transform,
                net_load_entity,
                visible_checker_handle,
                visible_entity_data,
                visible_entity_updates_component,
                visible_entity_id
            );
    
    
        }


    }


}
