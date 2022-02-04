use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use bevy::core::Timer;
use bevy::prelude::{Component, Transform};
use bevy::{math::Vec2, prelude::{Entity}};
use doryen_fov::FovRecursiveShadowCasting;
use crate::space_core::generics::gridmap::resources::{CellData, Vec3Int, Vec2Int, FOV_MAP_WIDTH, FOV_MAP_HEIGHT};
use crate::space_core::generics::health::components::{DamageModel, DamageFlag};
use crate::space_core::generics::inventory::components::Inventory;
use crate::space_core::generics::inventory_item::components::CombatSoundSet;
use crate::space_core::generics::networking::resources::{GridMapType, NetTabAction};

#[derive(Component)]
pub struct Boarding;

#[derive(Component)]
pub struct ConnectedPlayer {
    pub handle : u32,
    pub authid : u16,
    pub rcon : bool,
    pub connected : bool,
}

impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: 0,
            authid: 0,
            rcon : true,
            connected : true,
        }
    }
}

#[derive(Component)]
pub struct LinkedFootstepsSprinting{
    pub entity :Entity
}

#[derive(Component)]
pub struct LinkedFootstepsWalking{
    pub entity :Entity
}


#[derive(Clone)]
pub struct TabAction {
    pub id : String,
    pub text : String,
    pub tab_list_priority : u8,
    pub belonging_entity : Option<Entity>,
    pub prerequisite_check : Arc<dyn Fn(
        Option<Entity>,
        Option<u64>,
        Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
        f32,
        &Inventory,
    ) -> bool + Sync + Send>,
}

impl TabAction {
    pub fn into_net(&self, item_name : &str, entity_option : Option<u64>, cell_option : Option<(GridMapType, i16,i16,i16)>) -> NetTabAction {
        let self_belonging_entity;

        match self.belonging_entity {
            Some(rr) => {
                self_belonging_entity = Some(rr.to_bits());
            },
            None => {
                self_belonging_entity = None;
            },
        }

        NetTabAction {
            id: self.id.clone(),
            text: self.text.clone(),
            tab_list_priority: self.tab_list_priority,
            entity_option : entity_option,
            cell_option,
            item_name : item_name.to_string(),
            belonging_entity: self_belonging_entity,
        }
    }
}

#[derive(Component)]
pub struct Pawn {
    pub name : String,
    pub job : SpaceJobsEnum,
    pub facing_direction : FacingDirection,
    pub tab_actions : HashMap<u32, TabAction>,
    pub tab_actions_data : TabActionsData,
}

pub struct TabActionsData {
    pub layout : HashMap<Option<Entity>, HashMap<String, u32>>,
    pub tab_action_i : u32,
}

impl Default for TabActionsData {
    fn default() -> Self {
        Self {
            layout: HashMap::new(),
            tab_action_i:0,
        }
    }
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: SpaceJobsEnum::Security,
            facing_direction: FacingDirection::Up,
            tab_actions : HashMap::new(),
            tab_actions_data: TabActionsData::default(),
        }
    }
}


#[derive(Debug, Clone)]
pub enum FacingDirection {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

pub fn facing_direction_to_direction(direction : &FacingDirection) -> Vec2 {
    match direction {
        FacingDirection::UpLeft => {
            Vec2::new(-1.,1.)
        },
        FacingDirection::Up => {
            Vec2::new(0.,1.)
        },
        FacingDirection::UpRight => {
            Vec2::new(1. ,1.)
        },
        FacingDirection::Right => {
            Vec2::new(1., 0.)
        },
        FacingDirection::DownRight => {
            Vec2::new(1. , -1.)
        },
        FacingDirection::Down => {
            Vec2::new(0.,-1.)
        },
        FacingDirection::DownLeft => {
            Vec2::new(-1.,-1.)
        },
        FacingDirection::Left => {
            Vec2::new(-1.,0.)
        },
    }
}

#[derive(Copy, Clone)]
pub enum SpaceJobsEnum {
    Security,
    Control
}


#[derive(PartialEq)]
pub enum SpaceAccessEnum {
    Security,
    Common,
}

impl Pawn {

    pub fn tab_actions_add(&mut self, tab_action_id : &str, entity_option : Option<Entity>, tab_action : TabAction) {
        
        let entity_tab_ids;

        match  self.tab_actions_data.layout.contains_key(&entity_option) {
            true => {
                entity_tab_ids = self.tab_actions_data.layout.get_mut(&entity_option).unwrap();
            },
            false => {
                self.tab_actions_data.layout.insert(entity_option, HashMap::new());
                entity_tab_ids = self.tab_actions_data.layout.get_mut(&entity_option).unwrap();
            },
        }

        entity_tab_ids.insert(tab_action_id.to_string(), self.tab_actions_data.tab_action_i);
        self.tab_actions.insert(self.tab_actions_data.tab_action_i, tab_action);
        self.tab_actions_data.tab_action_i+=1;
        
    }
    pub fn tab_actions_remove_entity(&mut self, entity_option : Option<Entity>) {

        let entity_tab_ids;

        match self.tab_actions_data.layout.get_mut(&entity_option) {
            Some(s) => {
                entity_tab_ids=s;
            },
            None => {
                return;
            },
        };

        for (_s, hashmap_index) in entity_tab_ids.iter() {
            self.tab_actions.remove(hashmap_index);
        }

        self.tab_actions_data.layout.remove(&entity_option);

    }

}

#[derive(Clone, Component)]
pub struct PersistentPlayerData {
    pub user_name_is_set : bool,
    pub character_name : String,
    pub user_name : String,
}

impl Default for PersistentPlayerData {
    fn default() -> Self {
        Self {
            user_name_is_set : false,
            character_name: "".to_string(),
            user_name: "".to_string(),
        }
    }
}



#[derive(Component)]
pub struct PlayerInput {
    pub movement_vector : Vec2,
    pub sprinting : bool,
    pub is_mouse_action_pressed : bool,
    pub targetted_limb : String,
    pub auto_move_enabled : bool,
    pub auto_move_direction : Vec2,
    pub combat_targetted_entity : Option<Entity>,
    pub combat_targetted_cell : Option<Vec3Int>,
    pub alt_attack_mode : bool,
    pub pending_direction : Option<FacingDirection>,
}


impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            movement_vector : Vec2::ZERO,
            sprinting : false,
            is_mouse_action_pressed : false,
            targetted_limb : "torso".to_string(),
            auto_move_enabled : false,
            auto_move_direction : Vec2::ZERO,
            combat_targetted_entity: None,
            combat_targetted_cell: None,
            alt_attack_mode: false,
            pending_direction: None,
        }
    }
}

#[derive(Component)]
pub struct Radio {
    pub listen_access : Vec<RadioChannel>,
    pub speak_access : Vec<RadioChannel>
}

#[derive(PartialEq, Debug, Clone)]
pub enum RadioChannel {
    Proximity,
    ProximityEmote,
    Global,
    Common,
    Security,
    SpecialOps
}

#[derive(Component)]
pub struct Senser {
    pub cell_id : Vec2Int,
    pub fov : FovRecursiveShadowCasting,
    pub sensing : Vec<Entity>,
}

impl Default for Senser {
    fn default() -> Self {
        Self {
            cell_id: Vec2Int{
                x: 0,
                y: 0
            },
            fov: FovRecursiveShadowCasting::new(FOV_MAP_WIDTH, FOV_MAP_HEIGHT),
            sensing: vec![],
        }
    }
}

#[derive(Component)]
pub struct SoftPlayer;

#[derive(Component)]
pub struct SpaceAccess {
    pub access : Vec<SpaceAccessEnum>
}

#[derive(Component)]
pub struct Spawning {
    pub transform : Transform
}


#[derive(Component)]
pub struct StandardCharacter {
    pub current_lower_animation_state : CharacterAnimationState,
    pub character_name : String,
    pub combat_mode : bool,
    pub facing_direction : f32,
    pub is_attacking : bool,
    pub next_attack_timer : Timer,
    pub default_melee_damage_model : DamageModel,
    pub default_melee_sound_set : CombatSoundSet,
}

pub enum CharacterAnimationState {
    Idle,
    Jogging,
    Sprinting,
}

const FIRST_MELEE_TIME : u64 = 433;

impl Default for StandardCharacter {
    fn default() -> Self {
        let mut t = Timer::new(Duration::from_millis(FIRST_MELEE_TIME), false);
        let mut first_damage_flags = HashMap::new();
        first_damage_flags.insert(0, DamageFlag::SoftDamage);
        t.tick(Duration::from_millis(FIRST_MELEE_TIME));
        Self {
            current_lower_animation_state : CharacterAnimationState::Idle,
            character_name: "".to_string(),
            combat_mode : false,
            facing_direction : 0.,
            is_attacking : false,
            next_attack_timer : t,
            default_melee_damage_model: DamageModel {
                brute: 5.,
                damage_flags : first_damage_flags,
                ..Default::default()
            },
            default_melee_sound_set: CombatSoundSet::default(),
        }
    }
}

#[derive(Component)]
pub struct SetupPhase;

#[derive(Component)]
pub struct OnBoard;
