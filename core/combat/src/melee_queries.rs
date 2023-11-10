use bevy::{
    math::Vec3,
    prelude::{Component, Entity, Event, EventReader, EventWriter, Query, With},
};
use inventory::server::combat::{MeleeCombat, ProjectileCombat};
use resources::math::Vec3Int;

use crate::attack::{Attack, CombatType};

/// When a melee attack hit nothing as an event.
#[derive(Event)]
pub struct MeleeBlank {
    pub incremented_id: u64,
}

/// Attack physics query height.
pub const ATTACK_HEIGHT: f32 = 1.6;

/// The physics query attack result.
#[derive(Debug)]

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
#[derive(Event)]
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

/// Perform the attack handler logic for items. The combat logic and behaviour of items being used as weapons is defined here.

pub fn melee_attack_handler<T: Component>(
    weapon_entities: Query<(&MeleeCombat, Option<&ProjectileCombat>), With<T>>,
    mut attacks: EventReader<Attack>,
    mut melee_attack: EventWriter<MeleeDirectQuery>,
) {
    for attack in attacks.read() {
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
