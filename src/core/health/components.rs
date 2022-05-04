use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    prelude::Component,
    system::{Query, Res},
};
use rand::prelude::SliceRandom;

use crate::core::{
    chat::events::NetChatMessage,
    connected_player::resources::HandleToEntity,
    gridmap::resources::{to_doryen_coordinates, Vec3Int},
    inventory_item::components::HitSoundSurface,
    networking::resources::ReliableServerMessage,
    senser::components::Senser,
};

#[derive(Component)]
pub struct Health {
    pub health_container: HealthContainer,
    pub health_flags: HashMap<u32, HealthFlag>,
    pub raegent_container: RaegentContainer,
    pub hit_sound_surface: HitSoundSurface,
    pub is_combat_obstacle: bool,
    pub is_laser_obstacle: bool,
    pub is_reach_obstacle: bool,
}

impl Default for Health {
    fn default() -> Self {
        Self {
            health_container: HealthContainer::Entity(EntityContainer::default()),
            health_flags: HashMap::new(),
            raegent_container: RaegentContainer {
                raegents: HashMap::new(),
            },
            hit_sound_surface: HitSoundSurface::Soft,
            is_combat_obstacle: false,
            is_laser_obstacle: true,
            is_reach_obstacle: false,
        }
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum HealthFlag {
    ArmourPlated,
    HeadBruteDefence(f32),
    TorsoBruteDefence(f32),
}

pub enum HealthContainer {
    Humanoid(HumanoidHealth),
    Entity(EntityContainer),
}

#[allow(dead_code)]
pub struct RaegentContainer {
    raegents: HashMap<String, f32>,
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub enum DamageFlag {
    SoftDamage, //Ie fists.
    WeakLethalLaser,
    Stun(f32),
    Floor(f32),
}

#[derive(Debug)]
pub struct HumanoidHealth {
    pub head_brute: f32,
    pub head_burn: f32,
    pub head_toxin: f32,

    pub torso_brute: f32,
    pub torso_burn: f32,
    pub torso_toxin: f32,

    pub left_arm_brute: f32,
    pub left_arm_burn: f32,
    pub left_arm_toxin: f32,

    pub right_arm_brute: f32,
    pub right_arm_burn: f32,
    pub right_arm_toxin: f32,

    pub right_leg_brute: f32,
    pub right_leg_burn: f32,
    pub right_leg_toxin: f32,

    pub left_leg_brute: f32,
    pub left_leg_burn: f32,
    pub left_leg_toxin: f32,
}

pub struct EntityContainer {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}

#[derive(Clone)]
pub struct DamageModel {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
    pub damage_flags: HashMap<u32, DamageFlag>,
}

impl Default for DamageModel {
    fn default() -> Self {
        Self {
            brute: 0.,
            burn: 0.,
            toxin: 0.,
            damage_flags: HashMap::new(),
        }
    }
}

#[allow(dead_code)]
pub enum HitResult {
    HitSoft,
    Blocked,
    Missed,
}

pub fn calculate_damage(
    health_flags: &HashMap<u32, HealthFlag>,
    damage_flags: &HashMap<u32, DamageFlag>,

    brute: &f32,
    burn: &f32,
    toxin: &f32,
) -> (f32, f32, f32, HitResult) {
    let mut output_brute = brute.clone();
    let mut output_burn = burn.clone();
    let output_toxin = toxin.clone();

    let mut hit_result = HitResult::HitSoft;

    let mut damager_flags = vec![];

    for damage_flag in damage_flags.values() {
        damager_flags.push(damage_flag);
    }

    let mut structure_health_flags = vec![];

    for stucture_health_flag in health_flags.values() {
        structure_health_flags.push(stucture_health_flag);
    }

    let is_armour_plated = structure_health_flags.contains(&&HealthFlag::ArmourPlated);

    if damager_flags.contains(&&DamageFlag::SoftDamage) && is_armour_plated {
        output_brute = 0.;
        hit_result = HitResult::Blocked;
    } else if damager_flags.contains(&&DamageFlag::WeakLethalLaser) && is_armour_plated {
        output_burn *= 0.05;
        hit_result = HitResult::Blocked;
    }

    (output_brute, output_burn, output_toxin, hit_result)
}

pub enum DamageType {
    Melee,
    Projectile,
}

impl Health {
    pub fn apply_damage(
        &mut self,
        body_part: &str,
        damage_model: &DamageModel,
        net_new_chat_message_event: &mut EventWriter<NetChatMessage>,
        handle_to_entity: &Res<HandleToEntity>,
        attacker_cell_id: &Vec3Int,
        attacked_cell_id: &Vec3Int,
        sensers: &Query<(Entity, &Senser)>,
        attacker_name: &str,
        entity_name: &str,
        _damage_type: &DamageType,
        weapon_name: &str,
        weapon_a_name: &str,
        offense_words: &Vec<String>,
        trigger_words: &Vec<String>,
    ) -> HitResult {
        let (brute_damage, burn_damage, toxin_damage, hit_result) = calculate_damage(
            &self.health_flags,
            &damage_model.damage_flags,
            &damage_model.brute,
            &damage_model.burn,
            &damage_model.toxin,
        );

        let attacker_cell_id_doryen = to_doryen_coordinates(attacker_cell_id.x, attacker_cell_id.z);
        let attacked_cell_id_doryen = to_doryen_coordinates(attacked_cell_id.x, attacked_cell_id.z);

        match &mut self.health_container {
            HealthContainer::Humanoid(humanoid_health) => {
                for (entity, senser) in sensers.iter() {
                    let mut message = "".to_string();

                    let strike_word = offense_words.choose(&mut rand::thread_rng()).unwrap();

                    let attacker_is_visible;

                    if senser.fov.is_in_fov(
                        attacker_cell_id_doryen.0 as usize,
                        attacker_cell_id_doryen.1 as usize,
                    ) {
                        attacker_is_visible = true;
                    } else {
                        attacker_is_visible = false;
                    }

                    let attacked_is_visible;

                    if senser.fov.is_in_fov(
                        attacked_cell_id_doryen.0 as usize,
                        attacked_cell_id_doryen.1 as usize,
                    ) {
                        attacked_is_visible = true;
                    } else {
                        attacked_is_visible = false;
                    }

                    let mut send_message = false;

                    if attacker_is_visible && attacked_is_visible {
                        send_message = true;
                        if body_part == "head" {
                            humanoid_health.head_brute += brute_damage;
                            humanoid_health.head_burn += burn_damage;
                            humanoid_health.head_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the head with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "torso" {
                            humanoid_health.torso_brute += brute_damage;
                            humanoid_health.torso_burn += burn_damage;
                            humanoid_health.torso_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the torso with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_arm" {
                            humanoid_health.right_arm_brute += brute_damage;
                            humanoid_health.right_arm_burn += burn_damage;
                            humanoid_health.right_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the right arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_arm" {
                            humanoid_health.left_arm_brute += brute_damage;
                            humanoid_health.left_arm_burn += burn_damage;
                            humanoid_health.left_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the left arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_leg" {
                            humanoid_health.right_leg_brute += brute_damage;
                            humanoid_health.right_leg_burn += burn_damage;
                            humanoid_health.right_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the right leg with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_leg" {
                            humanoid_health.left_leg_brute += brute_damage;
                            humanoid_health.left_leg_burn += burn_damage;
                            humanoid_health.left_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + attacker_name
                                + " has "
                                + strike_word
                                + " "
                                + entity_name
                                + " in the left leg with "
                                + weapon_a_name
                                + "![/color]";
                        }
                    } else if attacker_is_visible && !attacked_is_visible {
                        send_message = true;
                        let trigger_word = trigger_words.choose(&mut rand::thread_rng()).unwrap();
                        message = "[color=#ff003c]".to_string()
                            + attacker_name
                            + " has "
                            + trigger_word
                            + " his "
                            + weapon_name
                            + "![/color]";
                    } else if !attacker_is_visible && attacked_is_visible {
                        send_message = true;
                        if body_part == "head" {
                            humanoid_health.head_brute += brute_damage;
                            humanoid_health.head_burn += burn_damage;
                            humanoid_health.head_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the head with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "torso" {
                            humanoid_health.torso_brute += brute_damage;
                            humanoid_health.torso_burn += burn_damage;
                            humanoid_health.torso_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the torso with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_arm" {
                            humanoid_health.right_arm_brute += brute_damage;
                            humanoid_health.right_arm_burn += burn_damage;
                            humanoid_health.right_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the right arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_arm" {
                            humanoid_health.left_arm_brute += brute_damage;
                            humanoid_health.left_arm_burn += burn_damage;
                            humanoid_health.left_arm_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the left arm with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "right_leg" {
                            humanoid_health.right_leg_brute += brute_damage;
                            humanoid_health.right_leg_burn += burn_damage;
                            humanoid_health.right_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the right leg with "
                                + weapon_a_name
                                + "![/color]";
                        } else if body_part == "left_leg" {
                            humanoid_health.left_leg_brute += brute_damage;
                            humanoid_health.left_leg_burn += burn_damage;
                            humanoid_health.left_leg_toxin += toxin_damage;

                            message = "[color=#ff003c]".to_string()
                                + entity_name
                                + " has been "
                                + strike_word
                                + " in the left leg with "
                                + weapon_a_name
                                + "![/color]";
                        }
                    }

                    if send_message {
                        match handle_to_entity.inv_map.get(&entity) {
                            Some(handle) => {
                                net_new_chat_message_event.send(NetChatMessage {
                                    handle: *handle,
                                    message: ReliableServerMessage::ChatMessage(message.clone()),
                                });
                            }
                            None => {}
                        }
                    }
                }
            }
            HealthContainer::Entity(item) => {
                item.brute += brute_damage;
                item.burn += burn_damage;
                item.toxin += toxin_damage;

                for (entity, senser) in sensers.iter() {
                    let mut message = "".to_string();

                    let strike_word = offense_words.choose(&mut rand::thread_rng()).unwrap();

                    let attacker_is_visible;

                    if senser.fov.is_in_fov(
                        attacker_cell_id_doryen.0 as usize,
                        attacker_cell_id_doryen.1 as usize,
                    ) {
                        attacker_is_visible = true;
                    } else {
                        attacker_is_visible = false;
                    }

                    let attacked_is_visible;

                    if senser.fov.is_in_fov(
                        attacked_cell_id_doryen.0 as usize,
                        attacked_cell_id_doryen.1 as usize,
                    ) {
                        attacked_is_visible = true;
                    } else {
                        attacked_is_visible = false;
                    }

                    let mut should_send = false;

                    if attacker_is_visible && attacked_is_visible {
                        message = "[color=#ff003c]".to_string()
                            + attacker_name
                            + " has "
                            + strike_word
                            + " "
                            + entity_name
                            + " with "
                            + weapon_a_name
                            + "![/color]";
                        should_send = true;
                    } else if attacker_is_visible && !attacked_is_visible {
                        let trigger_word = trigger_words.choose(&mut rand::thread_rng()).unwrap();
                        message = "[color=#ff003c]".to_string()
                            + attacker_name
                            + " has "
                            + trigger_word
                            + " his "
                            + weapon_a_name
                            + "![/color]";
                        should_send = true;
                    } else if !attacker_is_visible && attacked_is_visible {
                        message = "[color=#ff003c]".to_string()
                            + entity_name
                            + " has been "
                            + strike_word
                            + " with "
                            + weapon_a_name
                            + "![/color]";
                        should_send = true;
                    }

                    if should_send {
                        match handle_to_entity.inv_map.get(&entity) {
                            Some(handle) => {
                                net_new_chat_message_event.send(NetChatMessage {
                                    handle: *handle,
                                    message: ReliableServerMessage::ChatMessage(message.clone()),
                                });
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        hit_result
    }
}

impl Default for EntityContainer {
    fn default() -> Self {
        Self {
            brute: 0.,
            burn: 0.,
            toxin: 0.,
        }
    }
}

impl Default for HumanoidHealth {
    fn default() -> Self {
        Self {
            head_brute: 0.,
            head_burn: 0.,
            head_toxin: 0.,

            torso_brute: 0.,
            torso_burn: 0.,
            torso_toxin: 0.,

            left_arm_brute: 0.,
            left_arm_burn: 0.,
            left_arm_toxin: 0.,

            right_arm_brute: 0.,
            right_arm_burn: 0.,
            right_arm_toxin: 0.,

            right_leg_brute: 0.,
            right_leg_burn: 0.,
            right_leg_toxin: 0.,

            left_leg_brute: 0.,
            left_leg_burn: 0.,
            left_leg_toxin: 0.,
        }
    }
}
