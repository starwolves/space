use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Entity, EventWriter, Mut, Query, Res, Transform}};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::space_core::{components::{connected_player::ConnectedPlayer, entity_data::EntityData, entity_updates::EntityUpdates, static_transform::StaticTransform, sensable::Sensable, visible_checker::VisibleChecker}, events::net::{net_load_entity::NetLoadEntity, net_unload_entity::NetUnloadEntity}, functions::{isometry_to_transform::isometry_to_transform, load_entity_for_player::load_entity, unload_entity_for_player::unload_entity}, structs::{
        network_messages::{
            EntityUpdateData
        }
    }};

pub fn visible_checker(
    mut query_visible_entities: Query<(
        Entity,
        &mut Sensable,
        Option<&StaticTransform>,
        Option<&RigidBodyHandleComponent>,
        &EntityData,
        &EntityUpdates
    )>,
    query_visible_checker_entities_rigid : Query<(Entity, &VisibleChecker,  &RigidBodyHandleComponent, &ConnectedPlayer)>,
    rigid_bodies: Res<RigidBodySet>,
    mut net_load_entity: EventWriter<NetLoadEntity>,
    mut net_unload_entity: EventWriter<NetUnloadEntity>,
) {

    for (
        entity,
        _visible_checker_component,
        visible_checker_rigid_body_handle_component,
        visible_checker_connected_player_component
    ) in query_visible_checker_entities_rigid.iter() {

        let visible_checker_translation = rigid_bodies.get(visible_checker_rigid_body_handle_component.handle())
        .expect("visible_checker.rs rigidbody handle was not present in RigidBodySet resource.")
        .position().translation;

        let visible_checker_translation_vec = Vec3::new(
            visible_checker_translation.x,
            visible_checker_translation.y,
            visible_checker_translation.z
        );


        for (
            visible_entity_id,
            mut visible_component,
            static_transform_component_option,
            rigid_body_handle_component_option,
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
                    let visible_entity_isometry = 
                    rigid_bodies.get(
                        rigid_body_handle_component_option
                        .expect("visible_checker.rs tried to check an entity that does not have any transform components.")
                        .handle()
                    )
                    .expect("visible_checker.rs rigidbody handle was not present in RigidBodySet resource (1)")
                    .position();

                    visible_entity_transform = isometry_to_transform(*visible_entity_isometry);

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
                &entity_updates_component.updates,
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
    visible_entity_updates : &HashMap<String,HashMap<String, EntityUpdateData>>
) {

    let mut is_visible = !(visible_checker_translation.distance(visible_entity_transform.translation) > 90.);

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
                visible_entity_updates,
                visible_entity_transform,
                interpolated_transform,
                net_load_entity,
                visible_checker_handle,
                visible_entity_data,
                visible_entity_id
            );
    
    
        }


    }


}
