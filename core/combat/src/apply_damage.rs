use bevy::prelude::{warn, Entity, EventReader, EventWriter, Query, Res, ResMut, Resource};
use entity::health::{HealthComponent, HealthContainer};
use gridmap::{grid::Gridmap, set_cell::SetCell};
use inventory::combat::{DamageModel, MeleeCombat, ProjectileCombat};
use math::grid::Vec3Int;

use crate::{
    active_attacks::ActiveAttacks,
    attack::{calculate_damage, HitResult, QueryCombatHitResult},
};

/// Entity hits from [HealthCombatHitResult].

pub struct EntityHit {
    pub entity: Entity,
    pub hit_result: HitResult,
    pub limb_hit: String,
}
/// Cell hits from [HealthCombatHitResult].

pub struct CellHit {
    pub cell_id: Vec3Int,
    pub hit_result: HitResult,
}

/// Event with combat results for health processing.

pub struct HealthCombatHitResult {
    pub incremented_id: u64,
    pub entities_hits: Vec<EntityHit>,
    pub cell_hits: Vec<CellHit>,
}

/// Resource with active damage applyers for health processing.
#[derive(Default, Resource)]

pub struct ActiveApplyDamage {
    pub list: Vec<DamageApplyer>,
}

/// A health damage application event.

pub struct DamageApplyer {
    /// Attack id.
    pub incremented_id: u64,
    pub damage_models: Vec<ApplyDamageModel>,
    pub multipliers: Vec<DamageMultiplier>,
}

/// Information about a damage model contained by [DamageApplyer].

pub struct ApplyDamageModel {
    pub damage_model: DamageModel,
    /// Damage applyer type, "main" most of the time.
    pub signature: String,
}

/// Multiply damage data contained by [DamageApplyer].

pub struct DamageMultiplier {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
    /// Damage applyer type, "main" most of the time.
    pub signature: String,
}

/// Initiate damage applying.

pub(crate) fn start_apply_damage(
    mut query_hit_results: EventReader<QueryCombatHitResult>,
    combat_storage: Res<ActiveAttacks>,
    weapon_entities: Query<(&MeleeCombat, Option<&ProjectileCombat>)>,
    attackers: Query<&MeleeCombat>,
    mut active_applydamage: ResMut<ActiveApplyDamage>,
) {
    for hit_result in query_hit_results.iter() {
        let attack_cache;

        match combat_storage.map.get(&hit_result.incremented_id) {
            Some(st) => {
                attack_cache = st;
            }
            None => {
                warn!(
                    "QueryCombatHitResult event id wasnt fully cached ent. {}",
                    hit_result.incremented_id
                );
                continue;
            }
        }

        let melee;

        match attack_cache.melee {
            Some(n) => {
                melee = n;
            }
            None => {
                warn!("melee wasnt properly cached.");
                continue;
            }
        }

        let damage_model;
        match attack_cache.attack.weapon_option {
            Some(weapon_entity) => {
                let melee_combat_component;
                let projectile_combat_component_option;

                match weapon_entities.get(weapon_entity) {
                    Ok((m, p)) => {
                        melee_combat_component = m;
                        projectile_combat_component_option = p;
                    }
                    Err(_rr) => {
                        warn!("Coudlnt find belonging weapon entity components!");
                        continue;
                    }
                }

                let is_melee_attack = melee || projectile_combat_component_option.is_none();
                if is_melee_attack {
                    damage_model = melee_combat_component.combat_melee_damage_model.clone();
                } else {
                    damage_model = projectile_combat_component_option
                        .unwrap()
                        .combat_projectile_damage_model
                        .clone();
                }
            }
            None => {
                // Bare hand attack.
                match attackers.get(attack_cache.attack.attacker) {
                    Ok(melee_combat) => {
                        damage_model = melee_combat.combat_melee_damage_model.clone();
                    }
                    Err(_rr) => {
                        warn!("Attacker had no MeleeCombat component.");
                        continue;
                    }
                }
            }
        }

        active_applydamage.list.push(DamageApplyer {
            incremented_id: hit_result.incremented_id,
            damage_models: vec![ApplyDamageModel {
                damage_model,
                signature: "main".to_string(),
            }],
            multipliers: vec![],
        });
    }
}

/// Finalize damage applying.

pub(crate) fn finalize_apply_damage(
    combat_storage: Res<ActiveAttacks>,
    mut health_entities: Query<&mut HealthComponent>,
    mut health_combat_hit_result: EventWriter<HealthCombatHitResult>,
    _gridmap_main: Res<Gridmap>,
    mut active_applydamage: ResMut<ActiveApplyDamage>,
    mut _set_cell: EventWriter<SetCell>,
) {
    for damage_appler in active_applydamage.list.iter() {
        let attack_cache;

        match combat_storage.map.get(&damage_appler.incremented_id) {
            Some(st) => {
                attack_cache = st;
            }
            None => {
                warn!(
                    "QueryCombatHitResult event id wasnt fully cached ent. {}",
                    damage_appler.incremented_id
                );
                continue;
            }
        }

        let hit_result;

        match &attack_cache.hit_result {
            Some(h) => {
                hit_result = h;
            }
            None => {
                warn!("attack cache didnt yet have hit result for finalize apply damage.");
                continue;
            }
        }

        let mut entity_hits = vec![];
        let cell_hits = vec![];

        let mut brute_multiplier = 1.;
        let mut burn_multiplier = 1.;
        let mut toxin_multiplier = 1.;

        for multiplier in damage_appler.multipliers.iter() {
            brute_multiplier *= multiplier.brute;
            burn_multiplier *= multiplier.burn;
            toxin_multiplier *= multiplier.toxin;
        }

        for apply_damage_model in damage_appler.damage_models.iter() {
            for hit_entity in hit_result.entities_hits.iter() {
                match health_entities.get_mut(hit_entity.entity) {
                    Ok(mut health_comp) => {
                        let (brute_damage, burn_damage, toxin_damage, new_hit_result) =
                            calculate_damage(
                                &health_comp.health.health_flags,
                                &apply_damage_model.damage_model.damage_flags,
                                &(&apply_damage_model.damage_model.brute * brute_multiplier),
                                &(&apply_damage_model.damage_model.burn * burn_multiplier),
                                &(&apply_damage_model.damage_model.toxin * toxin_multiplier),
                            );

                        match &mut health_comp.health.health_container {
                            HealthContainer::Humanoid(humanoid_health) => {
                                if attack_cache.attack.targetted_limb == "head" {
                                    humanoid_health.head_brute += brute_damage;
                                    humanoid_health.head_burn += burn_damage;
                                    humanoid_health.head_toxin += toxin_damage;
                                } else if attack_cache.attack.targetted_limb == "torso" {
                                    humanoid_health.torso_brute += brute_damage;
                                    humanoid_health.torso_burn += burn_damage;
                                    humanoid_health.torso_toxin += toxin_damage;
                                } else if attack_cache.attack.targetted_limb == "right_arm" {
                                    humanoid_health.right_arm_brute += brute_damage;
                                    humanoid_health.right_arm_burn += burn_damage;
                                    humanoid_health.right_arm_toxin += toxin_damage;
                                } else if attack_cache.attack.targetted_limb == "left_arm" {
                                    humanoid_health.left_arm_brute += brute_damage;
                                    humanoid_health.left_arm_burn += burn_damage;
                                    humanoid_health.left_arm_toxin += toxin_damage;
                                } else if attack_cache.attack.targetted_limb == "right_leg" {
                                    humanoid_health.right_leg_brute += brute_damage;
                                    humanoid_health.right_leg_burn += burn_damage;
                                    humanoid_health.right_leg_toxin += toxin_damage;
                                } else if attack_cache.attack.targetted_limb == "left_leg" {
                                    humanoid_health.left_leg_brute += brute_damage;
                                    humanoid_health.left_leg_burn += burn_damage;
                                    humanoid_health.left_leg_toxin += toxin_damage;
                                }
                                if apply_damage_model.signature == "main" {
                                    entity_hits.push(EntityHit {
                                        entity: hit_entity.entity,
                                        hit_result: new_hit_result,
                                        limb_hit: attack_cache.attack.targetted_limb.clone(),
                                    });
                                }
                            }
                            HealthContainer::Entity(item) => {
                                item.brute += brute_damage;
                                item.burn += burn_damage;
                                item.toxin += toxin_damage;

                                if apply_damage_model.signature == "main" {
                                    entity_hits.push(EntityHit {
                                        entity: hit_entity.entity,
                                        hit_result: new_hit_result,
                                        limb_hit: attack_cache.attack.targetted_limb.clone(),
                                    });
                                }
                            }
                            _ => (),
                        }
                    }
                    Err(_rr) => {
                        warn!("Hit something without a health component!");
                        continue;
                    }
                }
            }
            /*
            for cell_id in hit_result.cell_hits.iter() {
                match gridmap_main.get_cell(cell_id.cell) {
                    Some(cell_data) => match cell_data.clone().health.health_container {
                        HealthContainer::Structure(_) => {
                            let (brute_damage, burn_damage, toxin_damage, hit_result) =
                                calculate_damage(
                                    &cell_data.health.health_flags,
                                    &cell_data.health.damage_flags,
                                    &apply_damage_model.damage_model.brute,
                                    &apply_damage_model.damage_model.burn,
                                    &apply_damage_model.damage_model.toxin,
                                );

                            let mut new_cell_data = cell_data.clone();
                            match &mut new_cell_data.health.health_container {
                                HealthContainer::Structure(cont) => {
                                    cont.brute += brute_damage;
                                    cont.burn += burn_damage;
                                    cont.toxin += toxin_damage;
                                }
                                _ => (),
                            }

                            set_cell.send(SetCell {
                                id: cell_id.cell,
                                data: new_cell_data,
                            });

                            if apply_damage_model.signature == "main" {
                                cell_hits.push(CellHit {
                                    cell_id: cell_id.cell,
                                    hit_result,
                                });
                            }
                        }
                        HealthContainer::Humanoid(_) => {
                            info!("struck humanoid instead.");
                        }
                        HealthContainer::Entity(_) => {
                            info!("struck entity instead.");
                        }
                    },
                    None => {
                        warn!("Couldnt find cellid in grid_map main.");
                        continue;
                    }
                }
            }*/
        }

        health_combat_hit_result.send(HealthCombatHitResult {
            incremented_id: damage_appler.incremented_id,
            entities_hits: entity_hits,
            cell_hits: cell_hits,
        });
    }
    active_applydamage.list.clear();
}
