use bevy::{math::{Quat, Vec3}, prelude::{Commands, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform, warn}};
use bevy_rapier3d::prelude::{ColliderHandle, Cuboid, InteractionGroups, QueryPipeline, QueryPipelineColliderComponentsQuery, QueryPipelineColliderComponentsSet, Ray, RigidBodyPosition};

use crate::space_core::{components::{cell::Cell, examinable::Examinable, health::{DamageType, Health, HitResult}, inventory_item::{CombatType}, senser::Senser}, events::{general::{attack::Attack, projectile_fov::ProjectileFOV}, net::{net_chat_message::NetChatMessage}}, functions::{entity::collider_interaction_groups::{ColliderGroup, get_bit_masks}, gridmap::{get_cell_name::get_cell_name, gridmap_functions::{cell_id_to_world, world_to_cell_id}}}, resources::{doryen_fov::Vec3Int, gridmap_main::GridmapMain, handle_to_entity::HandleToEntity, network_messages::{NetProjectileType}, sfx_auto_destroy_timers::SfxAutoDestroyTimers}};

use bevy_rapier3d::physics::IntoEntity;


struct AttackResult {
    entity_option : Option<Entity>,
    cell_id_option: Option<Vec3Int>,
    distance: f32,
    rigid_body_position : Vec3,
    collider_handle : ColliderHandle,
}

const PROJECTILE_WEAPON_HEIGHT : f32 = 1.6;

pub fn attack(

    mut attack_events : EventReader<Attack>,
    query_pipeline: Res<QueryPipeline>,
    collider_query: QueryPipelineColliderComponentsQuery,
    mut rigidbody_query : Query<(&mut Health, &Examinable, &RigidBodyPosition)>,
    mut world_cells : ResMut<GridmapMain>,
    physics_cells : Query<&Cell>,
    mut net_message_event: EventWriter<NetChatMessage>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands : Commands,
    mut sfx_auto_destroy_timers : ResMut<SfxAutoDestroyTimers>,
    mut projectile_fov : EventWriter<ProjectileFOV>,
    sensers : Query<(Entity, &Senser)>,

) {

    for attack_event in attack_events.iter() {

        let direction_additive = Vec3::new(
            -attack_event.angle.cos(),
            0.,
            attack_event.angle.sin(),
        );

        let attacker_cell_id = world_to_cell_id(attack_event.attacker_position);

        let mut sound_transform = Transform{ translation: attack_event.attacker_position, rotation: Quat::IDENTITY, scale: Vec3::ONE };

        match &attack_event.combat_type {
            CombatType::MeleeDirect => {

                let collider_groups = get_bit_masks(ColliderGroup::Standard);
                let interaction_groups = InteractionGroups::new(collider_groups.0,collider_groups.1);

                let additive = direction_additive * attack_event.range;

                let mut hit_entities : Vec<AttackResult> = vec![];

                query_pipeline.intersections_with_shape(
                    &QueryPipelineColliderComponentsSet(&collider_query),
                    &(
                        Vec3::new(
                            attack_event.attacker_position.x, 
                            1.0, 
                            attack_event.attacker_position.z,
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

                        match rigidbody_query.get_mut(collider_entity) {
                            Ok((mut _health_component, _examinable_component, rigid_body_position_component)) => {

                                let position = Vec3::new(
                                    rigid_body_position_component.position.translation.x,
                                    rigid_body_position_component.position.translation.y,
                                    rigid_body_position_component.position.translation.z
                                );

                                hit_entities.push(AttackResult{
                                    entity_option: Some(collider_entity),
                                    cell_id_option: None,
                                    distance: attack_event.attacker_position.distance(position),
                                    rigid_body_position : position,
                                    collider_handle,
                                });
                            },
                            Err(_rr) => {},
                        }

                        match physics_cells.get(collider_entity) {
                            Ok(cell_component) => {

                                let position = cell_id_to_world(cell_component.id);

                                hit_entities.push(AttackResult{
                                    entity_option: None,
                                    cell_id_option: Some(cell_component.id),
                                    distance: attack_event.attacker_position.distance(
                                        position
                                    ),
                                    rigid_body_position : position,
                                    collider_handle,
                                });
                            },
                            Err(_rr) => {},
                        }

                        true

                    }
                );

                hit_entities.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
                hit_entities.reverse();

                let mut hit_result =  HitResult::Missed;

                match hit_entities.first() {
                    Some(attack_result) => {

                        match attack_result.entity_option {
                            Some(collider_entity) => {

                                match rigidbody_query.get_mut(collider_entity) {
                                    Ok((mut health_component, examinable_component, rigid_body_position_component)) => {
        
                                        
                                        let attacked_cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());

                                        hit_result = health_component.apply_damage(
                                            &attack_event.targetted_limb, 
                                            &attack_event.damage_model,
                                            &mut net_message_event,
                                                &handle_to_entity,
                                                &attacker_cell_id,
                                                &attacked_cell_id,
                                                            &sensers,
                                                &attack_event.attacker_name,
                                                &examinable_component.name,
                                                &DamageType::Melee,
                                                &attack_event.weapon_name,
                                        );
        
                                    },
                                    Err(_rr) => {},
                                }

                            },
                            None => {

                                let attacked_cell_id = attack_result.cell_id_option.unwrap();
                                let cell_data = world_cells.data.get_mut(&attacked_cell_id).unwrap();

                                hit_result = cell_data.health.apply_damage(
                            &attack_event.targetted_limb, 
                            &attack_event.damage_model,
                                    &mut net_message_event,
                                    &handle_to_entity,
                                    &attacker_cell_id,
                                    &attacked_cell_id,
                                    &sensers,
                                    &attack_event.attacker_name,
                                    &get_cell_name(cell_data),
                                    &DamageType::Melee,
                                    &attack_event.weapon_name,
                                );

                            },
                        }

                    },
                    None => {},
                }

                match hit_result {
                    crate::space_core::components::health::HitResult::HitSoft => {
                        attack_event.combat_sound_set.spawn_hit_sfx(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                    },
                    crate::space_core::components::health::HitResult::Blocked => {
                        attack_event.combat_sound_set.spawn_hit_blocked(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                    },
                    crate::space_core::components::health::HitResult::Missed => {
                        attack_event.combat_sound_set.spawn_default_sfx(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);
                    },
                }

            },
            CombatType::Projectile(projectile_type) => {
                match projectile_type {
                    crate::space_core::components::inventory_item::ProjectileType::Laser(laser_color, laser_height, laser_radius, laser_range) => {
                        // Perform ray_cast and obtain start and stop position for this projectile all sensed_by netcode call.
                        // Setup hardcoded client-side laser emissive capsule laser DirectionalParticle with color, height, radius, start_pos, stop_pos.

                        attack_event.combat_sound_set.spawn_default_sfx(&mut commands, sound_transform, &mut sfx_auto_destroy_timers);

                        let collider_groups = get_bit_masks(ColliderGroup::Standard);
                        let interaction_groups = InteractionGroups::new(collider_groups.0,collider_groups.1);

                        let projectile_start_position = Vec3::new(
                            attack_event.attacker_position.x, 
                            PROJECTILE_WEAPON_HEIGHT,
                            attack_event.attacker_position.z,
                        );

                        let additive = direction_additive * *laser_range;

                        let mut hit_entities : Vec<AttackResult> = vec![];

                        let colliders = &QueryPipelineColliderComponentsSet(&collider_query);

                        query_pipeline.intersections_with_shape(
                            colliders,
                            &(
                                projectile_start_position
                                -
                                additive,
                                Quat::from_rotation_y(attack_event.angle)).into(),
                            &Cuboid::new(Vec3::new(*laser_range, 0.1, 0.6).into()),
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

                                match rigidbody_query.get_mut(collider_entity) {
                                    Ok((mut _health_component, _examinable_component, rigid_body_position_component)) => {

                                        let position = Vec3::new(
                                            rigid_body_position_component.position.translation.x,
                                            rigid_body_position_component.position.translation.y,
                                            rigid_body_position_component.position.translation.z
                                        );

                                        sound_transform.translation = position;

                                        hit_entities.push(AttackResult{
                                            entity_option: Some(collider_entity),
                                            cell_id_option: None,
                                            distance: attack_event.attacker_position.distance(
                                                position
                                            ),
                                            rigid_body_position: position,
                                            collider_handle
                                        });
                                        
                                    },
                                    Err(_rr) => {},
                                }

                                match physics_cells.get(collider_entity) {
                                    Ok(cell_component) => {

                                        let position = cell_id_to_world(cell_component.id);

                                        sound_transform.translation = position;

                                        hit_entities.push(AttackResult{
                                            entity_option: None,
                                            cell_id_option: Some(cell_component.id),
                                            distance: attack_event.attacker_position.distance(
                                                position
                                            ),
                                            rigid_body_position: position,
                                            collider_handle
                                        }
                                    );
                                    
                                    },
                                    Err(_rr) => {},
                                }
                                

                                true
                            }
                        );

                        hit_entities.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
                        hit_entities.reverse();

                        let mut hit_result = HitResult::Missed;

                        match hit_entities.first() {
                            Some(attack_result) => {

                                let ray = Ray::new(projectile_start_position.into(), (attack_result.rigid_body_position - projectile_start_position).into());

                                if let Some((_hit_collider_handle, hit_toi)) = query_pipeline.cast_ray(
                                    colliders,
                                    &ray,
                                    attack_result.distance * 1.2,
                                    true,
                                    interaction_groups,
                                    Some(&|collider_handle| {
                                        collider_handle == attack_result.collider_handle
                                    }),
                                ) {

                                    let mut hit_point : Vec3 = ray.point_at(hit_toi).into();

                                    match attack_result.entity_option {
                                        Some(_) => {},
                                        None => {
                                            hit_point.y = PROJECTILE_WEAPON_HEIGHT;
                                        },
                                    }

                                    sound_transform.translation = hit_point;

                                    match attack_result.entity_option {
                                        Some(collider_entity) => {
            
                                            match rigidbody_query.get_mut(collider_entity) {
                                                Ok((mut health_component,examinable_component,rigid_body_position_component)) => {
                                                    
                                                    let attacked_cell_id = world_to_cell_id(rigid_body_position_component.position.translation.into());

                                                    hit_result = health_component.apply_damage(
                                                        &attack_event.targetted_limb, 
                                                        &attack_event.damage_model,
                                                        &mut net_message_event,
                                                            &handle_to_entity,
                                                            &attacker_cell_id,
                                                            &attacked_cell_id,
                                                            &sensers,
                                                            &attack_event.attacker_name,
                                                            &examinable_component.name,
                                                            &DamageType::Projectile,
                                                            &attack_event.weapon_name,
                                                    );
            
                                                },
                                                Err(_rr) => {},
                                            }
            
                                        },
                                        None => {
            
                                            let attacked_cell_id = attack_result.cell_id_option.unwrap();
                                            let cell_data = world_cells.data.get_mut(&attacked_cell_id).unwrap();
            
                                            hit_result = cell_data.health.apply_damage(
                                                &attack_event.targetted_limb, 
                                                &attack_event.damage_model,
                                                &mut net_message_event,
                                                    &handle_to_entity,
                                                    &attacker_cell_id,
                                                    &attacked_cell_id,
                                                    &sensers,
                                                    &attack_event.attacker_name,
                                                    &get_cell_name(cell_data),
                                                    &DamageType::Projectile,
                                                    &attack_event.weapon_name,
                                            );
            
                                        },
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

                                    projectile_fov.send(ProjectileFOV {
                                        laser_projectile: NetProjectileType::Laser(
                                            *laser_color,
                                            *laser_height,
                                            *laser_radius,
                                            projectile_start_position - (direction_additive * 0.5),
                                            hit_point,
                                        ),
                                    });



                                } else {
                                    warn!("Exclusive collider_handle projectile ray_intersect had no results.");
                                };

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
