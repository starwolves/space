use std::{collections::HashMap, sync::Arc};

use bevy_ecs::{entity::Entity, prelude::Component, system::Query};

use crate::core::{
    data_link::components::DataLink,
    entity::{components::EntityData, resources::EntityDataResource},
    gridmap::resources::CellData,
    inventory::components::Inventory,
    networking::resources::{GridMapType, NetTabAction},
};

#[derive(Default)]
pub struct TabActionsData {
    pub layout: HashMap<Option<Entity>, HashMap<String, u32>>,
    pub tab_action_i: u32,
}

#[derive(Component)]
pub struct TabActions {
    pub tab_actions: Vec<TabAction>,
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
