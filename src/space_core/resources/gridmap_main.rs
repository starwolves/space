use std::collections::HashMap;
use bevy::{math::Vec3, prelude::{Entity, EventWriter, FromWorld, Res, World}};
use rand::prelude::SliceRandom;
use serde::{Deserialize};

use crate::space_core::{components::{health::{DamageModel, DamageType, HealthFlag, HitResult, MELEE_STRIKE_WORDS, calculate_damage}, inventory_item::HitSoundSurface}, events::net::net_chat_message::NetChatMessage, functions::entity::new_chat_message::{new_proximity_message}};

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
