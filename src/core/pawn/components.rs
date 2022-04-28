use crate::core::gridmap::resources::Vec3Int;
use crate::core::tab_actions::components::{TabAction, TabActionsData};

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use std::collections::HashMap;

#[derive(Component)]
pub struct Pawn {
    pub name: String,
    pub job: SpaceJobsEnum,
    pub facing_direction: FacingDirection,
    pub tab_actions: HashMap<u32, TabAction>,
    pub tab_actions_data: TabActionsData,
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: SpaceJobsEnum::Security,
            facing_direction: FacingDirection::Up,
            tab_actions: HashMap::new(),
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

pub fn facing_direction_to_direction(direction: &FacingDirection) -> Vec2 {
    match direction {
        FacingDirection::UpLeft => Vec2::new(-1., 1.),
        FacingDirection::Up => Vec2::new(0., 1.),
        FacingDirection::UpRight => Vec2::new(1., 1.),
        FacingDirection::Right => Vec2::new(1., 0.),
        FacingDirection::DownRight => Vec2::new(1., -1.),
        FacingDirection::Down => Vec2::new(0., -1.),
        FacingDirection::DownLeft => Vec2::new(-1., -1.),
        FacingDirection::Left => Vec2::new(-1., 0.),
    }
}

#[derive(Copy, Clone)]
pub enum SpaceJobsEnum {
    Security,
    Control,
}

#[derive(PartialEq)]
pub enum SpaceAccessEnum {
    Security,
    Common,
}

impl Pawn {
    pub fn tab_actions_add(
        &mut self,
        tab_action_id: &str,
        entity_option: Option<Entity>,
        tab_action: TabAction,
    ) {
        let entity_tab_ids;

        match self.tab_actions_data.layout.contains_key(&entity_option) {
            true => {
                entity_tab_ids = self
                    .tab_actions_data
                    .layout
                    .get_mut(&entity_option)
                    .unwrap();
            }
            false => {
                self.tab_actions_data
                    .layout
                    .insert(entity_option, HashMap::new());
                entity_tab_ids = self
                    .tab_actions_data
                    .layout
                    .get_mut(&entity_option)
                    .unwrap();
            }
        }

        entity_tab_ids.insert(
            tab_action_id.to_string(),
            self.tab_actions_data.tab_action_i,
        );
        self.tab_actions
            .insert(self.tab_actions_data.tab_action_i, tab_action);
        self.tab_actions_data.tab_action_i += 1;
    }
    pub fn tab_actions_remove_entity(&mut self, entity_option: Option<Entity>) {
        let entity_tab_ids;

        match self.tab_actions_data.layout.get_mut(&entity_option) {
            Some(s) => {
                entity_tab_ids = s;
            }
            None => {
                return;
            }
        };

        for (_s, hashmap_index) in entity_tab_ids.iter() {
            self.tab_actions.remove(hashmap_index);
        }

        self.tab_actions_data.layout.remove(&entity_option);
    }
}

#[derive(Clone, Component)]
pub struct PersistentPlayerData {
    pub user_name_is_set: bool,
    pub character_name: String,
    pub user_name: String,
}

impl Default for PersistentPlayerData {
    fn default() -> Self {
        Self {
            user_name_is_set: false,
            character_name: "".to_string(),
            user_name: "".to_string(),
        }
    }
}

#[derive(Component)]
pub struct ControllerInput {
    pub movement_vector: Vec2,
    pub sprinting: bool,
    pub is_mouse_action_pressed: bool,
    pub targetted_limb: String,
    pub auto_move_enabled: bool,
    pub auto_move_direction: Vec2,
    pub combat_targetted_entity: Option<Entity>,
    pub combat_targetted_cell: Option<Vec3Int>,
    pub alt_attack_mode: bool,
    pub pending_direction: Option<FacingDirection>,
}

impl Default for ControllerInput {
    fn default() -> Self {
        Self {
            movement_vector: Vec2::ZERO,
            sprinting: false,
            is_mouse_action_pressed: false,
            targetted_limb: "torso".to_string(),
            auto_move_enabled: false,
            auto_move_direction: Vec2::ZERO,
            combat_targetted_entity: None,
            combat_targetted_cell: None,
            alt_attack_mode: false,
            pending_direction: None,
        }
    }
}

#[derive(Component)]
pub struct SpaceAccess {
    pub access: Vec<SpaceAccessEnum>,
}
