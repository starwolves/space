use bevy::{
    hierarchy::Parent,
    math::{Quat, Vec3},
    prelude::{
        warn, Component, Entity, EventReader, EventWriter, Query, Res, ResMut, Transform, With,
    },
};
use bevy_rapier3d::{
    pipeline::QueryFilter,
    plugin::RapierContext,
    prelude::{Collider, InteractionGroups},
    rapier::prelude::{Group, Ray},
};
use entity::{examine::Examinable, health::HealthComponent};
use gridmap::{
    events::Cell,
    grid::{cell_id_to_world, Gridmap},
};
use inventory::combat::ProjectileCombat;
use math::grid::Vec3Int;

use crate::{
    active_attacks::ActiveAttacks,
    attack::{Attack, CellHitSimple, EntityHitSimple, QueryCombatHitResult},
    melee_queries::{AttackResult, ATTACK_HEIGHT},
};

/// The projectile attack physics query.

pub struct ProjectileQuery {
    /// Entity id of the attacker.
    pub attacker_entity: Entity,
    /// Entity id of the targetted entity.
    pub targetted_entity: Option<Entity>,
    /// Id of the targetted cell.
    pub targetted_cell: Option<Vec3Int>,
    /// Angle of attack.
    pub angle: f32,
    /// Excluded entities from physics query.
    pub exclude_physics: Vec<Entity>,
    /// Attack range.
    pub range: f32,
    /// Attack id.
    pub incremented: u64,
}
use physics::physics::{get_bit_masks, ColliderGroup};

/// Perform a projectile attack physics query by reading event [ProjectileQuery].

pub(crate) fn projectile_attack(
    mut projectile_events: EventReader<ProjectileQuery>,
    attacker_entities: Query<&Transform>,
    rapier_context: Res<RapierContext>,
    colliders: Query<&Parent, With<Collider>>,
    mut rigidbody_query: Query<(&mut HealthComponent, &Examinable, &Transform)>,
    physics_cells: Query<&Cell>,
    _world_cells: Res<Gridmap>,
    mut query_hit_result: EventWriter<QueryCombatHitResult>,
    mut cached_attacks: ResMut<ActiveAttacks>,
    mut blank_writer: EventWriter<ProjectileBlank>,
) {
    for attack_event in projectile_events.iter() {
        let direction_additive = Vec3::new(-attack_event.angle.cos(), 0., attack_event.angle.sin());

        let attacker_transform;

        match attacker_entities.get(attack_event.attacker_entity) {
            Ok(position) => {
                attacker_transform = position;
            }
            Err(_) => {
                warn!("Couldn't find attacker entity.");
                continue;
            }
        }

        let mut sound_transform = Transform {
            translation: attacker_transform.translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        };

        let collider_groups = get_bit_masks(ColliderGroup::Standard);

        let interaction_groups = InteractionGroups::new(
            Group::from_bits(collider_groups.0).unwrap(),
            Group::from_bits(collider_groups.1).unwrap(),
        );

        let query_filter = QueryFilter::new().groups(interaction_groups);

        let additive = direction_additive * attack_event.range;

        let mut hit_entities_query: Vec<AttackResult> = vec![];

        let attack_height;
        let cast_vertical_extents;

        if !attack_event.targetted_entity.is_none() || !attack_event.targetted_cell.is_none() {
            attack_height = 1.;
            cast_vertical_extents = 1.;
        } else {
            attack_height = ATTACK_HEIGHT;
            cast_vertical_extents = 0.1;
        }

        let projectile_start_position = Vec3::new(
            attacker_transform.translation.x,
            attack_height,
            attacker_transform.translation.z,
        );

        let projectile_rough_end_position = projectile_start_position - additive;

        let points_vec = Vec3::new(attack_event.range, cast_vertical_extents, 0.1);

        rapier_context.intersections_with_shape(
            projectile_rough_end_position,
            Quat::from_rotation_y(attack_event.angle).into(),
            &Collider::cuboid(points_vec.x, points_vec.y, points_vec.z),
            query_filter,
            |child_entity| {
                let collider_entity;

                match colliders.get(child_entity) {
                    Ok(parent_entity) => {
                        collider_entity = parent_entity.get();
                    }
                    Err(_rr) => {
                        collider_entity = child_entity;
                    }
                }

                if collider_entity == attack_event.attacker_entity {
                    return true;
                }

                if attack_event.exclude_physics.contains(&collider_entity) {
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
                        let distance = attacker_transform.translation.distance(position);

                        let ray = Ray::new(
                            projectile_start_position.into(),
                            (position - projectile_start_position).into(),
                        );
                        let max_toi = distance * 1.2;

                        let hit_point: Vec3;

                        if let Some((_hit_collider_handle, hit_toi)) = rapier_context.cast_ray(
                            ray.origin.into(),
                            ray.dir.into(),
                            max_toi,
                            true,
                            QueryFilter::new().groups(interaction_groups).predicate(
                                &|child_entity| {
                                    let colliderz;

                                    match colliders.get(child_entity) {
                                        Ok(parent_entity) => {
                                            colliderz = parent_entity.get();
                                        }
                                        Err(_rr) => {
                                            colliderz = child_entity;
                                        }
                                    }
                                    colliderz == collider_entity
                                },
                            ),
                        ) {
                            hit_point = ray.point_at(hit_toi).into();
                        } else {
                            hit_point = position;
                        };

                        sound_transform.translation = position;

                        hit_entities_query.push(AttackResult {
                            entity_option: Some(collider_entity),
                            cell_id_option: None,
                            distance,
                            hit_point,
                            collider_handle: collider_entity,
                            is_combat_obstacle: health_component.health.is_combat_obstacle,
                            is_laser_obstacle: health_component.health.is_laser_obstacle,
                        });
                    }
                    Err(_rr) => {}
                }

                match physics_cells.get(collider_entity) {
                    Ok(cell_component) => {
                        let position = cell_id_to_world(cell_component.id);
                        let distance = attacker_transform.translation.distance(position);

                        let ray = Ray::new(
                            projectile_start_position.into(),
                            (position - projectile_start_position).into(),
                        );
                        let max_toi = distance * 1.2;

                        let _hit_point: Vec3;

                        if let Some((_hit_collider_handle, hit_toi)) = rapier_context.cast_ray(
                            ray.origin.into(),
                            ray.dir.into(),
                            max_toi,
                            true,
                            QueryFilter::new().groups(interaction_groups).predicate(
                                &|child_entity| {
                                    let colliderz;

                                    match colliders.get(child_entity) {
                                        Ok(parent_entity) => {
                                            colliderz = parent_entity.get();
                                        }
                                        Err(_rr) => {
                                            colliderz = child_entity;
                                        }
                                    }
                                    colliderz == collider_entity
                                },
                            ),
                        ) {
                            _hit_point = ray.point_at(hit_toi).into();
                        } else {
                            _hit_point = position;
                        };

                        sound_transform.translation = position;

                        /*let cell_data = world_cells.get_cell(cell_component.id).unwrap();

                        let r = AttackResult {
                            entity_option: None,
                            cell_id_option: Some(cell_component.id),
                            distance: attacker_transform.translation.distance(position),
                            hit_point: hit_point,
                            collider_handle: collider_entity,
                            is_combat_obstacle: !world_cells
                                .non_combat_obstacle_cells_list
                                .contains(&cell_data.item_0.id),
                            is_laser_obstacle: !world_cells
                                .non_laser_obstacle_cells_list
                                .contains(&cell_data.item_0.id),
                        };

                        hit_entities_query.push(r);*/
                    }
                    Err(_rr) => {}
                }

                true
            },
        );

        hit_entities_query.sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
        hit_entities_query.reverse();

        let mut hit_results = vec![];

        match attack_event.targetted_entity {
            //projectile fired and targetted an entity.
            Some(targetted_entity) => {
                let mut found = false;
                let mut first_blocker = None;
                for attack_result in hit_entities_query.iter() {
                    match attack_result.entity_option {
                        Some(entity) => {
                            if targetted_entity == entity {
                                hit_results.push(attack_result);
                                found = true;
                                break;
                            }
                        }
                        None => {}
                    }

                    if attack_result.is_combat_obstacle && attack_result.is_laser_obstacle {
                        first_blocker = Some(attack_result);
                        break;
                    }
                }
                if !found {
                    match first_blocker {
                        Some(ff) => {
                            hit_results.push(ff);
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
                        for attack_result in hit_entities_query.iter() {
                            match attack_result.cell_id_option {
                                Some(cell_id) => {
                                    if targetted_cell == cell_id {
                                        hit_results.push(attack_result);
                                        found = true;
                                        break;
                                    }
                                }
                                None => {}
                            }

                            if attack_result.is_combat_obstacle && attack_result.is_laser_obstacle {
                                first_blocker = Some(attack_result);
                                break;
                            }
                        }
                        if !found {
                            match first_blocker {
                                Some(ff) => {
                                    hit_results.push(ff);
                                }
                                None => {}
                            }
                        }
                    }
                    None => {
                        // Projectile fired without targetting
                        for res in hit_entities_query.iter() {
                            if res.is_combat_obstacle && res.is_laser_obstacle {
                                hit_results.push(res);
                                break;
                            }
                        }
                    }
                }
            }
        }

        let mut hit_entities = vec![];
        let mut hit_cells = vec![];

        for hit in hit_results {
            match hit.entity_option {
                Some(e) => hit_entities.push({
                    EntityHitSimple {
                        entity: e,
                        hit_point: hit.hit_point,
                    }
                }),
                None => match hit.cell_id_option {
                    Some(cell_id) => {
                        hit_cells.push(CellHitSimple {
                            cell: cell_id,
                            hit_point: hit.hit_point,
                        });
                    }
                    None => {
                        warn!("AttackResult had empty options.");
                    }
                },
            }
        }

        if hit_entities.len() == 0 && hit_cells.len() == 0 {
            // Blank, we hit nothing but the projectile still fired (visuals etc).
            blank_writer.send(ProjectileBlank {
                hit_point: projectile_rough_end_position,
                incremented_id: attack_event.incremented,
            });
        }

        let hit_result = QueryCombatHitResult {
            incremented_id: attack_event.incremented,
            entities_hits: hit_entities,
            cell_hits: hit_cells,
        };

        match cached_attacks.map.get_mut(&attack_event.incremented) {
            Some(c) => {
                c.hit_result = Some(hit_result.clone());
                c.melee = Some(false);
            }
            None => {
                warn!("Couldnt find cached attack! {}", attack_event.incremented);
            }
        }

        query_hit_result.send(hit_result);
    }
}

/// In case projectile hit nothing as an event.

pub struct ProjectileBlank {
    /// Hit point location.
    pub hit_point: Vec3,
    /// Attack id.
    pub incremented_id: u64,
}

/// Perform projectile attack handler logic.

pub fn projectile_attack_handler<T: Component>(
    weapon_entities: Query<&ProjectileCombat, With<T>>,
    mut attacks: EventReader<Attack>,
    mut projectile_attack: EventWriter<ProjectileQuery>,
) {
    for attack in attacks.iter() {
        let combat_component;
        let weapon_entity;

        match attack.weapon_option {
            Some(ent) => {
                weapon_entity = ent;
                match weapon_entities.get(ent) {
                    Ok(i) => {
                        combat_component = i;
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
            None => {
                continue;
            }
        }
        if !attack.alt_attack_mode {
            projectile_attack.send(ProjectileQuery {
                attacker_entity: attack.attacker,
                targetted_entity: attack.targetted_entity,
                targetted_cell: attack.targetted_cell,
                angle: attack.angle,
                range: combat_component.laser_range,
                exclude_physics: vec![weapon_entity],
                incremented: attack.incremented_id,
            });
        }
    }
}
