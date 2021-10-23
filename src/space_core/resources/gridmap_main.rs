use std::collections::HashMap;
use bevy::{prelude::{Entity, EventWriter, FromWorld, Query, Res, World}};
use rand::prelude::SliceRandom;
use serde::{Deserialize};

use crate::space_core::{components::{health::{DamageModel, DamageType, HealthFlag, HitResult, calculate_damage}, inventory_item::HitSoundSurface, senser::Senser}, events::net::net_chat_message::NetChatMessage};

use super::{doryen_fov::{Vec3Int, to_doryen_coordinates}, handle_to_entity::HandleToEntity, network_messages::ReliableServerMessage};


pub struct GridmapMain {
    pub data : HashMap<Vec3Int, CellData>
}

#[derive(Deserialize)]
pub struct CellDataWID {
    pub id: String,
    pub item: String,
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
        attacker_cell_id: &Vec3Int,
        attacked_cell_id : &Vec3Int,
        sensers : &Query<(Entity, &Senser)>,
        attacker_name : &str,
        cell_name : &str,
        _damage_type : &DamageType,
        weapon_name : &str,
        weapon_a_name : &str,
        offense_words : &Vec<String>,
        trigger_words : &Vec<String>,
    ) -> HitResult {

        let (
            brute_damage,
            burn_damage,
            toxin_damage,
            hit_result,
        ) = calculate_damage(
            &self.health_flags,
            &damage_model.damage_flags,
            &damage_model.brute,
            &damage_model.burn,
            &damage_model.toxin
        );

        let attacker_cell_id_doryen = to_doryen_coordinates(attacker_cell_id.x, attacker_cell_id.z);
        let attacked_cell_id_doryen = to_doryen_coordinates(attacked_cell_id.x, attacked_cell_id.z);

        self.brute+=brute_damage;
        self.burn+=burn_damage;
        self.toxin+=toxin_damage;

        for (entity, senser) in sensers.iter() {

            let mut message = "".to_string();

            let strike_word = offense_words.choose(&mut rand::thread_rng()).unwrap();

            let attacker_is_visible;

            if senser.fov.is_in_fov(attacker_cell_id_doryen.0 as usize, attacker_cell_id_doryen.1 as usize) {
                attacker_is_visible=true;
            } else {
                attacker_is_visible=false;
            }

            let attacked_is_visible;

            if senser.fov.is_in_fov(attacked_cell_id_doryen.0 as usize, attacked_cell_id_doryen.1 as usize) {
                attacked_is_visible=true;
            } else {
                attacked_is_visible=false;
            }

            let mut should_send = false;

            if attacker_is_visible && attacked_is_visible {
                message = "[color=#ff003c]".to_string() + attacker_name + " has " + strike_word + " " + cell_name + " with " + weapon_a_name + "![/color]";
                should_send=true;
            } else if attacker_is_visible && !attacked_is_visible {
                let trigger_word = trigger_words.choose(&mut rand::thread_rng()).unwrap();
                message = "[color=#ff003c]".to_string() + attacker_name + " has " + trigger_word + " his " + weapon_name + "![/color]";
                should_send=true;
            } else if !attacker_is_visible && attacked_is_visible {
                message = "[color=#ff003c]".to_string() + cell_name + " has been " + strike_word + " with " + weapon_a_name + "![/color]";
                should_send=true;
            }

            if should_send {
                match handle_to_entity.inv_map.get(&entity) {
                    Some(handle) => {
                        net_new_chat_message_event.send(NetChatMessage {
                            handle: *handle,
                            message: ReliableServerMessage::ChatMessage(message.clone()),
                        });
                    },
                    None => {},
                }
            }

        }

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
