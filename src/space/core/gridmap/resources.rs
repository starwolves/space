use std::collections::HashMap;

use bevy::{prelude::{FromWorld, World, Entity, EventWriter, Res, Query, Transform}, math::Quat};
use doryen_fov::MapData;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};

use crate::space::{core::{inventory_item::components::HitSoundSurface, health::components::{HealthFlag, DamageModel, DamageType, HitResult, calculate_damage}, pawn::{events::NetChatMessage, components::Senser, resources::HandleToEntity}, entity::{components::RichName, functions::string_to_type_converters::string_transform_to_transform}, networking::resources::ReliableServerMessage}};

use super::MainCellProperties;


pub struct GridmapData {
    pub non_fov_blocking_cells_list: Vec<i64>,
    pub non_combat_obstacle_cells_list: Vec<i64>,
    pub non_laser_obstacle_cells_list: Vec<i64>,
    pub placeable_items_cells_list: Vec<i64>,
    pub ordered_main_names : Vec<String>,
    pub ordered_details1_names: Vec<String>,
    pub main_name_id_map : HashMap<String, i64>,
    pub main_id_name_map : HashMap<i64, String>,
    pub details1_name_id_map: HashMap<String, i64>,
    pub details1_id_name_map: HashMap<i64,String>,
    pub main_text_names : HashMap<i64, RichName>,
    pub details1_text_names  : HashMap<i64, RichName>,
    pub main_text_examine_desc : HashMap<i64, String>,
    pub details1_text_examine_desc : HashMap<i64, String>,
    pub blackcell_id : i64,
    pub blackcell_blocking_id : i64,
    pub main_cell_properties : HashMap<i64, MainCellProperties>,
}

impl FromWorld for GridmapData {
    fn from_world(_world: &mut World) -> Self {
        GridmapData {
            non_fov_blocking_cells_list : vec![],
            non_combat_obstacle_cells_list : vec![],
            non_laser_obstacle_cells_list : vec![],
            placeable_items_cells_list : vec![],
            ordered_main_names : vec![],
            ordered_details1_names : vec![],
            main_name_id_map : HashMap::new(),
            main_id_name_map : HashMap::new(),
            details1_name_id_map : HashMap::new(),
            details1_id_name_map : HashMap::new(),
            main_text_names : HashMap::new(),
            details1_text_names  : HashMap::new(),
            main_text_examine_desc : HashMap::new(),
            details1_text_examine_desc : HashMap::new(),
            blackcell_id : 0,
            blackcell_blocking_id : 0,
            main_cell_properties: HashMap::new(),
        }
    }
}

pub struct GridmapDetails1 {
    pub data : HashMap<Vec3Int, CellData>,
    pub updates : HashMap<Vec3Int, CellUpdate>,
}


impl FromWorld for GridmapDetails1 {
    fn from_world(_world: &mut World) -> Self {
        GridmapDetails1 {
           data : HashMap::new(),
           updates : HashMap::new(),
        }
    }
}


pub struct GridmapMain {
    pub data : HashMap<Vec3Int, CellData>,
    pub updates : HashMap<Vec3Int, CellUpdate>,
}


pub struct CellUpdate {
    pub entities_received : Vec<Entity>,
    pub cell_data : CellData,
}

#[derive(Deserialize)]
pub struct CellDataWID {
    pub id: String,
    pub item: String,
    pub orientation: i64
}


#[derive(Clone)]
pub struct CellData {
    pub item: i64,
    pub orientation: i64,
    pub health : StructureHealth,
    pub entity : Option<Entity>,
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
            updates: HashMap::new(), 
        }
    }
}


pub struct DoryenMap {

    pub map : MapData,

}

impl FromWorld for DoryenMap {
    fn from_world(_world: &mut World) -> Self {

        DoryenMap {
            
            map : MapData::new(FOV_MAP_WIDTH, FOV_MAP_WIDTH),

        }
    }
}

pub fn to_doryen_coordinates(x : i16, y : i16) -> (usize, usize){

    let mut n_x=x+FOV_MAP_WIDTH as i16/2;
    let mut n_y=y+FOV_MAP_WIDTH as i16/2;

    if doryen_coordinates_out_of_range(n_x as usize, n_y as usize) {
        n_x=0;
        n_y=0;
    }

    (n_x as usize,n_y as usize)

}


pub fn doryen_coordinates_out_of_range(x : usize, y : usize) -> bool {
    x > FOV_MAP_WIDTH || y > FOV_MAP_WIDTH
}


#[derive(PartialEq,Eq, Hash, Copy, Clone, Debug)]
pub struct Vec2Int {
    pub x : i16,
    pub y : i16,   
}


impl Default for Vec2Int {
    fn default() -> Self {
        Self {
            x:0,
            y:0,
        }
    }
}


#[derive(PartialEq,Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Vec3Int {
    pub x : i16,
    pub y : i16,  
    pub z : i16,  
}


// Turning up these values drastically increases fov calculation time.
// The largest maps we can support with f32 accuracy is a 2000x2000 tiled map.
// FOV calculation time will take 10x-15x slower, up to 2-3ms for just a single player calculation.
// For bigger maps than 500x500 gridmaps we need a new and better FOV algorithm.
pub const FOV_MAP_WIDTH : usize = 500;



#[derive(Deserialize)]
pub struct SpawnPointRaw {
    pub point_type : String,
    pub transform : String
}

pub struct SpawnPoint {
    pub point_type : String,
    pub transform : Transform
}

pub struct SpawnPoints {
    pub list : Vec<SpawnPoint>,
    pub i : usize
}

impl SpawnPoint {
    pub fn new(raw : &SpawnPointRaw) -> SpawnPoint {

        let mut this_transform = string_transform_to_transform(&raw.transform);
        
        this_transform.translation.y = 0.05;

        this_transform.rotation = Quat::IDENTITY;


        SpawnPoint {
            point_type : raw.point_type.clone(),
            transform : this_transform
        }


    }
}

impl FromWorld for SpawnPoints {
    fn from_world(_world: &mut World) -> Self {
        SpawnPoints {
            list : vec![],
            i : 0,
        }
    }
}
