use bevy::{
    math::Vec3,
    prelude::{Component, Entity, EventReader, EventWriter, Query, With},
};
use inventory::server::combat::ProjectileCombat;
use resources::math::Vec3Int;

use crate::attack::Attack;

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
