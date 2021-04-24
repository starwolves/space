use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Entity, EventWriter, Mut, Query, Res, Transform}};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::space_core::{
    components::{
        connected_player::ConnectedPlayer,
        static_transform::StaticTransform,
        visible::Visible, visible_checker::VisibleChecker,
        entity_data::EntityData
    },
    events::net_visible_checker::NetVisibleChecker,
    structs::{
        network_messages::{
            ReliableServerMessage,
            EntityUpdateData
        }
    },
    functions::{
        isometry_to_transform::isometry_to_transform
    }
};

pub fn visible_checker(
    mut query_visible_entities_static : Query<(Entity, &mut Visible, &StaticTransform, &EntityData)>,
    mut query_visible_entities_rigid : Query<(Entity, &mut Visible, &RigidBodyHandleComponent, &EntityData)>,
    query_visible_checker_entities_rigid : Query<(Entity, &VisibleChecker,  &RigidBodyHandleComponent, &ConnectedPlayer)>,
    rigid_bodies: Res<RigidBodySet>,
    mut net_visible_checker: EventWriter<NetVisibleChecker>,
) {

    // Loop through all relevant entities and automatically obtain their transform translations.
    // Checkers can "cached see" and "normal see" entities.
    // Checkers should store who they already (cached) see.
    // Checkers should now be able to see omni_lights, other players and themselves with self-extra logic.
    // When entities are (cached) seen and unseen by checkers, netcode them to appear/disappear/cache/uncache.
    // When we connect as a player we now see the full map in perfect condition, including ourselves.

    

    for (
        entity,
        _visible_checker_component,
        visible_checker_rigid_body_handle_component,
        visible_checker_connected_player_component
    ) in query_visible_checker_entities_rigid.iter() {

        let visible_checker_translation = rigid_bodies.get(visible_checker_rigid_body_handle_component.handle())
        .expect("visible_checker.rs rigidbody handle was not present in RigidBodySet resource")
        .position().translation;

        let visible_checker_translation_vec = Vec3::new(
            visible_checker_translation.x,
            visible_checker_translation.y,
            visible_checker_translation.z
        );


        for (
            visible_entity_id,
            mut visible_component,
            static_transform_component,
            entity_data_component
        ) in query_visible_entities_static.iter_mut() {

            visible_check(
                &mut visible_component,
                static_transform_component.transform,
                visible_checker_translation_vec,
                entity,
                &mut net_visible_checker,
                visible_checker_connected_player_component.handle,
                entity_data_component,
                visible_entity_id.id(),
                false
            );

        }

        


        for (
            visible_entity_id,
            mut visible_component,
            visible_rigid_body_handle_component,
            entity_data_component
        ) in query_visible_entities_rigid.iter_mut() {

            let visible_entity_isometry = rigid_bodies.get(visible_rigid_body_handle_component.handle())
            .expect("visible_checker.rs rigidbody handle was not present in RigidBodySet resource (1)")
            .position();

            let visible_entity_transform = isometry_to_transform(*visible_entity_isometry);

            visible_check(
                &mut visible_component,
                visible_entity_transform,
                visible_checker_translation_vec,
                entity,
                &mut net_visible_checker,
                visible_checker_connected_player_component.handle,
                entity_data_component,
                visible_entity_id.id(),
                true
            );

        }


    }




}


fn visible_check(
    visible_component : &mut Mut<Visible>,
    visible_entity_transform : Transform,
    _visible_checker_transform: Vec3,
    visible_checker_entity_id : Entity,
    net_visible_checker : &mut EventWriter<NetVisibleChecker>,
    visible_checker_handle : u32,
    visible_entity_data : &EntityData,
    visible_entity_id : u32,
    interpolated_transform : bool
) {

    if visible_component.sensed_by.contains(&visible_checker_entity_id) == false {
        
        visible_component.sensed_by.push(visible_checker_entity_id);

        // 1. Load in omni_lights with transform and omni light data.
        //
        // For each entity we need to add a entity_data component with nested hashmaps that have entityUpdates per node ready.
        // For each granular component we need to create a PostUpdate system that checks for component data changes,
        // when changes are detected in any specific component in its specific data listener system, we modify the hashmap
        // of the entity_data component so the Godot entity_updates remain up to date with the Bevy components data.
        // Then on a detected change in the entity_data hashmap component per entity we send new EntityUpdates to who see it.
        // And we also use this entity_data component hashmap data for LoadEntity calls for players.
        // This needs to be done with 0 frame lag.
        // Also when we have to LoadEntity and also apply entity_updates changes, we should do entity_updates changes before
        // real LoadEntity calls in the same frame to ensure it all stays in sync.
        //
        // 2. Ensure basic recognizable world shows up with own player and omni lights.
        // 3. Make GIProbes and ReflectionProbes permanently load in via a different system on first time world load.

        let mut hash_map = HashMap::new();

        let mut transform_hash_map = HashMap::new();
        transform_hash_map.insert("transform".to_string(), EntityUpdateData::Transform(
            visible_entity_transform.translation,
            visible_entity_transform.rotation,
            visible_entity_transform.scale
        ));



        if visible_entity_data.entity_class == "entity" {



        } else if visible_entity_data.entity_class == "omni_light" {

            let omni_hash_map = HashMap::new();

            hash_map.insert(".".to_string(), omni_hash_map);

        }

        match interpolated_transform {
            true => {
                hash_map.insert("raw_transform".to_string(), transform_hash_map);
            },
            false => {
                hash_map.insert(".".to_string(), transform_hash_map);
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
                    false
                )
            }
        );


    }

}
