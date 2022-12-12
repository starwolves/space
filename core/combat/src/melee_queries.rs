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
    rapier::prelude::Group,
};
use entity::{examine::Examinable, health::HealthComponent};
use gridmap::{
    events::Cell,
    grid::{cell_id_to_world, GridmapData, GridmapMain},
};
use inventory_item::combat::{MeleeCombat, ProjectileCombat};
use math::grid::Vec3Int;

use crate::{
    active_attacks::ActiveAttacks,
    attack::{Attack, CellHitSimple, CombatType, EntityHitSimple, QueryCombatHitResult},
};

/// When a melee attack hit nothing as an event.
pub struct MeleeBlank {
    pub incremented_id: u64,
}

/// Attack physics query height.
pub const ATTACK_HEIGHT: f32 = 1.6;

/// The physics query attack result.
#[derive(Debug)]
#[cfg(feature = "server")]
pub struct AttackResult {
    /// The entity id of the hit entity.
    pub entity_option: Option<Entity>,
    /// The cell id of the hit entity.
    pub cell_id_option: Option<Vec3Int>,
    /// The distance between the attacker and hit entity.
    pub distance: f32,
    /// The hit point of the attack.
    pub hit_point: Vec3,
    /// The entity id of the hit entity.
    pub collider_handle: Entity,
    /// Whether the hit entity is a combat obstacle.
    pub is_combat_obstacle: bool,
    /// Whether the hit entity is a laser obstacle.
    pub is_laser_obstacle: bool,
}

/// The melee physics query event.
#[cfg(feature = "server")]
pub struct MeleeDirectQuery {
    /// The entity id of the attacker.
    pub attacker_entity: Entity,
    /// The entity id of the targetted entity.
    pub targetted_entity: Option<Entity>,
    /// The id of the targetted cell.
    pub targetted_cell: Option<Vec3Int>,
    /// Attack angle.
    pub angle: f32,
    /// Attack range.
    pub range: f32,
    /// Exclude hitting certain entities in this physics query.
    pub exclude_physics: Vec<Entity>,
    /// Attack with bare hands.
    pub barehanded: bool,
    /// Attack id.
    pub incremented_id: u64,
}
use physics::physics::{get_bit_masks, ColliderGroup};

/// Perform a melee physics query with event [MeleeDirectQuery].
#[cfg(feature = "server")]
pub(crate) fn melee_direct(
    mut melee_direct_events: EventReader<MeleeDirectQuery>,
    attacker_entities: Query<&Transform>,
    rapier_context: Res<RapierContext>,
    colliders: Query<&Parent, With<Collider>>,
    mut rigidbody_query: Query<(&mut HealthComponent, &Examinable, &Transform)>,
    physics_cells: Query<&Cell>,
    mut world_cells: ResMut<GridmapMain>,
    gridmap_data: Res<GridmapData>,
    mut query_hit_result: EventWriter<QueryCombatHitResult>,
    mut cached_attacks: ResMut<ActiveAttacks>,
    mut blank: EventWriter<MeleeBlank>,
) {
    for attack_event in melee_direct_events.iter() {
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
        let collider_groups = get_bit_masks(ColliderGroup::Standard);

        let query_filter = QueryFilter::new().groups(InteractionGroups::new(
            Group::from_bits(collider_groups.0).unwrap(),
            Group::from_bits(collider_groups.1).unwrap(),
        ));

        let additive = direction_additive * attack_event.range;

        let mut hit_entities: Vec<AttackResult> = vec![];

        let attack_height;
        let cast_vertical_extents;

        if attack_event.barehanded
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
                attacker_transform.translation.x,
                attack_height,
                attacker_transform.translation.z,
            ) - additive,
            Quat::from_rotation_y(attack_event.angle).into(),
            &Collider::cuboid(shape_vec.x, shape_vec.y, shape_vec.z),
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

                        hit_entities.push(AttackResult {
                            entity_option: Some(collider_entity),
                            cell_id_option: None,
                            distance: attacker_transform.translation.distance(position),
                            hit_point: position,
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

                        let cell_data = world_cells.grid_data.get_mut(&cell_component.id).unwrap();

                        hit_entities.push(AttackResult {
                            entity_option: None,
                            cell_id_option: Some(cell_component.id),
                            distance: attacker_transform.translation.distance(position),
                            hit_point: position,
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

        let mut entity_hits = vec![];
        let mut cell_hits = vec![];

        match hit_entity {
            Some(attack_result) => {
                match attack_result.cell_id_option {
                    Some(cell_id) => {
                        cell_hits.push(CellHitSimple {
                            cell: cell_id,
                            hit_point: cell_id_to_world(cell_id),
                        });
                    }
                    None => {}
                }
                match attack_result.entity_option {
                    Some(e) => {
                        entity_hits.push(e);
                    }
                    None => {}
                }
            }
            None => {}
        }

        let mut entity_hits_transforms = vec![];

        for e in entity_hits {
            match attacker_entities.get(e) {
                Ok(t) => {
                    entity_hits_transforms.push(EntityHitSimple {
                        entity: e,
                        hit_point: t.translation,
                    });
                }
                Err(_rr) => {
                    warn!("hit entity without a transform!");
                }
            }
        }

        if entity_hits_transforms.len() == 0 && cell_hits.len() == 0 {
            blank.send(MeleeBlank {
                incremented_id: attack_event.incremented_id,
            });
        }

        let hit_result = QueryCombatHitResult {
            incremented_id: attack_event.incremented_id,
            entities_hits: entity_hits_transforms,
            cell_hits,
        };

        match cached_attacks.map.get_mut(&attack_event.incremented_id) {
            Some(c) => {
                c.hit_result = Some(hit_result.clone());
                c.melee = Some(true);
            }
            None => {
                warn!(
                    "Couldnt find cached attack! {}",
                    attack_event.incremented_id
                );
            }
        }

        query_hit_result.send(hit_result);
    }
}

/// Perform the attack handler logic for items. The combat logic and behaviour of items being used as weapons is defined here.
#[cfg(feature = "server")]
pub fn melee_attack_handler<T: Component>(
    weapon_entities: Query<(&MeleeCombat, Option<&ProjectileCombat>), With<T>>,
    mut attacks: EventReader<Attack>,
    mut melee_attack: EventWriter<MeleeDirectQuery>,
) {
    for attack in attacks.iter() {
        let combat_component;
        let projectile_combat_component_option;
        let weapon_entity;

        match attack.weapon_option {
            Some(ent) => {
                weapon_entity = ent;
                match weapon_entities.get(ent) {
                    Ok((i, p_o)) => {
                        combat_component = i;
                        projectile_combat_component_option = p_o;
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

        let mut combat_type;
        if projectile_combat_component_option.is_some() {
            combat_type = &CombatType::Projectile;
        } else {
            combat_type = &CombatType::MeleeDirect;
        }

        match projectile_combat_component_option {
            None => {}
            Some(_projecttile_combat) => {
                if attack.alt_attack_mode {
                    combat_type = &CombatType::MeleeDirect;
                }
            }
        }
        if matches!(combat_type, CombatType::MeleeDirect) {
            melee_attack.send(MeleeDirectQuery {
                attacker_entity: attack.attacker,
                targetted_entity: attack.targetted_entity,
                targetted_cell: attack.targetted_cell,
                angle: attack.angle,
                range: combat_component.melee_attack_range,
                exclude_physics: vec![weapon_entity],
                barehanded: false,
                incremented_id: attack.incremented_id,
            });
        }
    }
}
