pub fn tab_action(
    events: EventReader<InputTabAction>,

    criteria_query: Query<&ConnectedPlayer, Without<SoftPlayer>>,

    pawns: Query<(&Pawn, &Transform, &Inventory, &DataLink)>,
    targettable_entities: Query<(&Transform, Option<&TabActions>)>,

    gridmap_main_data: Res<GridmapMain>,
    entity_data_resource: Res<EntityDataResource>,
    entity_datas: Query<&EntityData>,
    handle_to_entity: Res<HandleToEntity>,
    mut queue: ResMut<QueuedTabActions>,
) {
    let mut input_tab_action_events = events;

    for event in input_tab_action_events.iter() {
        // Safety check.
        match criteria_query.get(event.action_performing_entity) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        let pawn_component;
        let pawn_inventory_component;
        let pawn_rigid_body_position_component;
        let data_link_component;

        match pawns.get(event.action_performing_entity) {
            Ok((c, c1, c2, c3)) => {
                pawn_component = c;
                pawn_rigid_body_position_component = c1;
                pawn_inventory_component = c2;
                data_link_component = c3;
            }
            Err(_rr) => {
                warn!("Couldn't find pawn_component.");
                continue;
            }
        }

        let distance;
        let start_pos: Vec3;
        let end_pos: Vec3 = pawn_rigid_body_position_component.translation.into();

        let mut tab_actions_component_option = None;

        match event.target_entity_option {
            Some(target_entity_bits) => {
                match targettable_entities.get(Entity::from_bits(target_entity_bits)) {
                    Ok((rigid_body_position, tab_actions_comp_option)) => {
                        start_pos = rigid_body_position.translation;
                        tab_actions_component_option = tab_actions_comp_option;
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
            None => {
                let cell_data;
                match event.target_cell_option.as_ref() {
                    Some(v) => {
                        cell_data = v;
                    }
                    None => {
                        continue;
                    }
                }
                start_pos = cell_id_to_world(Vec3Int {
                    x: cell_data.1,
                    y: cell_data.2,
                    z: cell_data.3,
                });
            }
        }

        distance = start_pos.distance(end_pos);

        let mut action_option = None;

        for (_entity_option, action_id_index_map) in pawn_component.tab_actions_data.layout.iter() {
            for (action_id, index) in action_id_index_map {
                if action_id == &event.tab_id {
                    action_option = Some(pawn_component.tab_actions.get(index).unwrap());
                    break;
                }
            }
        }

        if action_option.is_none() && &tab_actions_component_option.is_some() == &true {
            for act in &tab_actions_component_option.unwrap().tab_actions {
                if act.id == event.tab_id {
                    action_option = Some(act);
                }
            }
        }

        match action_option {
            Some(action) => {
                let self_belonging_entity;

                match event.belonging_entity_option {
                    Some(e) => {
                        self_belonging_entity = Some(Entity::from_bits(e));
                    }
                    None => {
                        self_belonging_entity = None;
                    }
                }

                let mut cell_option = None;

                match &event.target_cell_option {
                    Some(gridmap_cell_data) => {
                        let cell_item;
                        match gridmap_main_data.grid_data.get(&Vec3Int {
                            x: gridmap_cell_data.1,
                            y: gridmap_cell_data.2,
                            z: gridmap_cell_data.3,
                        }) {
                            Some(x) => {
                                cell_item = Some(x);
                            }
                            None => {
                                cell_item = None;
                            }
                        }
                        cell_option = Some((
                            gridmap_cell_data.0.clone(),
                            gridmap_cell_data.1,
                            gridmap_cell_data.2,
                            gridmap_cell_data.3,
                            cell_item,
                        ))
                    }
                    None => {}
                }

                // Safety check 2.
                match (action.prerequisite_check)(
                    self_belonging_entity,
                    event.target_entity_option,
                    cell_option,
                    distance,
                    pawn_inventory_component,
                    &entity_data_resource,
                    &entity_datas,
                    &data_link_component,
                ) {
                    true => {}
                    false => {
                        continue;
                    }
                }
            }
            None => {
                continue;
            }
        }

        let mut handle = None;

        match handle_to_entity
            .inv_map
            .get(&event.action_performing_entity)
        {
            Some(x) => {
                handle = Some(*x);
            }
            None => {}
        }

        queue.queue.push(QueuedTabAction {
            handle_option: handle,
            target_cell_option: event.target_cell_option.clone(),
            target_entity_option: event.target_entity_option,
            belonging_entity_option: event.belonging_entity_option,
            tab_id: event.tab_id.clone(),
            player_entity: event.action_performing_entity,
        });
    }
}

use std::{collections::HashMap, sync::Arc};

use bevy::{
    math::Vec3,
    prelude::{warn, Component, Entity, EventReader, Query, Res, ResMut, Transform, Without},
};

use crate::core::{
    connected_player::{
        connection::{ConnectedPlayer, SoftPlayer},
        plugin::HandleToEntity,
    },
    data_link::data_link::DataLink,
    entity::entity_data::{EntityData, EntityDataResource},
    gridmap::gridmap::{cell_id_to_world, CellData, GridmapMain, Vec3Int},
    inventory::inventory::Inventory,
    networking::networking::{GridMapType, NetTabAction},
    pawn::{can_reach_entity::REACH_DISTANCE, pawn::Pawn},
};

#[derive(Default)]
pub struct TabActionsData {
    pub layout: HashMap<Option<Entity>, HashMap<String, u32>>,
    pub tab_action_i: u32,
}

#[derive(Component, Default)]
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
pub struct InputTabAction {
    pub tab_id: String,
    pub action_performing_entity: Entity,
    pub target_entity_option: Option<u64>,
    pub belonging_entity_option: Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
}

pub fn get_tab_action(id: &str) -> Option<TabAction> {
    let result;

    if id == "actions::pawn/examine" {
        result = Some(TabAction {
            id: id.to_string(),
            text: "Examine".to_string(),
            tab_list_priority: u8::MAX,
            prerequisite_check: Arc::new(examine_tab_prerequisite_check),
            belonging_entity: None,
        });
    } else if id == "actions::inventory/pickup" {
        result = Some(TabAction {
            id: id.to_string(),
            text: "Pickup".to_string(),
            tab_list_priority: u8::MAX - 1,
            prerequisite_check: Arc::new(pickup_tab_prerequisite_check),
            belonging_entity: None,
        });
    } else {
        result = None;
    }

    result
}

pub fn examine_tab_prerequisite_check(
    _self_tab_entity: Option<Entity>,
    entity_id_bits_option: Option<u64>,
    cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    _distance: f32,
    _inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    cell_id_option.is_some() || entity_id_bits_option.is_some()
}

pub fn pickup_tab_prerequisite_check(
    _self_tab_entity: Option<Entity>,
    entity_id_bits_option: Option<u64>,
    _cell_id_option: Option<(GridMapType, i16, i16, i16, Option<&CellData>)>,
    distance: f32,
    inventory_component: &Inventory,
    _entity_data_resource: &EntityDataResource,
    _entity_datas: &Query<&EntityData>,
    _data_link_component: &DataLink,
) -> bool {
    distance < REACH_DISTANCE
        && entity_id_bits_option.is_some()
        && inventory_component.get_active_slot_entity().is_none()
}

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
