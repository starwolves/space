use std::collections::HashMap;

use bevy::prelude::{Color, Commands, Entity, ResMut, Transform};
use rand::prelude::SliceRandom;

use crate::space_core::{bundles::{block1_sfx::{BLOCK1_PLAY_BACK_DURATION, Block1SfxBundle}, block2_sfx::{BLOCK2_PLAY_BACK_DURATION, Block2SfxBundle}, block3_sfx::{BLOCK3_PLAY_BACK_DURATION, Block3SfxBundle}, punch1_sfx::{PUNCH1_PLAY_BACK_DURATION, Punch1SfxBundle}, punch2_sfx::{PUNCH2_PLAY_BACK_DURATION, Punch2SfxBundle}, punch3_sfx::{PUNCH3_PLAY_BACK_DURATION, Punch3SfxBundle}, punch4_sfx::{PUNCH4_PLAY_BACK_DURATION, Punch4SfxBundle}, swing1_sfx::{SWING1_PLAY_BACK_DURATION, Swing1SfxBundle}, swing2_sfx::{SWING2_PLAY_BACK_DURATION, Swing2SfxBundle}, swing3_sfx::{SWING3_PLAY_BACK_DURATION, Swing3SfxBundle}, swing4_sfx::{SWING4_PLAY_BACK_DURATION, Swing4SfxBundle}}, resources::{sfx_auto_destroy_timers::SfxAutoDestroyTimers}};

use super::{health::DamageModel, inventory::SlotType, sfx::sfx_auto_destroy};


pub struct InventoryItem {
    pub in_inventory_of_entity : Option<Entity>,
    pub attachment_transforms : HashMap<String, Transform>,
    pub drop_transform : Transform,
    pub slot_type : SlotType,
    pub is_attached_when_worn : bool,
    pub combat_standard_animation : CombatStandardAnimation,
    pub combat_attack_animation: CombatAttackAnimation,
    pub combat_type: CombatType,
    pub combat_melee_damage_model : DamageModel,
    pub combat_projectile_damage_model: Option<DamageModel>,
    pub combat_sound_set : CombatSoundSet,
}

pub enum CombatStandardAnimation {
    StandardStance,
    PistolStance,
}

pub enum CombatAttackAnimation {
    OneHandedMeleePunch,
    PistolShot,
}

#[derive(Clone)]
pub enum CombatType {
    MeleeDirect,
    Projectile(ProjectileType),
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum ProjectileType {
    Laser(Color, f32, f32, f32),
    Ballistic,
}

#[derive(Clone)]
pub struct CombatSoundSet {
    pub melee_miss : Vec<CombatSound>,
    pub melee_hit_soft : Vec<CombatSound>,
    pub melee_hit_blocked : Vec<CombatSound>,

    pub projectile_hit_soft : Vec<CombatSound>,
    pub projectile_hit_blocked : Vec<CombatSound>,
}


impl CombatSoundSet {

    pub fn spawn_miss_sfx(&self, commands : &mut Commands, transform : Transform, auto_destroy_timers : &mut ResMut<SfxAutoDestroyTimers>) {
        

        match self.melee_miss.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::Swing1 => {
                let sfx_entity = commands.spawn().insert_bundle(Swing1SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,SWING1_PLAY_BACK_DURATION);
            },
            CombatSound::Swing2 => {
                let sfx_entity = commands.spawn().insert_bundle(Swing2SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,SWING2_PLAY_BACK_DURATION);
            },
            CombatSound::Swing3 => {
                let sfx_entity = commands.spawn().insert_bundle(Swing3SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,SWING3_PLAY_BACK_DURATION);
            },
            CombatSound::Swing4 => {
                let sfx_entity = commands.spawn().insert_bundle(Swing4SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,SWING4_PLAY_BACK_DURATION);
            },
            _=> (),
        }

    }

    pub fn spawn_hit_sfx(&self, commands : &mut Commands, transform : Transform, auto_destroy_timers : &mut ResMut<SfxAutoDestroyTimers>) {

        match self.melee_hit_soft.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::Punch1 => {
                let sfx_entity = commands.spawn().insert_bundle(Punch1SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,PUNCH1_PLAY_BACK_DURATION);
            },
            CombatSound::Punch2 => {
                let sfx_entity = commands.spawn().insert_bundle(Punch2SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,PUNCH2_PLAY_BACK_DURATION);
            },
            CombatSound::Punch3 => {
                let sfx_entity = commands.spawn().insert_bundle(Punch3SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,PUNCH3_PLAY_BACK_DURATION);
            },
            CombatSound::Punch4 => {
                let sfx_entity = commands.spawn().insert_bundle(Punch4SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,PUNCH4_PLAY_BACK_DURATION);
            },
            _=> (),
        }

    }

    pub fn spawn_hit_blocked(&self, commands : &mut Commands, transform : Transform, auto_destroy_timers : &mut ResMut<SfxAutoDestroyTimers>) {

        match self.melee_hit_blocked.choose(&mut rand::thread_rng()).unwrap() {
            CombatSound::Block1 => {
                let sfx_entity = commands.spawn().insert_bundle(Block1SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,BLOCK1_PLAY_BACK_DURATION);
            },
            CombatSound::Block2 => {
                let sfx_entity = commands.spawn().insert_bundle(Block2SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,BLOCK2_PLAY_BACK_DURATION); 
            },
            CombatSound::Block3 => {
                let sfx_entity = commands.spawn().insert_bundle(Block3SfxBundle::new(transform)).id();
                sfx_auto_destroy(sfx_entity,auto_destroy_timers,BLOCK3_PLAY_BACK_DURATION);
            },
            _=> (),
        }

    }
}

#[derive(Clone)]
pub enum CombatSound {
    Swing1,
    Swing2,
    Swing3,
    Swing4,
    Punch1,
    Punch2,
    Punch3,
    Punch4,
    Block1,
    Block2,
    Block3,
}

impl Default for CombatSoundSet {
    fn default() -> Self {
        Self {
            melee_miss: vec![
                CombatSound::Swing1,
                CombatSound::Swing2,
                CombatSound::Swing3,
                CombatSound::Swing4,
            ],
            melee_hit_soft: vec![
                CombatSound::Punch1,
                CombatSound::Punch2,
                CombatSound::Punch3,
                CombatSound::Punch4,
            ],
            melee_hit_blocked: vec![
                CombatSound::Block1,
                CombatSound::Block2,
                CombatSound::Block3,
            ],

            projectile_hit_soft: vec![

            ],
            projectile_hit_blocked: vec![

            ],
        }
    }
}

#[derive(Clone)]
pub enum HitSoundSurface {
    Soft,
    Metaloid,
}
