use bevy::prelude::{warn, Commands, Component, EventReader, Query, Res, ResMut, Transform};
use gridmap::grid::cell_id_to_world;
use sfx::entity_update::SfxAutoDestroyTimers;
use sounds::shared::CombatSoundSet;

use crate::{
    active_attacks::ActiveAttacks,
    apply_damage::HealthCombatHitResult,
    attack::{Attack, HitResult},
};

/// Get the right sounds to spawn for combat.

pub fn health_combat_hit_result_sfx<T: Component>(
    mut combat_hit_results: EventReader<HealthCombatHitResult>,
    cached_attacks: Res<ActiveAttacks>,
    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
    transforms: Query<&Transform>,
    attacker_criteria: Query<&T>,
) {
    for health_combat_hit_result in combat_hit_results.iter() {
        match cached_attacks
            .map
            .get(&health_combat_hit_result.incremented_id)
        {
            Some(attack_cache) => {
                let melee;

                match attack_cache.melee {
                    Some(n) => {
                        melee = n;
                    }
                    None => {
                        warn!("cache of attack wasnt yet setup.");
                        continue;
                    }
                }

                for entity_hit in &health_combat_hit_result.entities_hits {
                    match attacker_criteria.get(entity_hit.entity) {
                        Ok(_) => {}
                        Err(_rr) => {
                            continue;
                        }
                    }

                    let tra;
                    match transforms.get(entity_hit.entity) {
                        Ok(t) => {
                            tra = t;
                        }
                        Err(_) => {
                            warn!("Couldnt find transform of hit result hit entity!");
                            continue;
                        }
                    }

                    if melee {
                        match entity_hit.hit_result {
                            HitResult::HitSoft => {
                                CombatSoundSet::default().spawn_hit_sfx(
                                    &mut commands,
                                    *tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Blocked => {
                                CombatSoundSet::default().spawn_hit_blocked(
                                    &mut commands,
                                    *tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Missed => {
                                CombatSoundSet::default().spawn_default_sfx(
                                    &mut commands,
                                    *tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                        }
                    } else {
                        match entity_hit.hit_result {
                            HitResult::HitSoft => {
                                CombatSoundSet::default_laser_projectiles().spawn_hit_sfx(
                                    &mut commands,
                                    *tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Blocked => {
                                CombatSoundSet::default_laser_projectiles().spawn_hit_blocked(
                                    &mut commands,
                                    *tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Missed => {}
                        }
                    }
                }
            }
            None => {
                warn!("Couldnt find cached attack.");
                continue;
            }
        }
    }
}

/// Spawns sound effects.

pub(crate) fn health_combat_hit_result_sfx_cells(
    mut combat_hit_results: EventReader<HealthCombatHitResult>,
    cached_attacks: Res<ActiveAttacks>,
    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut commands: Commands,
) {
    for health_combat_hit_result in combat_hit_results.iter() {
        match cached_attacks
            .map
            .get(&health_combat_hit_result.incremented_id)
        {
            Some(attack_cache) => {
                let melee;

                match attack_cache.melee {
                    Some(n) => {
                        melee = n;
                    }
                    None => {
                        warn!("cache of attack wasnt yet setup.");
                        continue;
                    }
                }

                for cell_hit in &health_combat_hit_result.cell_hits {
                    let tra = Transform::from_translation(cell_id_to_world(cell_hit.cell_id));

                    if melee {
                        match cell_hit.hit_result {
                            HitResult::HitSoft => {
                                CombatSoundSet::default().spawn_hit_sfx(
                                    &mut commands,
                                    tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Blocked => {
                                CombatSoundSet::default().spawn_hit_blocked(
                                    &mut commands,
                                    tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Missed => {
                                CombatSoundSet::default().spawn_default_sfx(
                                    &mut commands,
                                    tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                        }
                    } else {
                        match cell_hit.hit_result {
                            HitResult::HitSoft => {
                                CombatSoundSet::default_laser_projectiles().spawn_hit_sfx(
                                    &mut commands,
                                    tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Blocked => {
                                CombatSoundSet::default_laser_projectiles().spawn_hit_blocked(
                                    &mut commands,
                                    tra,
                                    &mut sfx_auto_destroy_timers,
                                );
                            }
                            HitResult::Missed => {}
                        }
                    }
                }
            }
            None => {
                warn!("Couldnt find cached attack.");
                continue;
            }
        }
    }
}

/// The attack sfx handler for items used to attack. This decides the sound effects that will play during combat on a per entity basis.

pub fn attack_sfx<T: Component>(
    mut attack_events: EventReader<Attack>,
    transforms: Query<&Transform>,
    mut commands: Commands,
    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    cached_attacks: Res<ActiveAttacks>,
    weapon_criteria: Query<&T>,
) {
    for attack in attack_events.iter() {
        let attack_cache;

        match cached_attacks.map.get(&attack.incremented_id) {
            Some(c) => {
                attack_cache = c;
            }
            None => {
                warn!("Couldnt find attack cache for sfx.");
                continue;
            }
        }

        let melee;

        match attack_cache.melee {
            Some(n) => {
                melee = n;
            }
            None => {
                warn!("attack cache attack not yet setup for sfx.");
                continue;
            }
        }

        match attack_cache.attack.weapon_option {
            Some(w) => match weapon_criteria.get(w) {
                Ok(_) => {}
                Err(_) => {
                    continue;
                }
            },
            None => {}
        }

        match transforms.get(attack.attacker) {
            Ok(transform) => {
                if !melee {
                    CombatSoundSet::default_laser_projectiles().spawn_default_sfx(
                        &mut commands,
                        *transform,
                        &mut sfx_auto_destroy_timers,
                    );
                } else {
                    CombatSoundSet::default().spawn_default_sfx(
                        &mut commands,
                        *transform,
                        &mut sfx_auto_destroy_timers,
                    );
                }
            }
            Err(_) => {}
        }
    }
}
