use std::collections::HashMap;
use bevy::{math::Vec3, prelude::{Entity, EventWriter, FromWorld, Res, World}};
use rand::prelude::SliceRandom;
use serde::{Deserialize};

use crate::space_core::{components::{health::{DamageFlag, DamageModel, HealthFlag, HitResult, MELEE_STRIKE_WORDS}, inventory_item::HitSoundSurface}, events::net::net_chat_message::NetChatMessage, functions::entity::new_chat_message::{new_proximity_message}};

use super::{doryen_fov::Vec3Int, handle_to_entity::HandleToEntity};


pub struct GridmapMain {
    pub data : HashMap<Vec3Int, CellData>
}

#[derive(Deserialize)]
pub struct CellDataWID {
    pub id: String,
    pub item: i64,
    pub orientation: i64
}

pub struct CellData {
    pub item: i64,
    pub orientation: i64,
    pub health : StructureHealth,
}


#[derive(Clone)]
pub struct StructureHealth {
    pub brute : f32,
    pub burn : f32,
    pub toxin : f32,
    pub health_flags : HashMap<u32, HealthFlag>,
    pub hit_sound_surface : HitSoundSurface,
}

impl Default for StructureHealth {
    fn default() -> Self {
        Self {
            brute:0.,
            burn:0.,
            toxin:0.,
            health_flags:HashMap::new(),
            hit_sound_surface: HitSoundSurface::Metaloid,
        }
    }
}

impl StructureHealth {

    pub fn apply_damage(
        &mut self, 
        _body_part : &str, 
        damage_model : &DamageModel,
        net_new_chat_message_event: &mut EventWriter<NetChatMessage>,
        handle_to_entity: &Res<HandleToEntity>, 
        sensed_by: &Vec<Entity>, 
        sensed_by_distance: &Vec<Entity>, 
        position: Vec3,
        attacker_name : &str,
        cell_name : &str,
    ) -> HitResult {

        let mut hit_result = HitResult::HitSoft;

        let mut damager_flags = vec![];

        for damage_flag in damage_model.damage_flags.values() {
            damager_flags.push(damage_flag);
        }

        let mut structure_health_flags = vec![];

        for stucture_health_flag in self.health_flags.values() {
            structure_health_flags.push(stucture_health_flag);
        }

        let mut brute_damage = damage_model.brute;
        let burn_damage = damage_model.burn;
        let toxin_damage = damage_model.toxin;

        if damager_flags.contains(&&DamageFlag::SoftDamage) && structure_health_flags.contains(&&HealthFlag::ArmourPlated)  {
            brute_damage = 0.;
            hit_result = HitResult::Blocked;
        }

        self.brute+=brute_damage;
        self.burn+=burn_damage;
        self.toxin+=toxin_damage;

        let strike_word = MELEE_STRIKE_WORDS.choose(&mut rand::thread_rng()).unwrap();

        let message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + cell_name + "![/color]";

        new_proximity_message(
            net_new_chat_message_event,
            handle_to_entity,
            sensed_by,
            sensed_by_distance,
            position,
            message,
        );

        hit_result

    }

}

impl FromWorld for GridmapMain {
    fn from_world(_world: &mut World) -> Self {
        GridmapMain {
           data : HashMap::new(), 
        }
    }
}
