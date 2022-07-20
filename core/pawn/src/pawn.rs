use std::collections::HashMap;

use bevy::{
    math::Vec2,
    prelude::{Component, Entity},
};
use shared::{
    data::Vec3Int,
    get_spawn_position::FacingDirection,
    tab_actions::{TabAction, TabActionsData},
};

#[derive(PartialEq)]
pub enum ShipAuthorizationEnum {
    Security,
    Common,
}
#[derive(Copy, Clone)]
pub enum ShipJobsEnum {
    Security,
    Control,
}
#[derive(Component)]
pub struct Pawn {
    pub name: String,
    pub job: ShipJobsEnum,
    pub facing_direction: FacingDirection,
    pub tab_actions: HashMap<u32, TabAction>,
    pub tab_actions_data: TabActionsData,
}

impl Default for Pawn {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            job: ShipJobsEnum::Security,
            facing_direction: FacingDirection::Up,
            tab_actions: HashMap::new(),
            tab_actions_data: TabActionsData::default(),
        }
    }
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

#[derive(Component)]
pub struct ShipAuthorization {
    pub access: Vec<ShipAuthorizationEnum>,
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

#[derive(Default, Clone)]
pub struct UsedNames {
    pub names: HashMap<String, Entity>,
    pub user_names: HashMap<String, Entity>,
    pub player_i: u32,
    pub dummy_i: u32,
}
pub fn get_dummy_name(used_names: &mut UsedNames) -> String {
    let return_name = format!("Dummy {}", used_names.dummy_i);

    used_names.dummy_i += 1;

    return_name
}
