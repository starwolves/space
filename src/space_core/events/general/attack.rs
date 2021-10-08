use bevy::{math::Vec3, prelude::Entity};

use crate::space_core::components::{health::DamageModel, inventory_item::{CombatSoundSet, CombatType}};

pub struct Attack {

    pub attacker_entity : Entity,
    pub attacker_sensed_by : Vec<Entity>,
    pub attacker_sensed_by_cached : Vec<Entity>,
    pub attacker_name : String,
    pub weapon_entity : Option<Entity>,
    pub targetted_limb : String,
    pub position : Vec3,
    pub angle : f32,
    pub damage_model : DamageModel,
    pub range : f32,
    pub combat_type : CombatType,
    pub combat_sound_set : CombatSoundSet,

}
