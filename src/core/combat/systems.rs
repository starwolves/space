use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    system::{Commands, Query, Res, ResMut},
};
use bevy_math::{Quat, Vec3};
use bevy_rapier3d::{
    parry::query::Ray,
    plugin::RapierContext,
    prelude::{Collider, InteractionGroups},
};
use bevy_transform::components::Transform;

use crate::core::{
    chat::events::NetChatMessage,
    connected_player::resources::HandleToEntity,
    examinable::components::Examinable,
    gridmap::{
        components::Cell,
        events::ProjectileFOV,
        functions::{
            get_cell_name::get_cell_name,
            gridmap_functions::{cell_id_to_world, world_to_cell_id},
        },
        resources::{GridmapData, GridmapMain, Vec3Int},
    },
    health::{
        components::{DamageType, Health, HitResult},
        events::Attack,
    },
    inventory_item::components::CombatType,
    networking::resources::NetProjectileType,
    physics::functions::{get_bit_masks, ColliderGroup},
    senser::components::Senser,
    sfx::resources::SfxAutoDestroyTimers,
};

#[derive(Debug)]
struct AttackResult {
    entity_option: Option<Entity>,
    cell_id_option: Option<Vec3Int>,
    distance: f32,
    rigid_body_position: Vec3,
    collider_handle: Entity,
    is_combat_obstacle: bool,
    is_laser_obstacle: bool,
}

const ATTACK_HEIGHT: f32 = 1.6;

pub fn attack(
    mut attack_events: EventReader<Attack>,
    rapier_context: Res<RapierContext>,
    mut rigidbody_query: Query<(&mut Health, &Examinable, &Transform)>,
    mut world_cells: ResMut<GridmapMain>,
    physics_cells: Query<&Cell>,
    mut net_message_event: EventWriter<NetChatMessage>,
    handle_to_entity: Res<HandleToEntity>,
    mut commands: Commands,
    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut projectile_fov: EventWriter<ProjectileFOV>,
    sensers: Query<(Entity, &Senser)>,
    gridmap_data: Res<GridmapData>,
) {
    for attack_event in attack_events.iter() {
        let direction_additive = Vec3::new(-attack_event.angle.cos(), 0., attack_event.angle.sin());

        let attacker_cell_id = world_to_cell_id(attack_event.attacker_position);

        let mut sound_transform = Transform {
            translation: attack_event.attacker_position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        };

        match &attack_event.combat_type {
            CombatType::MeleeDirect => {
                let collider_groups = get_bit_masks(ColliderGroup::Standard);
                let interaction_groups =
                    InteractionGroups::new(collider_groups.0, collider_groups.1);

                let additive = direction_additive * attack_event.range;

                let mut hit_entities: Vec<AttackResult> = vec![];

                let attack_height;
                let cast_vertical_extents;

                if attack_event.weapon_entity.is_none()
                    || !attack_event.targetted_entity.is_none()
                    || !attack_event.targetted_cell.is_none()
                {
                    attack_height = 1.;
                    cast_vertical_extents = 1.;
                } else {
                    attack_height = ATTACK_HEIGHT;
                    cast_vertical_extents = 0.1;
                }

                let shape_vec = Vec3::new(attack_event.range, cast_vertical_extents, 0.1);

                rapier_context.intersections_with_shape(
                    Vec3::new(
                        attack_event.attacker_position.x,
                        attack_height,
                        attack_event.attacker_position.z,
                    ) - additive,
                    Quat::from_rotation_y(attack_event.angle).into(),
                    &Collider::cuboid(shape_vec.x, shape_vec.y, shape_vec.z),
                    interaction_groups,
                    None,
                    |collider_entity| {
                        if collider_entity == attack_event.attacker_entity {
                            return true;
                        }

                        match attack_event.weapon_entity {
                            Some(weapon_entity) => {
                                if collider_entity == weapon_entity {
                                    return true;
                                }
                            }
                            None => {}
                        }

                        match rigidbody_query.get_mut(collider_entity) {
                            Ok((
                                health_component,
                                _examinable_component,
                                rigid_body_position_component,
                            )) => {
                                let position = Vec3::new(
                                    rigid_body_position_component.translation.x,
                                    rigid_body_position_component.translation.y,
                                    rigid_body_position_component.translation.z,
                                );

                                hit_entities.push(AttackResult {
                                    entity_option: Some(collider_entity),
                                    cell_id_option: None,
                                    distance: attack_event.attacker_position.distance(position),
                                    rigid_body_position: position,
                                    collider_handle: collider_entity,
                                    is_combat_obstacle: health_component.is_combat_obstacle,
                                    is_laser_obstacle: health_component.is_laser_obstacle,
                                });
                            }
                            Err(_rr) => {}
                        }

                        match physics_cells.get(collider_entity) {
                            Ok(cell_component) => {
                                let position = cell_id_to_world(cell_component.id);

                                let cell_data =
                                    world_cells.grid_data.get_mut(&cell_component.id).unwrap();

                                hit_entities.push(AttackResult {
                                    entity_option: None,
                                    cell_id_option: Some(cell_component.id),
                                    distance: attack_event.attacker_position.distance(position),
                                    rigid_body_position: position,
                                    collider_handle: collider_entity,
                                    is_combat_obstacle: !gridmap_data
                                        .non_combat_obstacle_cells_list
                                        .contains(&cell_data.item),
                                    is_laser_obstacle: !gridmap_data
                                        .non_laser_obstacle_cells_list
                                        .contains(&cell_data.item),
                                });
                            }
                            Err(_rr) => {}
                        }

                        true
                    },
                );

                hit_entities.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
                hit_entities.reverse();

                let mut hit_result = HitResult::Missed;

                let mut hit_entity = None;

                match attack_event.targetted_entity {
                    Some(targetted_entity) => {
                        //melee hit and targetting an entity.
                        let mut found = false;
                        let mut first_blocker = None;
                        for attack_result in hit_entities.iter() {
                            match attack_result.entity_option {
                                Some(entity) => {
                                    if targetted_entity == entity {
                                        hit_entity = Some(attack_result);
                                        found = true;
                                        break;
                                    }
                                }
                                None => {}
                            }

                            if attack_result.is_combat_obstacle {
                                first_blocker = Some(attack_result);
                                break;
                            }
                        }
                        if !found {
                            match first_blocker {
                                Some(ff) => {
                                    hit_entity = Some(ff);
                                }
                                None => {}
                            }
                        }
                    }
                    None => {
                        match attack_event.targetted_cell {
                            // melee hit targetting a cell.
                            Some(targetted_cell) => {
                                let mut found = false;
                                let mut first_blocker = None;
                                for attack_result in hit_entities.iter() {
                                    match attack_result.cell_id_option {
                                        Some(cell_id) => {
                                            if targetted_cell == cell_id {
                                                hit_entity = Some(attack_result);
                                                found = true;
                                                break;
                                            }
                                        }
                                        None => {}
                                    }

                                    if attack_result.is_combat_obstacle {
                                        first_blocker = Some(attack_result);
                                        break;
                                    }
                                }
                                if !found {
                                    match first_blocker {
                                        Some(ff) => {
                                            hit_entity = Some(ff);
                                        }
                                        None => {}
                                    }
                                }
                            }
                            None => {
                                //melee hit without a specific target.
                                for res in hit_entities.iter() {
                                    if res.is_combat_obstacle {
                                        hit_entity = Some(res);
                                        break;
                                    }
                                }
                                if hit_entity.is_none() {
                                    hit_entity = hit_entities.first();
                                }
                            }
                        }
                    }
                }

                match hit_entity {
                    Some(attack_result) => match attack_result.entity_option {
                        Some(collider_entity) => match rigidbody_query.get_mut(collider_entity) {
                            Ok((
                                mut health_component,
                                examinable_component,
                                rigid_body_position_component,
                            )) => {
                                let attacked_cell_id = world_to_cell_id(
                                    rigid_body_position_component.translation.into(),
                                );

                                hit_result = health_component.apply_damage(
                                    &attack_event.targetted_limb,
                                    &attack_event.damage_model,
                                    &mut net_message_event,
                                    &handle_to_entity,
                                    &attacker_cell_id,
                                    &attacked_cell_id,
                                    &sensers,
                                    &attack_event.attacker_name,
                                    &examinable_component.name.get_a_name(),
                                    &DamageType::Melee,
                                    &attack_event.weapon_name,
                                    &attack_event.weapon_a_name,
                                    &attack_event.offense_words,
                                    &attack_event.trigger_words,
                                );
                            }
                            Err(_rr) => {}
                        },
                        None => {
                            let attacked_cell_id = attack_result.cell_id_option.unwrap();
                            let cell_data =
                                world_cells.grid_data.get_mut(&attacked_cell_id).unwrap();

                            hit_result = cell_data.health.apply_damage(
                                &attack_event.targetted_limb,
                                &attack_event.damage_model,
                                &mut net_message_event,
                                &handle_to_entity,
                                &attacker_cell_id,
                                &attacked_cell_id,
                                &sensers,
                                &attack_event.attacker_name,
                                &get_cell_name(cell_data, &gridmap_data),
                                &DamageType::Melee,
                                &attack_event.weapon_name,
                                &attack_event.weapon_a_name,
                                &attack_event.offense_words,
                                &attack_event.trigger_words,
                            );
                        }
                    },
                    None => {}
                }

                match hit_result {
                    crate::core::health::components::HitResult::HitSoft => {
                        attack_event.combat_sound_set.spawn_hit_sfx(
                            &mut commands,
                            sound_transform,
                            &mut sfx_auto_destroy_timers,
                        );
                    }
                    crate::core::health::components::HitResult::Blocked => {
                        attack_event.combat_sound_set.spawn_hit_blocked(
                            &mut commands,
                            sound_transform,
                            &mut sfx_auto_destroy_timers,
                        );
                    }
                    crate::core::health::components::HitResult::Missed => {
                        attack_event.combat_sound_set.spawn_default_sfx(
                            &mut commands,
                            sound_transform,
                            &mut sfx_auto_destroy_timers,
                        );
                    }
                }
            }
            CombatType::Projectile(projectile_type) => {
                match projectile_type {
                    crate::core::inventory_item::components::ProjectileType::Laser(
                        laser_color,
                        laser_height,
                        laser_radius,
                        laser_range,
                    ) => {
                        attack_event.combat_sound_set.spawn_default_sfx(
                            &mut commands,
                            sound_transform,
                            &mut sfx_auto_destroy_timers,
                        );

                        let collider_groups = get_bit_masks(ColliderGroup::Standard);
                        let interaction_groups =
                            InteractionGroups::new(collider_groups.0, collider_groups.1);

                        let additive = direction_additive * *laser_range;

                        let mut hit_entities: Vec<AttackResult> = vec![];

                        let attack_height;
                        let cast_vertical_extents;

                        if attack_event.weapon_entity.is_none()
                            || !attack_event.targetted_entity.is_none()
                            || !attack_event.targetted_cell.is_none()
                        {
                            attack_height = 1.;
                            cast_vertical_extents = 1.;
                        } else {
                            attack_height = ATTACK_HEIGHT;
                            cast_vertical_extents = 0.1;
                        }

                        let projectile_start_position = Vec3::new(
                            attack_event.attacker_position.x,
                            attack_height,
                            attack_event.attacker_position.z,
                        );

                        let projectile_rough_end_position = projectile_start_position - additive;

                        let points_vec = Vec3::new(*laser_range, cast_vertical_extents, 0.1);

                        rapier_context.intersections_with_shape(
                            projectile_rough_end_position,
                            Quat::from_rotation_y(attack_event.angle).into(),
                            &Collider::cuboid(points_vec.x, points_vec.y, points_vec.z),
                            interaction_groups,
                            None,
                            |collider_entity| {
                                if collider_entity == attack_event.attacker_entity {
                                    return true;
                                }

                                let projectile_weapon_entity = attack_event.weapon_entity.unwrap();

                                if collider_entity == projectile_weapon_entity {
                                    return true;
                                }

                                match rigidbody_query.get_mut(collider_entity) {
                                    Ok((
                                        health_component,
                                        _examinable_component,
                                        rigid_body_position_component,
                                    )) => {
                                        let position = Vec3::new(
                                            rigid_body_position_component.translation.x,
                                            rigid_body_position_component.translation.y,
                                            rigid_body_position_component.translation.z,
                                        );

                                        sound_transform.translation = position;

                                        hit_entities.push(AttackResult {
                                            entity_option: Some(collider_entity),
                                            cell_id_option: None,
                                            distance: attack_event
                                                .attacker_position
                                                .distance(position),
                                            rigid_body_position: position,
                                            collider_handle: collider_entity,
                                            is_combat_obstacle: health_component.is_combat_obstacle,
                                            is_laser_obstacle: health_component.is_laser_obstacle,
                                        });
                                    }
                                    Err(_rr) => {}
                                }

                                match physics_cells.get(collider_entity) {
                                    Ok(cell_component) => {
                                        let position = cell_id_to_world(cell_component.id);

                                        sound_transform.translation = position;

                                        let cell_data = world_cells
                                            .grid_data
                                            .get_mut(&cell_component.id)
                                            .unwrap();

                                        let r = AttackResult {
                                            entity_option: None,
                                            cell_id_option: Some(cell_component.id),
                                            distance: attack_event
                                                .attacker_position
                                                .distance(position),
                                            rigid_body_position: position,
                                            collider_handle: collider_entity,
                                            is_combat_obstacle: !gridmap_data
                                                .non_combat_obstacle_cells_list
                                                .contains(&cell_data.item),
                                            is_laser_obstacle: !gridmap_data
                                                .non_laser_obstacle_cells_list
                                                .contains(&cell_data.item),
                                        };

                                        hit_entities.push(r);
                                    }
                                    Err(_rr) => {}
                                }

                                true
                            },
                        );

                        hit_entities.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
                        hit_entities.reverse();

                        let mut hit_result = HitResult::Missed;

                        let mut hit_entity = None;

                        match attack_event.targetted_entity {
                            //projectile fired and targetted an entity.
                            Some(targetted_entity) => {
                                let mut found = false;
                                let mut first_blocker = None;
                                for attack_result in hit_entities.iter() {
                                    match attack_result.entity_option {
                                        Some(entity) => {
                                            if targetted_entity == entity {
                                                hit_entity = Some(attack_result);
                                                found = true;
                                                break;
                                            }
                                        }
                                        None => {}
                                    }

                                    if attack_result.is_combat_obstacle
                                        && attack_result.is_laser_obstacle
                                    {
                                        first_blocker = Some(attack_result);
                                        break;
                                    }
                                }
                                if !found {
                                    match first_blocker {
                                        Some(ff) => {
                                            hit_entity = Some(ff);
                                        }
                                        None => {}
                                    }
                                }
                            }
                            None => {
                                match attack_event.targetted_cell {
                                    // Projectile fired and targetted a cell.
                                    Some(targetted_cell) => {
                                        let mut found = false;
                                        let mut first_blocker = None;
                                        for attack_result in hit_entities.iter() {
                                            match attack_result.cell_id_option {
                                                Some(cell_id) => {
                                                    if targetted_cell == cell_id {
                                                        hit_entity = Some(attack_result);
                                                        found = true;
                                                        break;
                                                    }
                                                }
                                                None => {}
                                            }

                                            if attack_result.is_combat_obstacle
                                                && attack_result.is_laser_obstacle
                                            {
                                                first_blocker = Some(attack_result);
                                                break;
                                            }
                                        }
                                        if !found {
                                            match first_blocker {
                                                Some(ff) => {
                                                    hit_entity = Some(ff);
                                                }
                                                None => {}
                                            }
                                        }
                                    }
                                    None => {
                                        // Projectile fired without targetting
                                        for res in hit_entities.iter() {
                                            if res.is_combat_obstacle && res.is_laser_obstacle {
                                                hit_entity = Some(res);
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        let mut hit_point: Vec3;

                        match hit_entity {
                            Some(attack_result) => {
                                let ray = Ray::new(
                                    projectile_start_position.into(),
                                    (attack_result.rigid_body_position - projectile_start_position)
                                        .into(),
                                );
                                let max_toi = attack_result.distance * 1.2;

                                if let Some((_hit_collider_handle, hit_toi)) = rapier_context
                                    .cast_ray(
                                        ray.origin.into(),
                                        ray.dir.into(),
                                        max_toi,
                                        true,
                                        interaction_groups,
                                        Some(&|collider_handle| {
                                            collider_handle == attack_result.collider_handle
                                        }),
                                    )
                                {
                                    hit_point = ray.point_at(hit_toi).into();

                                    match attack_result.entity_option {
                                        Some(_) => {}
                                        None => {
                                            hit_point.y = ATTACK_HEIGHT;
                                        }
                                    }

                                    sound_transform.translation = hit_point;

                                    match attack_result.entity_option {
                                        Some(collider_entity) => {
                                            match rigidbody_query.get_mut(collider_entity) {
                                                Ok((
                                                    mut health_component,
                                                    examinable_component,
                                                    rigid_body_position_component,
                                                )) => {
                                                    let attacked_cell_id = world_to_cell_id(
                                                        rigid_body_position_component
                                                            .translation
                                                            .into(),
                                                    );

                                                    hit_result = health_component.apply_damage(
                                                        &attack_event.targetted_limb,
                                                        &attack_event.damage_model,
                                                        &mut net_message_event,
                                                        &handle_to_entity,
                                                        &attacker_cell_id,
                                                        &attacked_cell_id,
                                                        &sensers,
                                                        &attack_event.attacker_name,
                                                        &examinable_component.name.get_a_name(),
                                                        &DamageType::Projectile,
                                                        &attack_event.weapon_name,
                                                        &attack_event.weapon_a_name,
                                                        &attack_event.offense_words,
                                                        &attack_event.trigger_words,
                                                    );
                                                }
                                                Err(_rr) => {}
                                            }
                                        }
                                        None => {
                                            let attacked_cell_id =
                                                attack_result.cell_id_option.unwrap();
                                            let cell_data = world_cells
                                                .grid_data
                                                .get_mut(&attacked_cell_id)
                                                .unwrap();

                                            hit_result = cell_data.health.apply_damage(
                                                &attack_event.targetted_limb,
                                                &attack_event.damage_model,
                                                &mut net_message_event,
                                                &handle_to_entity,
                                                &attacker_cell_id,
                                                &attacked_cell_id,
                                                &sensers,
                                                &attack_event.attacker_name,
                                                &get_cell_name(cell_data, &gridmap_data),
                                                &DamageType::Projectile,
                                                &attack_event.weapon_name,
                                                &attack_event.weapon_a_name,
                                                &attack_event.offense_words,
                                                &attack_event.trigger_words,
                                            );
                                        }
                                    }

                                    match hit_result {
                                        crate::core::health::components::HitResult::HitSoft => {
                                            attack_event.combat_sound_set.spawn_hit_sfx(
                                                &mut commands,
                                                sound_transform,
                                                &mut sfx_auto_destroy_timers,
                                            );
                                        }
                                        crate::core::health::components::HitResult::Blocked => {
                                            attack_event.combat_sound_set.spawn_hit_blocked(
                                                &mut commands,
                                                sound_transform,
                                                &mut sfx_auto_destroy_timers,
                                            );
                                        }
                                        crate::core::health::components::HitResult::Missed => {}
                                    }
                                } else {
                                    hit_point = attack_result.rigid_body_position;
                                };
                            }
                            None => {
                                hit_point = projectile_rough_end_position;
                            }
                        }

                        let c_start_pos = projectile_start_position - (direction_additive * 0.5);

                        if c_start_pos.distance(hit_point) > 0.8 {
                            projectile_fov.send(ProjectileFOV {
                                laser_projectile: NetProjectileType::Laser(
                                    *laser_color,
                                    *laser_height,
                                    *laser_radius,
                                    c_start_pos,
                                    hit_point,
                                ),
                            });
                        }
                    }
                    crate::core::inventory_item::components::ProjectileType::Ballistic => {}
                }
            }
        }
    }
}
