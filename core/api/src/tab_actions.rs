use std::{collections::HashMap, sync::Arc};

use bevy::prelude::{Component, Entity, Query, SystemLabel};

use crate::{
    data::EntityDataResource,
    data_link::DataLink,
    entity_updates::EntityData,
    gridmap::{CellData, GridMapType},
    inventory::Inventory,
    network::NetTabAction,
};

#[derive(Default)]
pub struct QueuedTabActions {
    pub queue: Vec<QueuedTabAction>,
}

pub struct QueuedTabAction {
    pub tab_id: String,
    pub handle_option: Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
    pub target_entity_option: Option<u64>,
    pub belonging_entity_option: Option<u64>,
    pub player_entity: Entity,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum TabActionsQueueLabels {
    TabAction,
}

#[derive(Default)]
pub struct TabActionsData {
    pub layout: HashMap<Option<Entity>, HashMap<String, u32>>,
    pub tab_action_i: u32,
}

#[derive(Clone)]
pub struct TabAction {
    pub id: String,
    pub text: String,
    pub tab_list_priority: u8,
    pub belonging_entity: Option<Entity>,
    pub prerequisite_check: Arc<
        dyn Fn(
                Option<Entity>,
                Option<u64>,
                Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
                f32,
                &Inventory,
                &EntityDataResource,
                &Query<&EntityData>,
                &DataLink,
            ) -> bool
            + Sync
            + Send,
    >,
}

#[derive(Component, Default)]
pub struct TabActions {
    pub tab_actions: Vec<TabAction>,
}

impl TabAction {
    pub fn into_net(
        &self,
        item_name: &str,
        entity_option: Option<u64>,
        cell_option: Option<(GridMapType, i16, i16, i16)>,
    ) -> NetTabAction {
        let self_belonging_entity;

        match self.belonging_entity {
            Some(rr) => {
                self_belonging_entity = Some(rr.to_bits());
            }
            None => {
                self_belonging_entity = None;
            }
        }

        NetTabAction {
            id: self.id.clone(),
            text: self.text.clone(),
            tab_list_priority: self.tab_list_priority,
            entity_option: entity_option,
            cell_option,
            item_name: item_name.to_string(),
            belonging_entity: self_belonging_entity,
        }
    }
}
