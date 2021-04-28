use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Entity, EventWriter, Mut, Query, Res, Transform}};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::space_core::{components::{connected_player::ConnectedPlayer, entity_data::EntityData, entity_updates::EntityUpdates, static_transform::StaticTransform, visible::Visible, visible_checker::VisibleChecker}, events::net::net_visible_checker::NetVisibleChecker, functions::{
        isometry_to_transform::isometry_to_transform
    }, structs::{
        network_messages::{
            ReliableServerMessage,
            EntityUpdateData
        }
    }};

pub fn visible_checker(
    mut query_visible_entities: Query<(
        Entity,
        &mut Visible,
        Option<&StaticTransform>,
        Option<&RigidBodyHandleComponent>,
        &EntityData,
        &EntityUpdates
    )>,
    query_visible_checker_entities_rigid : Query<(Entity, &VisibleChecker,  &RigidBodyHandleComponent, &ConnectedPlayer)>,
    rigid_bodies: Res<RigidBodySet>,
    mut net_visible_checker: EventWriter<NetVisibleChecker>,
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
                &mut net_visible_checker,
                visible_checker_connected_player_component.handle,
                entity_data_component,
                visible_entity_id.id(),
                is_interpolated,
                &entity_updates_component.updates
            );

        }


    }




}


fn visible_check(
    visible_component : &mut Mut<Visible>,
    visible_entity_transform : Transform,
    visible_checker_translation: Vec3,
    visible_checker_entity_id : Entity,
    net_visible_checker : &mut EventWriter<NetVisibleChecker>,
    visible_checker_handle : u32,
    visible_entity_data : &EntityData,
    visible_entity_id : u32,
    interpolated_transform : bool,
    visible_entity_updates : &HashMap<String,HashMap<String, EntityUpdateData>>
) {

    if visible_checker_translation.distance(visible_entity_transform.translation) > 90. {
        return;
    }

    if visible_component.sensed_by.contains(&visible_checker_entity_id) == false {
        
        visible_component.sensed_by.push(visible_checker_entity_id);

        // 1. Load in omni_lights with transform and omni light data.
        // 2. Load in player with transform.
        // 3. Ensure basic recognizable world shows up with own player and omni lights.
        // 4. Make GIProbes and ReflectionProbes permanently load in via a different system on first time world load.

        let mut hash_map = visible_entity_updates.clone();

        let transform_entity_update= EntityUpdateData::Transform(
            visible_entity_transform.translation,
            visible_entity_transform.rotation,
            visible_entity_transform.scale
        );

        match interpolated_transform {
            true => {
                let mut transform_hash_map = HashMap::new();
                transform_hash_map.insert("transform".to_string(), transform_entity_update);

                hash_map.insert("rawTransform".to_string(), transform_hash_map);
            },
            false => {
                let root_map_option = hash_map.get_mut(&".".to_string());

                match root_map_option {
                    Some(root_map) => {
                        root_map.insert("transform".to_string(), transform_entity_update);
                    }
                    None => {

                        let mut transform_hash_map = HashMap::new();
                        transform_hash_map.insert("transform".to_string(), transform_entity_update);

                        hash_map.insert(".".to_string(), transform_hash_map);
                    }
                }

                
            }
        }

        

        net_visible_checker.send(
            NetVisibleChecker {
                handle: visible_checker_handle,
                message: ReliableServerMessage::LoadEntity(
                    visible_entity_data.entity_class.clone(),
                    visible_entity_data.entity_type.clone(),
                    hash_map,
                    visible_entity_id,
                    true,
                    "main".to_string(),
                    "".to_string(),
                    false
                )
            }
        );

        


    }

}
