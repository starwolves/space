use std::collections::HashMap;

use bevy::prelude::{Component, Entity, SystemLabel};

use crate::{gridmap::GridMapType, network::NetAction};

#[derive(Default)]
pub struct QueuedActions {
    pub queue: Vec<QueuedAction>,
}

pub struct QueuedAction {
    pub id: String,
    pub handle_option: Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
    pub target_entity_option: Option<Entity>,
    pub belonging_entity_option: Option<Entity>,
    pub player_entity: Entity,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum TabActionsQueueLabels {
    TabAction,
}

#[derive(Default)]
pub struct ActionsData {
    pub layout: HashMap<Option<Entity>, HashMap<String, u32>>,
    pub action_i: u32,
}

#[derive(Clone)]
pub struct Action {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
}

#[derive(Component, Default)]
pub struct Actions {
    pub actions: Vec<Action>,
}

impl Action {
    pub fn into_net(
        &self,
        item_name: &str,
        examined_entity_option: Option<Entity>,
        examined_cell_option: Option<(GridMapType, i16, i16, i16)>,
        examiner_entity: Entity,
    ) -> NetAction {
        let mut new_entity_option = None;
        match examined_entity_option {
            Some(b) => new_entity_option = Some(b.to_bits()),
            None => {}
        }

        NetAction {
            id: self.id.clone(),
            text: self.text.clone(),
            tab_list_priority: self.tab_list_priority,
            entity_option: new_entity_option,
            cell_option: examined_cell_option,
            item_name: item_name.to_string(),
            belonging_entity: Some(examiner_entity.to_bits()),
        }
    }
}
