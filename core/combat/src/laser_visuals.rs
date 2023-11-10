use bevy::log::warn;
use bevy::prelude::{Component, EventReader, EventWriter, Query, Res, Transform, Vec3};
use gridmap::fov::ProjectileFOV;
use inventory::server::combat::ProjectileCombat;

use crate::{
    active_attacks::ActiveAttacks, attack::QueryCombatHitResult,
    projectile_queries::ProjectileBlank,
};

/// Manage laser projectile visuals that are integrated with FOV checks with [ProjectileFOV].

pub fn projectile_laser_visuals<T: Component>(
    mut blanks: EventReader<ProjectileBlank>,
    mut hits: EventReader<QueryCombatHitResult>,
    mut projectile_fov: EventWriter<ProjectileFOV>,
    active_attacks: Res<ActiveAttacks>,
    projectile_weapons: Query<&ProjectileCombat>,
    transforms: Query<&Transform>,
    weapon_criteria: Query<&T>,
) {
    use gridmap::net::ProjectileData;

    for blank in blanks.read() {
        let active_attack;

        match active_attacks.map.get(&blank.incremented_id) {
            Some(a) => {
                active_attack = a;
            }
            None => {
                warn!("Couldnt find active attack");
                continue;
            }
        }

        let weapon = active_attack.attack.weapon_option.unwrap();

        match weapon_criteria.get(weapon) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        match active_attack.melee {
            Some(is_melee) => {
                if is_melee {
                    continue;
                }
            }
            None => {
                warn!("melee cache was none.");
                continue;
            }
        }

        let projectile_combat_component;

        match projectile_weapons.get(weapon) {
            Ok(c) => {
                projectile_combat_component = c;
            }
            Err(_rr) => {
                warn!("couldnt find projectile_weapons.");
                continue;
            }
        }

        let attacker_position;

        match transforms.get(active_attack.attack.attacker) {
            Ok(t) => {
                attacker_position = t.translation;
            }
            Err(_rr) => {
                warn!("Couldnt find transform of attacker!");
                continue;
            }
        }

        let direction_additive = Vec3::new(
            -active_attack.attack.angle.cos(),
            0.,
            active_attack.attack.angle.sin(),
        );
        let c_start_pos = attacker_position - (direction_additive * 0.5);

        if c_start_pos.distance(blank.hit_point) > 0.8 {
            projectile_fov.send(ProjectileFOV {
                laser_projectile: ProjectileData::Laser(
                    (
                        projectile_combat_component.laser_color.r,
                        projectile_combat_component.laser_color.g,
                        projectile_combat_component.laser_color.b,
                        projectile_combat_component.laser_color.a,
                    ),
                    projectile_combat_component.laser_height,
                    projectile_combat_component.laser_radius,
                    c_start_pos,
                    blank.hit_point,
                ),
            });
        }
    }

    for hit in hits.read() {
        for entity_hit in hit.entities_hits.iter() {
            let active_attack;

            match active_attacks.map.get(&hit.incremented_id) {
                Some(a) => {
                    active_attack = a;
                }
                None => {
                    warn!("Couldnt find active attack");
                    continue;
                }
            }

            match active_attack.melee {
                Some(is_melee) => {
                    if is_melee {
                        continue;
                    }
                }
                None => {
                    warn!("melee cache was none.");
                    continue;
                }
            }

            let projectile_combat_component;

            match projectile_weapons.get(active_attack.attack.weapon_option.unwrap()) {
                Ok(c) => {
                    projectile_combat_component = c;
                }
                Err(_rr) => {
                    warn!("couldnt find projectile_weapons.");
                    continue;
                }
            }

            let attacker_position;

            match transforms.get(active_attack.attack.attacker) {
                Ok(t) => {
                    attacker_position = t.translation;
                }
                Err(_rr) => {
                    warn!("Couldnt find transform of attacker!");
                    continue;
                }
            }

            let direction_additive = Vec3::new(
                -active_attack.attack.angle.cos(),
                0.,
                active_attack.attack.angle.sin(),
            );
            let c_start_pos = attacker_position - (direction_additive * 0.5);

            if c_start_pos.distance(entity_hit.hit_point) > 0.8 {
                projectile_fov.send(ProjectileFOV {
                    laser_projectile: ProjectileData::Laser(
                        (
                            projectile_combat_component.laser_color.r,
                            projectile_combat_component.laser_color.g,
                            projectile_combat_component.laser_color.b,
                            projectile_combat_component.laser_color.a,
                        ),
                        projectile_combat_component.laser_height,
                        projectile_combat_component.laser_radius,
                        c_start_pos,
                        entity_hit.hit_point,
                    ),
                });
            }
        }

        for cell_hit in hit.cell_hits.iter() {
            let active_attack;

            match active_attacks.map.get(&hit.incremented_id) {
                Some(a) => {
                    active_attack = a;
                }
                None => {
                    warn!("Couldnt find active attack");
                    continue;
                }
            }

            match active_attack.melee {
                Some(is_melee) => {
                    if is_melee {
                        continue;
                    }
                }
                None => {
                    warn!("melee cache was none.");
                    continue;
                }
            }

            let projectile_combat_component;

            match projectile_weapons.get(active_attack.attack.weapon_option.unwrap()) {
                Ok(c) => {
                    projectile_combat_component = c;
                }
                Err(_rr) => {
                    warn!("couldnt find projectile_weapons.");
                    continue;
                }
            }

            let attacker_position;

            match transforms.get(active_attack.attack.attacker) {
                Ok(t) => {
                    attacker_position = t.translation;
                }
                Err(_rr) => {
                    warn!("Couldnt find transform of attacker!");
                    continue;
                }
            }

            let direction_additive = Vec3::new(
                -active_attack.attack.angle.cos(),
                0.,
                active_attack.attack.angle.sin(),
            );
            let c_start_pos = attacker_position - (direction_additive * 0.5);

            if c_start_pos.distance(cell_hit.hit_point) > 0.8 {
                projectile_fov.send(ProjectileFOV {
                    laser_projectile: ProjectileData::Laser(
                        (
                            projectile_combat_component.laser_color.r,
                            projectile_combat_component.laser_color.g,
                            projectile_combat_component.laser_color.b,
                            projectile_combat_component.laser_color.a,
                        ),
                        projectile_combat_component.laser_height,
                        projectile_combat_component.laser_radius,
                        c_start_pos,
                        cell_hit.hit_point,
                    ),
                });
            }
        }
    }
}
