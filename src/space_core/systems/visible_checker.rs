use std::collections::HashMap;

use bevy::{math::Vec3, prelude::{Entity, EventWriter, Mut, Query, Res}};
use bevy_rapier3d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::space_core::{components::{
    connected_player::ConnectedPlayer,
    static_transform::StaticTransform,
    visible::Visible, visible_checker::VisibleChecker,
    entity_data::EntityData
    }, events::net_visible_checker::NetVisibleChecker, structs::{
        network_messages::{
            ReliableServerMessage
        }
    }};

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
                Vec3::new(
                    static_transform_component.transform.translation.x,
                    static_transform_component.transform.translation.y,
                    static_transform_component.transform.translation.z
                ),
                visible_checker_translation_vec,
                entity,
                &mut net_visible_checker,
                visible_checker_connected_player_component.handle,
                entity_data_component,
                visible_entity_id.id()
            );

        }

        


        for (
            visible_entity_id,
            mut visible_component,
            visible_rigid_body_handle_component,
            entity_data_component
        ) in query_visible_entities_rigid.iter_mut() {

            let visible_entity_translation = rigid_bodies.get(visible_rigid_body_handle_component.handle())
            .expect("visible_checker.rs rigidbody handle was not present in RigidBodySet resource (1)")
            .position().translation;

            let visible_entity_translation_vec = Vec3::new(
                visible_entity_translation.x,
                visible_entity_translation.y,
                visible_entity_translation.z
            );

            visible_check(
                &mut visible_component,
                visible_entity_translation_vec,
                visible_checker_translation_vec,
                entity,
                &mut net_visible_checker,
                visible_checker_connected_player_component.handle,
                entity_data_component,
                visible_entity_id.id()
            );

        }


    }




}


fn visible_check(
    visible_component : &mut Mut<Visible>,
    _visible_entity_transform : Vec3,
    _visible_checker_transform: Vec3,
    visible_checker_entity_id : Entity,
    net_visible_checker : &mut EventWriter<NetVisibleChecker>,
    visible_checker_handle : u32,
    visible_entity_data : &EntityData,
    visible_entity_id : u32
) {

    if visible_component.sensed_by.contains(&visible_checker_entity_id) == false {
        
        visible_component.sensed_by.push(visible_checker_entity_id);

        // 1. Add persistent_entity_updates component to Visible entities.
        // 2. Make client spawn itself and omni_lights for a basic recognizable world.
        // 3. Make GIProbes and ReflectionProbes permanently load in via a different system on first time world load.

        let hash_map = HashMap::new();

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
