use std::collections::HashMap;
use bevy::prelude::{FromWorld, World};
use serde::{Deserialize};

use crate::space_core::components::{health::{DamageFlag, DamageModel}, inventory_item::HitSoundSurface};

use super::doryen_fov::Vec3Int;


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
    pub health_flags : HashMap<u32, StructureHealthFlag>,
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

    pub fn apply_damage(&mut self, _body_part : &str, damage_model : &DamageModel) {

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

        if damager_flags.contains(&&DamageFlag::SoftDamage) && structure_health_flags.contains(&&StructureHealthFlag::ArmourPlated)  {
            brute_damage = 0.;
        }

        self.brute+=brute_damage;
        self.burn+=burn_damage;
        self.toxin+=toxin_damage;

    }

}


#[derive(Clone, PartialEq)]
pub enum StructureHealthFlag {
    ArmourPlated,
}

impl FromWorld for GridmapMain {
    fn from_world(_world: &mut World) -> Self {
        GridmapMain {
           data : HashMap::new(), 
        }
    }
}
