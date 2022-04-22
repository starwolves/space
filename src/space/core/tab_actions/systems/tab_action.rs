use bevy_app::{EventReader, EventWriter};
use bevy_ecs::{
    entity::Entity,
    prelude::Without,
    system::{Query, Res},
};
use bevy_log::warn;
use bevy_math::Vec3;
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space::{
    core::{
        connected_player::{
            components::{ConnectedPlayer, SoftPlayer},
            events::{InputExamineEntity, InputExamineMap},
            resources::HandleToEntity,
        },
        data_link::components::DataLink,
        entity::{components::EntityData, resources::EntityDataResource},
        gridmap::{
            functions::gridmap_functions::cell_id_to_world,
            resources::{GridmapMain, Vec3Int},
        },
        inventory::{components::Inventory, events::InputUseWorldItem},
        pawn::components::Pawn,
        static_body::components::StaticTransform,
        tab_actions::{components::TabActions, events::InputTabAction},
    },
    entities::{
        air_locks::events::{AirLockLockClosed, AirLockLockOpen, InputAirLockToggleOpen},
        construction_tool_admin::events::{
            InputConstruct, InputConstructionOptions, InputDeconstruct,
        },
        counter_windows::events::{
            CounterWindowLockClosed, CounterWindowLockOpen, InputCounterWindowToggleOpen,
        },
    },
};

pub fn tab_action(
    events: (
        EventReader<InputTabAction>,
        EventWriter<InputExamineEntity>,
        EventWriter<InputExamineMap>,
        EventWriter<InputConstruct>,
        EventWriter<InputDeconstruct>,
        EventWriter<InputUseWorldItem>,
        EventWriter<AirLockLockOpen>,
        EventWriter<AirLockLockClosed>,
    ),
    mut event_construction_options: EventWriter<InputConstructionOptions>,
    mut air_lock_toggle_open_event: EventWriter<InputAirLockToggleOpen>,
    mut counter_window_toggle_open_event: EventWriter<InputCounterWindowToggleOpen>,
    mut counter_window_lock_open_event: EventWriter<CounterWindowLockOpen>,
    mut counter_window_lock_closed_event: EventWriter<CounterWindowLockClosed>,

    criteria_query: Query<&ConnectedPlayer, Without<SoftPlayer>>,

    pawns: Query<(&Pawn, &RigidBodyPositionComponent, &Inventory, &DataLink)>,
    targettable_entities: Query<(
        Option<&RigidBodyPositionComponent>,
        Option<&StaticTransform>,
        Option<&TabActions>,
    )>,

    gridmap_main_data: Res<GridmapMain>,
    entity_data_resource: Res<EntityDataResource>,
    entity_datas: Query<&EntityData>,
    handle_to_entity: Res<HandleToEntity>,
) {
    let (
        mut input_tab_action_events,
        mut event_examine_entity,
        mut event_examine_map,
        mut event_construct,
        mut event_deconstruct,
        mut pickup_world_item_event,
        mut air_lock_lock_open_event,
        mut air_lock_lock_closed_event,
    ) = events;

    for event in input_tab_action_events.iter() {
        // Safety check.
        match criteria_query.get(event.player_entity) {
            Ok(_) => {}
            Err(_rr) => {
                continue;
            }
        }

        let pawn_component;
        let pawn_inventory_component;
        let pawn_rigid_body_position_component;
        let data_link_component;

        match pawns.get(event.player_entity) {
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
        let end_pos: Vec3 = pawn_rigid_body_position_component
            .0
            .position
            .translation
            .into();

        let mut tab_actions_component_option = None;

        match event.target_entity_option {
            Some(target_entity_bits) => {
                match targettable_entities.get(Entity::from_bits(target_entity_bits)) {
                    Ok((
                        rigid_body_position_comp_option,
                        static_transform_comp_option,
                        tab_actions_comp_option,
                    )) => {
                        match static_transform_comp_option {
                            Some(static_transform_component) => {
                                start_pos = static_transform_component.transform.translation;
                            }
                            None => {
                                start_pos = rigid_body_position_comp_option
                                    .unwrap()
                                    .0
                                    .position
                                    .translation
                                    .into();
                            }
                        }

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

                match event.belonging_entity {
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

        let handle;

        match handle_to_entity.inv_map.get(&event.player_entity) {
            Some(x) => {
                handle = x;
            }
            None => {
                warn!("Couldn't find handle.");
                continue;
            }
        }

        if event.tab_id == "examine" {
            match event.target_entity_option {
                Some(entity_bits) => {
                    event_examine_entity.send(InputExamineEntity {
                        handle: *handle,
                        examine_entity_bits: entity_bits,
                        entity: event.player_entity,
                    });
                }
                None => match &event.target_cell_option {
                    Some((gridmap_type, idx, idy, idz)) => {
                        event_examine_map.send(InputExamineMap {
                            handle: *handle,
                            entity: event.player_entity,
                            gridmap_type: gridmap_type.clone(),
                            gridmap_cell_id: Vec3Int {
                                x: *idx,
                                y: *idy,
                                z: *idz,
                            },
                        });
                    }
                    None => {}
                },
            }
        } else if event.tab_id == "construct" {
            if event.target_cell_option.is_some() && event.belonging_entity.is_some() {
                event_construct.send(InputConstruct {
                    handle: *handle,
                    target_cell: event.target_cell_option.as_ref().unwrap().clone(),
                    belonging_entity: event.belonging_entity.unwrap(),
                });
            }
        } else if event.tab_id == "deconstruct" {
            if (event.target_entity_option.is_some() || event.target_cell_option.is_some())
                && event.belonging_entity.is_some()
            {
                event_deconstruct.send(InputDeconstruct {
                    handle: *handle,
                    target_cell_option: event.target_cell_option.clone(),
                    target_entity_option: event.target_entity_option,
                    belonging_entity: event.belonging_entity.unwrap(),
                });
            }
        } else if event.tab_id == "constructionoptions" {
            if event.belonging_entity.is_some() {
                event_construction_options.send(InputConstructionOptions {
                    handle: *handle,
                    belonging_entity: event.belonging_entity.unwrap(),
                });
            }
        } else if event.tab_id == "pickup" {
            if event.target_entity_option.is_some() {
                pickup_world_item_event.send(InputUseWorldItem {
                    pickuper_entity: event.player_entity,
                    pickupable_entity_bits: event.target_entity_option.unwrap(),
                });
            }
        } else if event.tab_id == "airlocktoggleopen" {
            if event.target_entity_option.is_some() {
                air_lock_toggle_open_event.send(InputAirLockToggleOpen {
                    opener: event.player_entity,
                    opened: event.target_entity_option.unwrap(),
                });
            }
        } else if event.tab_id == "counterwindowtoggleopen" {
            if event.target_entity_option.is_some() {
                counter_window_toggle_open_event.send(InputCounterWindowToggleOpen {
                    opener: event.player_entity,
                    opened: event.target_entity_option.unwrap(),
                });
            }
        } else if event.tab_id == "counterwindowlockopen" {
            if event.target_entity_option.is_some() {
                counter_window_lock_open_event.send(CounterWindowLockOpen {
                    locked: Entity::from_bits(event.target_entity_option.unwrap()),
                    locker: event.player_entity,
                });
            }
        } else if event.tab_id == "counterwindowlockclosed" {
            if event.target_entity_option.is_some() {
                counter_window_lock_closed_event.send(CounterWindowLockClosed {
                    locked: Entity::from_bits(event.target_entity_option.unwrap()),
                    locker: event.player_entity,
                });
            }
        } else if event.tab_id == "airlocklockopen" {
            if event.target_entity_option.is_some() {
                air_lock_lock_open_event.send(AirLockLockOpen {
                    locked: Entity::from_bits(event.target_entity_option.unwrap()),
                    locker: event.player_entity,
                });
            }
        } else if event.tab_id == "airlocklockclosed" {
            if event.target_entity_option.is_some() {
                air_lock_lock_closed_event.send(AirLockLockClosed {
                    locked: Entity::from_bits(event.target_entity_option.unwrap()),
                    locker: event.player_entity,
                });
            }
        }
    }
}
