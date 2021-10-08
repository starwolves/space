use bevy::{math::{Quat, Vec3}, prelude::{Commands, EventReader, EventWriter, Query, Res, ResMut, Transform}};
use bevy_rapier3d::prelude::{Cuboid, InteractionGroups, QueryPipeline, QueryPipelineColliderComponentsQuery, QueryPipelineColliderComponentsSet};

use crate::space_core::{components::{cell::Cell, examinable::Examinable, health::{DamageType, Health, HitResult}, inventory_item::{CombatType}}, events::{general::attack::Attack, net::net_chat_message::NetChatMessage}, functions::{entity::collider_interaction_groups::{ColliderGroup, get_bit_masks}, gridmap::get_cell_name::get_cell_name}, resources::{gridmap_main::GridmapMain, handle_to_entity::HandleToEntity, sfx_auto_destroy_timers::SfxAutoDestroyTimers}};

use bevy_rapier3d::physics::IntoEntity;

pub fn attack(

    mut attack_events : EventReader<Attack>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut health_query : Query<(&mut Health, &Examinable)>,
    mut world_cells : ResMut<GridmapMain>,
    physics_cells : Query<&Cell>,
    mut net_message_event: EventWriter<NetChatMessage>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands : Commands,
    mut sfx_auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,

) {

    for attack_event in attack_events.iter() {

        let direction_additive = Vec3::new(
            -attack_event.angle.cos(),
            0.,
            attack_event.angle.sin(),
        );

        let sound_transform = Transform{ translation: attack_event.position, rotation: Quat::IDENTITY, scale: Vec3::ONE };

        match &attack_event.combat_type {
            CombatType::MeleeDirect => {

                let collider_groups = get_bit_masks(ColliderGroup::Standard);
                let interaction_groups = InteractionGroups::new(collider_groups.0,collider_groups.1);

                let additive = direction_additive * attack_event.range;

                let mut hit_anything = false;

                query_pipeline.intersections_with_shape(
                    &QueryPipelineColliderComponentsSet(&collider_query),
                    &(
                        Vec3::new(
                            attack_event.position.x, 
                            1.0, 
                            attack_event.position.z,
                        )
                        -
                        additive,
                        Quat::from_rotation_y(attack_event.angle)).into(),
                    &Cuboid::new(Vec3::new(attack_event.range, 1.0, 0.3).into()),
                    interaction_groups,
                    None, 
                    |collider_handle| {


                        let collider_entity = collider_handle.entity();

                        if collider_entity == attack_event.attacker_entity {
                            return true;
                        }

                        match attack_event.weapon_entity {
                            Some(weapon_entity) => {
                                if collider_entity == weapon_entity {
                                    return true;
                                }
                            },
                            None => {},
                        }

                        let mut hit_target = false;
                        let mut hit_result = HitResult::Missed;

                        match health_query.get_mut(collider_entity) {
                            Ok((mut health_component, examinable_component)) => {

                                hit_target = true;

                                hit_result = health_component.apply_damage(
                                    &attack_event.targetted_limb, 
                                    &attack_event.damage_model,
                                    &mut net_message_event,
                                        &handle_to_entity,
                                        &attack_event.attacker_sensed_by,
                                    &attack_event.attacker_sensed_by_cached,
                                    attack_event.position.into(),
                                        &attack_event.attacker_name,
                                        &examinable_component.name,
                                        &DamageType::Melee,
                                );

                                

                            },
                            Err(_rr) => {},
                        }

                        if !hit_target {
                            match physics_cells.get(collider_entity) {
                                Ok(cell_component) => {

                                    hit_target = true;

                                    let cell_data = world_cells.data.get_mut(&cell_component.id).unwrap();

                                    hit_result = cell_data.health.apply_damage(
                              &attack_event.targetted_limb, 
                                &attack_event.damage_model,
                                        &mut net_message_event,
                                        &handle_to_entity,
                                        &attack_event.attacker_sensed_by,
                                        &attack_event.attacker_sensed_by_cached,
                       attack_event.position.into(),
                                        &attack_event.attacker_name,
                                        &get_cell_name(cell_data),
                                        &DamageType::Melee,
                                    );


                                    
                                },
                                Err(_rr) => {},
                            }
                        }
                        
                        match hit_result {
                            crate::space_core::components::health::HitResult::HitSoft => {
                                attack_event.combat_sound_set.spawn_hit_sfx(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                            },
                            crate::space_core::components::health::HitResult::Blocked => {
                                attack_event.combat_sound_set.spawn_hit_blocked(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                            },
                            crate::space_core::components::health::HitResult::Missed => {},
                        }

                        if hit_target {
                            hit_anything = true;
                        }

                        !hit_target

                    }
                );

                if !hit_anything {
                    attack_event.combat_sound_set.spawn_miss_sfx(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                }

            },
            CombatType::Projectile(projectile_type) => {
                match projectile_type {
                    crate::space_core::components::inventory_item::ProjectileType::Laser(_color, _height, _radius, range) => {
                        // Perform ray_cast and obtain start and stop position for this projectile all sensed_by netcode call.
                        // Setup hardcoded client-side laser emissive capsule laser DirectionalParticle with color, height, radius, start_pos, stop_pos.

                        let collider_groups = get_bit_masks(ColliderGroup::Standard);
                        let interaction_groups = InteractionGroups::new(collider_groups.0,collider_groups.1);

                        let additive = direction_additive * *range;

                        let mut hit_entity = None;

                        query_pipeline.intersections_with_shape(
                            &QueryPipelineColliderComponentsSet(&collider_query),
                            &(
                                Vec3::new(
                                    attack_event.position.x, 
                                    1.0,
                                    attack_event.position.z,
                                )
                                -
                                additive,
                                Quat::from_rotation_y(attack_event.angle)).into(),
                            &Cuboid::new(Vec3::new(*range, 1.0, 0.3).into()),
                            interaction_groups,
                            None, 
                            |collider_handle| {

                                let collider_entity = collider_handle.entity();

                                if collider_entity == attack_event.attacker_entity {
                                    return true;
                                }

                                let projectile_weapon_entity = attack_event.weapon_entity.unwrap();

                                if collider_entity == projectile_weapon_entity {
                                    return true;
                                }

                                let mut hit_target = false;
                                let mut hit_result = HitResult::Missed;

                                // combat_damage_model Needs to be inventory_item_component projectile damage model.
                                match health_query.get_mut(collider_entity) {
                                    Ok((mut health_component, examinable_component)) => {

                                        hit_target = true;

                                        hit_result = health_component.apply_damage(
                                            &attack_event.targetted_limb, 
                                            &attack_event.damage_model,
                                            &mut net_message_event,
                                                &handle_to_entity,
                                                &attack_event.attacker_sensed_by,
                                            &attack_event.attacker_sensed_by_cached,
                                            attack_event.position.into(),
                                                &attack_event.attacker_name,
                                                &examinable_component.name,
                                                &DamageType::Projectile,
                                        );

                                        

                                    },
                                    Err(_rr) => {},
                                }

                                // combat_damage_model Needs to be inventory_item_component projectile damage model.
                                if !hit_target {
                                    match physics_cells.get(collider_entity) {
                                        Ok(cell_component) => {
    
                                            hit_target = true;
    
                                            let cell_data = world_cells.data.get_mut(&cell_component.id).unwrap();
    
                                            hit_result = cell_data.health.apply_damage(
                                    &attack_event.targetted_limb, 
                                    &attack_event.damage_model,
                                                &mut net_message_event,
                                                &handle_to_entity,
                                                &attack_event.attacker_sensed_by,
                                                &attack_event.attacker_sensed_by_cached,
                            attack_event.position.into(),
                                                &attack_event.attacker_name,
                                                &get_cell_name(cell_data),
                                                &DamageType::Projectile,
                                            );
    
    
                                            
                                        },
                                        Err(_rr) => {},
                                    }
                                }
                                
                                match hit_result {
                                    crate::space_core::components::health::HitResult::HitSoft => {
                                        attack_event.combat_sound_set.spawn_hit_sfx(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                                    },
                                    crate::space_core::components::health::HitResult::Blocked => {
                                        attack_event.combat_sound_set.spawn_hit_blocked(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                                    },
                                    crate::space_core::components::health::HitResult::Missed => {},
                                }

                                if hit_target {
                                    hit_entity=Some(collider_entity);
                                }

                                !hit_target
                                
                            }
                        );

                        match hit_entity {
                            Some(_collider_entity) => {

                                //let collider_position = rigid_body_positions_mut.get_mut(collider_entity).unwrap();


                            },
                            None => {},
                        }

                    },
                    crate::space_core::components::inventory_item::ProjectileType::Ballistic => {},
                }
            },
        }

    }

}
