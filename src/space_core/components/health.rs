use std::{collections::HashMap};

use bevy::{math::Vec3, prelude::{Entity, EventWriter, Res}};
use rand::prelude::SliceRandom;

use crate::space_core::{events::net::net_chat_message::NetChatMessage, functions::entity::new_chat_message::new_proximity_message, resources::handle_to_entity::HandleToEntity};

use super::inventory_item::HitSoundSurface;

pub struct Health {

    pub health_container : HealthContainer,
    pub health_flags : HashMap<u32, HealthFlag>,
    pub raegent_container : RaegentContainer,
    pub hit_sound_surface : HitSoundSurface,
}


pub const MELEE_STRIKE_WORDS : [&str; 3] = [
    "hit",
    "struck",
    "striked"
];

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
    raegents : HashMap<String, f32>,
}

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum DamageFlag {
    SoftDamage, //Ie fists.
    Stun(f32),
    Floor(f32),
}


#[derive(Debug)]
pub struct HumanoidHealth {

    pub head_brute : f32,
    pub head_burn: f32,
    pub head_toxin: f32,

    pub torso_brute: f32,
    pub torso_burn: f32,
    pub torso_toxin: f32,

    pub left_arm_brute : f32,
    pub left_arm_burn : f32,
    pub left_arm_toxin : f32,

    pub right_arm_brute : f32,
    pub right_arm_burn : f32,
    pub right_arm_toxin : f32,

    pub right_leg_brute : f32,
    pub right_leg_burn : f32,
    pub right_leg_toxin : f32,

    pub left_leg_brute : f32,
    pub left_leg_burn : f32,
    pub left_leg_toxin : f32,

}

pub struct EntityContainer {

    pub brute : f32,
    pub burn : f32,
    pub toxin : f32,

}


pub struct DamageModel {
    pub brute : f32,
    pub burn : f32,
    pub toxin : f32,
    pub damage_flags : HashMap<u32, DamageFlag>,
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

    health_flags : &HashMap<u32,HealthFlag>,
    damage_flags : &HashMap<u32,DamageFlag>,

    mut brute : f32,
    burn : f32,
    toxin : f32,

) -> (f32,f32,f32, HitResult){

    let mut hit_result = HitResult::HitSoft;

    let mut damager_flags = vec![];

    for damage_flag in damage_flags.values() {
        damager_flags.push(damage_flag);
    }

    let mut structure_health_flags = vec![];

    for stucture_health_flag in health_flags.values() {
        structure_health_flags.push(stucture_health_flag);
    }

    if damager_flags.contains(&&DamageFlag::SoftDamage) && structure_health_flags.contains(&&HealthFlag::ArmourPlated)  {
        brute = 0.;
        hit_result = HitResult::Blocked;
    }

    (brute,burn,toxin,hit_result)

}

pub enum DamageType {
    Melee,
    Projectile,
}

impl Health {

    pub fn apply_damage(
        &mut self, 
        body_part : &str, 
        damage_model : &DamageModel,
        net_new_chat_message_event: &mut EventWriter<NetChatMessage>,
        handle_to_entity: &Res<HandleToEntity>, 
        sensed_by: &Vec<Entity>, 
        sensed_by_distance: &Vec<Entity>, 
        position: Vec3,
        attacker_name : &str,
        entity_name : &str,
        _damage_type : &DamageType,
    ) -> HitResult {

        let (
            brute_damage,
            burn_damage,
            toxin_damage,
            hit_result,
        ) = calculate_damage(
            &self.health_flags,
            &damage_model.damage_flags,
            damage_model.brute,
            damage_model.burn,
            damage_model.toxin
        );

        match &mut self.health_container {
            HealthContainer::Humanoid(humanoid_health) => {

                let mut message = "".to_string();

                let strike_word = MELEE_STRIKE_WORDS.choose(&mut rand::thread_rng()).unwrap();

                if body_part == "head" {
                    humanoid_health.head_brute+=brute_damage;
                    humanoid_health.head_burn+=burn_damage;
                    humanoid_health.head_toxin+=toxin_damage;

                    message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + entity_name + " in the head![/color]";

                
                } else if body_part == "torso" {
                    humanoid_health.torso_brute+=brute_damage;
                    humanoid_health.torso_burn+=burn_damage;
                    humanoid_health.torso_toxin+=toxin_damage;

                    message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + entity_name + " in the torso![/color]";

                } else if body_part == "right_arm" {
                    humanoid_health.right_arm_brute+=brute_damage;
                    humanoid_health.right_arm_burn+=burn_damage;
                    humanoid_health.right_arm_toxin+=toxin_damage;

                    message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + entity_name + " in the right arm![/color]";

                } else if body_part == "left_arm" {
                    humanoid_health.left_arm_brute+=brute_damage;
                    humanoid_health.left_arm_burn+=burn_damage;
                    humanoid_health.left_arm_toxin+=toxin_damage;

                    message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + entity_name + " in the left arm![/color]";

                } else if body_part == "right_leg" {
                    humanoid_health.right_leg_brute+=brute_damage;
                    humanoid_health.right_leg_burn+=burn_damage;
                    humanoid_health.right_leg_toxin+=toxin_damage;

                    message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + entity_name + " in the right leg![/color]";

                } else if body_part == "left_leg" {
                    humanoid_health.left_leg_brute+=brute_damage;
                    humanoid_health.left_leg_burn+=burn_damage;
                    humanoid_health.left_leg_toxin+=toxin_damage;

                    message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + entity_name + " in the left leg![/color]";

                }

                new_proximity_message(
                    net_new_chat_message_event,
                    handle_to_entity,
                    sensed_by,
                    sensed_by_distance,
                    position,
                    message,
                );

            },
            HealthContainer::Entity(item) => {

                item.brute+=brute_damage;
                item.burn+=burn_damage;
                item.toxin+=toxin_damage;

                let strike_word = MELEE_STRIKE_WORDS.choose(&mut rand::thread_rng()).unwrap();

                let message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + entity_name + "![/color]";

                new_proximity_message(
                    net_new_chat_message_event,
                    handle_to_entity,
                    sensed_by,
                    sensed_by_distance,
                    position,
                    message,
                );

            },
        }

        hit_result

    }

}

impl Default for Health {
    fn default() -> Self {
        Self {
            health_container : HealthContainer::Entity(EntityContainer::default()),
            health_flags: HashMap::new(),
            raegent_container : RaegentContainer {
                raegents: HashMap::new(),
            },
            hit_sound_surface: HitSoundSurface::Soft,
        }
    }
}

impl Default for EntityContainer {
    fn default() -> Self {
        Self {
            brute:0.,
            burn:0.,
            toxin:0.,
        }
    }
}

impl Default for HumanoidHealth {

    fn default() -> Self {
        Self {
        
            head_brute : 0.,
            head_burn: 0.,
            head_toxin: 0.,

            torso_brute: 0.,
            torso_burn: 0.,
            torso_toxin: 0.,

            left_arm_brute : 0.,
            left_arm_burn : 0.,
            left_arm_toxin : 0.,

            right_arm_brute : 0.,
            right_arm_burn : 0.,
            right_arm_toxin : 0.,

            right_leg_brute : 0.,
            right_leg_burn : 0.,
            right_leg_toxin : 0.,

            left_leg_brute : 0.,
            left_leg_burn : 0.,
            left_leg_toxin : 0.,
            
        }
    }

}
