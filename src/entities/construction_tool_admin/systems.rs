use std::collections::HashMap;

use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    prelude::Without,
    system::{Commands, Query, Res, ResMut},
};
use bevy_log::{info, warn};
use bevy_math::Quat;
use bevy_transform::components::Transform;
use doryen_fov::FovAlgorithm;
use rand::Rng;

use crate::{
    core::{
        atmospherics::{
            functions::get_atmos_index,
            resources::{AtmosphericsResource, EffectType},
            systems::rigidbody_forces_atmospherics::AdjacentTileDirection,
        },
        chat::functions::FURTHER_ITALIC_FONT,
        connected_player::{components::ConnectedPlayer, resources::HandleToEntity},
        entity::{
            components::EntityData,
            resources::{EntityDataResource, SpawnData},
            spawn::DefaultSpawnEvent,
        },
        gridmap::{
            events::RemoveCell,
            functions::{
                build_gridmap_from_data::spawn_main_cell,
                gridmap_functions::{cell_id_to_world, world_to_cell_id},
            },
            resources::{
                to_doryen_coordinates, CellData, CellUpdate, DoryenMap, EntityGridData,
                GridmapData, GridmapDetails1, GridmapMain, StructureHealth, Vec2Int, Vec3Int,
            },
            systems::senser_update_fov::FOV_DISTANCE,
        },
        inventory_item::components::InventoryItem,
        networking::resources::{GridMapType, ReliableServerMessage, TextTreeBit},
        pawn::components::Pawn,
        rigid_body::components::RigidBodyDisabled,
        sensable::components::Sensable,
        senser::components::Senser,
        sfx::{
            components::sfx_auto_destroy, functions::sfx_builder, resources::SfxAutoDestroyTimers,
        },
    },
    entities::{
        construction_tool_admin::{
            components::ConstructionTool,
            events::{
                InputConstruct, InputConstructionOptions, InputConstructionOptionsSelection,
                InputDeconstruct, NetConstructionTool,
            },
        },
        sfx::{
            construction::{
                construct1_sfx::Construct1SfxBundle, construct2_sfx::Construct2SfxBundle,
                construct_light1_sfx::ConstructLight1SfxBundle,
                construct_light2_sfx::ConstructLight2SfxBundle,
                deconstruct1_sfx::Deconstruct1SfxBundle,
            },
            ui::{
                ui_interaction1_sfx::UIInteraction1SfxBundle,
                ui_interaction2_sfx::UIInteraction2SfxBundle,
                ui_interaction3_sfx::UIInteraction3SfxBundle,
            },
        },
    },
};

pub fn construction_tool(
    event_readers: (
        EventReader<InputConstruct>,
        EventReader<InputDeconstruct>,
        EventReader<InputConstructionOptions>,
        EventReader<InputConstructionOptionsSelection>,
        EventWriter<RemoveCell>,
    ),
    entity_data: Res<EntityDataResource>,
    gridmap_data: Res<GridmapData>,
    mut gridmap_main: ResMut<GridmapMain>,
    gridmap_details1: Res<GridmapDetails1>,
    handle_to_entity: Res<HandleToEntity>,
    mut net_construction_tool: EventWriter<NetConstructionTool>,
    mut construction_tools: Query<(
        Entity,
        &mut ConstructionTool,
        &Sensable,
        &InventoryItem,
        &Transform,
    )>,
    pawns: Query<&Pawn>,
    mut commands: Commands,
    mut sfx_auto_destroy_timers: ResMut<SfxAutoDestroyTimers>,
    mut fov_map: ResMut<DoryenMap>,
    mut sensers: Query<(&mut Senser, &ConnectedPlayer)>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
    rigid_bodies: Query<(&Transform, &EntityData), Without<RigidBodyDisabled>>,
    mut default_spawner: EventWriter<DefaultSpawnEvent>,
) {
    let (
        mut input_construct_event,
        mut input_deconstruct_event,
        mut input_construction_options_event,
        mut input_construction_options_selection_event,
        mut remove_cell_events,
    ) = event_readers;

    // Retreive all construction and complex constructions as a text list and make generic client GUI text list call.
    for event in input_construction_options_event.iter() {
        let mut text_options = vec![];

        let entity = Entity::from_bits(event.belonging_entity);

        for entity_data_properties in entity_data.data.iter() {
            match &entity_data_properties.grid_item {
                Some(_d) => {
                    text_options.push(entity_data_properties.name.clone());
                }
                None => {}
            }
        }

        for (i, gridmap_data_properties) in gridmap_data.main_cell_properties.iter() {
            if gridmap_data_properties.constructable {
                text_options.push(gridmap_data.main_id_name_map.get(i).unwrap().to_string());
            }
        }

        text_options.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        let mut text_tree_selection_map = HashMap::new();

        text_tree_selection_map.insert("main".to_string(), TextTreeBit::Final(text_options));

        let mut pawn_name = "";

        let inventory_item_component = construction_tools
            .get_component::<InventoryItem>(entity)
            .unwrap();

        match inventory_item_component.in_inventory_of_entity {
            Some(owner_entity) => match pawns.get(owner_entity) {
                Ok(pawn_component) => {
                    pawn_name = &pawn_component.name;
                }
                Err(_) => {
                    warn!("This construction tool's owner isnt a pawn!");
                }
            },
            None => {
                warn!("This construction tool has no owner!");
            }
        }

        let public_notification = "[font=".to_owned()
            + FURTHER_ITALIC_FONT
            + "]"
            + pawn_name
            + " navigates the interface of the construction tool.[/font]";

        let sensable_component = construction_tools
            .get_component::<Sensable>(entity)
            .unwrap();

        match event.handle_option {
            Some(t) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(sensed_by_entity) {
                        Some(senser_handle) => {
                            if senser_handle != &t {
                                net_construction_tool.send(NetConstructionTool {
                                    handle: *senser_handle,
                                    message: ReliableServerMessage::ChatMessage(
                                        public_notification.clone(),
                                    ),
                                });
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        // Make a generic GUI netcode call now.
        match event.handle_option {
            Some(t) => {
                net_construction_tool.send(NetConstructionTool {
                    handle: t,
                    message: ReliableServerMessage::TextTreeSelection(
                        Some(event.belonging_entity),
                        "action::construction_tool_admin/constructionoptions".to_string(),
                        "textselection::construction_tool_admin/constructionoptionslist"
                            .to_string(),
                        "Construction Options".to_string(),
                        text_tree_selection_map,
                    ),
                });
            }
            None => {}
        }
    }

    for event in input_construction_options_selection_event.iter() {
        let mut text_options = vec![];

        for entity_data_properties in entity_data.data.iter() {
            match &entity_data_properties.grid_item {
                Some(_d) => {
                    text_options.push(entity_data_properties.name.clone());
                }
                None => {}
            }
        }

        for (i, gridmap_data_properties) in gridmap_data.main_cell_properties.iter() {
            if gridmap_data_properties.constructable {
                text_options.push(gridmap_data.main_id_name_map.get(i).unwrap().to_string());
            }
        }

        let mut construction_tool_component = construction_tools
            .get_component_mut::<ConstructionTool>(event.entity)
            .unwrap();

        if text_options.contains(&event.menu_selection) {
            construction_tool_component.construction_option = Some(event.menu_selection.clone());
        }

        let personal_update_text = "[font=".to_owned()
            + FURTHER_ITALIC_FONT
            + "]"
            + "Cycled construction option "
            + &event.menu_selection
            + ".[/font]";

        match event.handle_option {
            Some(t) => {
                net_construction_tool.send(NetConstructionTool {
                    handle: t,
                    message: ReliableServerMessage::ChatMessage(personal_update_text),
                });
            }
            None => {}
        }

        let mut pawn_name = "";

        let (_s, _y, sensable_component, inventory_item_component, rgpc) =
            construction_tools.get(event.entity).unwrap();

        match inventory_item_component.in_inventory_of_entity {
            Some(owner_entity) => match pawns.get(owner_entity) {
                Ok(pawn_component) => {
                    pawn_name = &pawn_component.name;
                }
                Err(_) => {
                    warn!("This construction tool's owner isnt a pawn!");
                }
            },
            None => {
                warn!("This construction tool has no owner!");
            }
        }

        let public_notification = "[font=".to_owned()
            + FURTHER_ITALIC_FONT
            + "]"
            + pawn_name
            + " cycles the options of the construction tool.[/font]";

        match event.handle_option {
            Some(t) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(sensed_by_entity) {
                        Some(senser_handle) => {
                            if senser_handle != &t {
                                net_construction_tool.send(NetConstructionTool {
                                    handle: *senser_handle,
                                    message: ReliableServerMessage::ChatMessage(
                                        public_notification.clone(),
                                    ),
                                });
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        let mut rng = rand::thread_rng();
        let random_pick: i32 = rng.gen_range(0..3);

        let sfx_entity;

        if random_pick == 0 {
            sfx_entity = sfx_builder(&mut commands, *rgpc, Box::new(UIInteraction1SfxBundle::new));
        } else if random_pick == 1 {
            sfx_entity = sfx_builder(&mut commands, *rgpc, Box::new(UIInteraction2SfxBundle::new));
        } else {
            sfx_entity = sfx_builder(&mut commands, *rgpc, Box::new(UIInteraction3SfxBundle::new));
        }

        sfx_auto_destroy(sfx_entity, &mut sfx_auto_destroy_timers);
    }

    // Write to a DespawnShipCell event.
    for event in input_deconstruct_event.iter() {
        let belonging_entity = Entity::from_bits(event.belonging_entity);

        let (
            _construction_tool_entity,
            _construction_tool_component,
            sensable_component,
            inventory_item_component,
            rigid_body_position_component,
        );

        match construction_tools.get(belonging_entity) {
            Ok((
                construction_tool_entity_passed,
                construction_tool_component_passed,
                sensable_component_passed,
                inventory_item_component_passed,
                rigid_body_position_component_passed,
            )) => {
                _construction_tool_entity = construction_tool_entity_passed;
                _construction_tool_component = construction_tool_component_passed;
                sensable_component = sensable_component_passed;
                inventory_item_component = inventory_item_component_passed;
                rigid_body_position_component = rigid_body_position_component_passed;
            }
            Err(_rr) => {
                warn!("Couldn't find belonging entity construction tool.");
                continue;
            }
        }

        let sfx_entity;
        let deconstructed_item_name;

        match &event.target_cell_option {
            Some((gridmap_type, cell_x, cell_y, cell_z)) => {
                let cell_data;

                let text_names_map;

                let cell_id_int = Vec3Int {
                    x: *cell_x,
                    y: *cell_y,
                    z: *cell_z,
                };

                match gridmap_type {
                    GridMapType::Main => {
                        match gridmap_main.grid_data.get(&cell_id_int) {
                            Some(cell_data_passed) => {
                                cell_data = cell_data_passed;
                                text_names_map = &gridmap_data.main_text_names;
                            }
                            None => {
                                warn!("Couldnt find gridmap_main.data for cellid.");
                                continue;
                            }
                        }

                        sfx_entity = sfx_builder(
                            &mut commands,
                            *rigid_body_position_component,
                            Box::new(Deconstruct1SfxBundle::new),
                        );
                    }
                    GridMapType::Details1 => {
                        match gridmap_details1.data.get(&cell_id_int) {
                            Some(cell_data_passed) => {
                                cell_data = cell_data_passed;
                                text_names_map = &gridmap_data.details1_text_names;
                            }
                            None => {
                                warn!("Couldnt find gridmap_details1.data for cellid.");
                                continue;
                            }
                        }

                        let mut rng = rand::thread_rng();
                        let random_pick: i32 = rng.gen_range(0..2);

                        if random_pick == 0 {
                            sfx_entity = sfx_builder(
                                &mut commands,
                                *rigid_body_position_component,
                                Box::new(ConstructLight1SfxBundle::new),
                            );
                        } else {
                            sfx_entity = sfx_builder(
                                &mut commands,
                                *rigid_body_position_component,
                                Box::new(ConstructLight2SfxBundle::new),
                            );
                        }
                    }
                }

                let mut cell_data_clone = (*cell_data).clone();
                cell_data_clone.item = -1;

                deconstructed_item_name = text_names_map.get(&cell_data.item).unwrap().get_name();

                gridmap_main.updates.insert(
                    cell_id_int,
                    CellUpdate {
                        entities_received: vec![],
                        cell_data: cell_data_clone.clone(),
                    },
                );

                remove_cell_events.send(RemoveCell {
                    gridmap_type: gridmap_type.clone(),
                    id: Vec3Int {
                        x: *cell_x,
                        y: *cell_y,
                        z: *cell_z,
                    },
                    handle_option: event.handle_option,
                    cell_data: cell_data_clone,
                });
            }
            None => {
                let deconstruct_entity = Entity::from_bits(event.target_entity_option.unwrap());

                let g = rigid_bodies.get(deconstruct_entity).unwrap();

                let entity_position_component = g.0;
                let entity_data = g.1;

                deconstructed_item_name = &entity_data.entity_name;

                let cell_id = world_to_cell_id(entity_position_component.translation.into());

                gridmap_main.entity_data.remove(&cell_id);

                commands.entity(deconstruct_entity).despawn();

                sfx_entity = sfx_builder(
                    &mut commands,
                    *rigid_body_position_component,
                    Box::new(Deconstruct1SfxBundle::new),
                );
            }
        }

        sfx_auto_destroy(sfx_entity, &mut sfx_auto_destroy_timers);

        let personal_update_text = "[font=".to_owned()
            + FURTHER_ITALIC_FONT
            + "]"
            + "You've deconstructed the "
            + &deconstructed_item_name
            + ".[/font]";
        match event.handle_option {
            Some(t) => {
                net_construction_tool.send(NetConstructionTool {
                    handle: t,
                    message: ReliableServerMessage::ChatMessage(personal_update_text),
                });
            }
            None => {}
        }

        let pawn_name;

        match inventory_item_component.in_inventory_of_entity {
            Some(owner_entity) => match pawns.get(owner_entity) {
                Ok(pawn_component) => {
                    pawn_name = &pawn_component.name;
                }
                Err(_) => {
                    warn!("This construction tool's owner isnt a pawn!");
                    continue;
                }
            },
            None => {
                warn!("This construction tool has no owner!");
                continue;
            }
        }

        let public_notification = "[font=".to_owned()
            + FURTHER_ITALIC_FONT
            + "]"
            + pawn_name
            + " has deconstructed "
            + &deconstructed_item_name
            + ".[/font]";
        match event.handle_option {
            Some(t) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(sensed_by_entity) {
                        Some(senser_handle) => {
                            if senser_handle != &t {
                                net_construction_tool.send(NetConstructionTool {
                                    handle: *senser_handle,
                                    message: ReliableServerMessage::ChatMessage(
                                        public_notification.clone(),
                                    ),
                                });
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }

    // Write to a SpawnShipCell event.
    for event in input_construct_event.iter() {
        let entity = Entity::from_bits(event.belonging_entity);
        let construction_tool_components;

        match construction_tools.get(entity) {
            Ok(s) => {
                construction_tool_components = s;
            }
            Err(_rr) => {
                warn!("Couldn't find construction tool!");
                continue;
            }
        }

        let (
            _construction_tool_entity,
            construction_tool_component,
            sensable_component,
            inventory_item_component,
            rigid_body_position_component,
        ) = construction_tool_components;

        let construction_selection;

        match &construction_tool_component.construction_option {
            Some(s) => {
                construction_selection = s;
            }
            None => {
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "Please select a construction option first.[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_construction_tool.send(NetConstructionTool {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                continue;
            }
        }

        let construction_is_entity;

        let mut allowed_grid_items = vec![];
        let construction_entity_name = construction_tool_component
            .construction_option
            .as_ref()
            .unwrap();

        match gridmap_data.main_name_id_map.get(construction_selection) {
            Some(_) => {
                construction_is_entity = false;
            }
            None => {
                // Get entity data.
                let built_entity_data;

                match entity_data.name_to_id.get(construction_entity_name) {
                    Some(i) => {
                        built_entity_data = entity_data.data.get(*i).unwrap();
                    }
                    None => {
                        continue;
                    }
                };

                match &built_entity_data.grid_item {
                    Some(d) => {
                        allowed_grid_items = d.can_be_built_with_grid_item.clone();
                    }
                    None => {
                        continue;
                    }
                }
                construction_is_entity = true;
            }
        }

        let input_cell = Vec3Int {
            x: event.target_cell.1,
            y: event.target_cell.2,
            z: event.target_cell.3,
        };

        let mut target_cell_id = input_cell.clone();

        if !construction_is_entity {
            match gridmap_main.grid_data.get(&input_cell) {
                Some(_input_cell_data) => {
                    target_cell_id.y = 0;
                }
                None => {
                    target_cell_id.y = -1;
                }
            }
        } else {
            target_cell_id.y = 0;
        }

        match gridmap_details1.data.get(&target_cell_id) {
            Some(_input_cell_data) => {
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "Construction blocked.[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_construction_tool.send(NetConstructionTool {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                continue;
            }
            None => {}
        }

        match gridmap_main.grid_data.get(&target_cell_id) {
            Some(target_cell_data) => {
                let name = gridmap_data
                    .main_id_name_map
                    .get(&target_cell_data.item)
                    .unwrap();

                if (construction_is_entity && allowed_grid_items.contains(name)) == false {
                    let personal_update_text = "[font=".to_owned()
                        + FURTHER_ITALIC_FONT
                        + "]"
                        + "Construction blocked.[/font]";
                    match event.handle_option {
                        Some(t) => {
                            net_construction_tool.send(NetConstructionTool {
                                handle: t,
                                message: ReliableServerMessage::ChatMessage(personal_update_text),
                            });
                        }
                        None => {}
                    }
                    continue;
                }
            }
            None => {}
        }

        match gridmap_main.entity_data.get(&target_cell_id) {
            Some(ed) => {
                let entity_data = entity_data
                    .data
                    .get(*entity_data.name_to_id.get(&ed.entity_name).unwrap())
                    .unwrap();

                let mut is_blocking = true;

                match &entity_data.grid_item {
                    Some(data) => {
                        if data
                            .can_be_built_with_grid_item
                            .contains(construction_entity_name)
                        {
                            is_blocking = false;
                        }
                    }
                    None => {}
                }
                if is_blocking {
                    let personal_update_text = "[font=".to_owned()
                        + FURTHER_ITALIC_FONT
                        + "]"
                        + "Construction blocked.[/font]";
                    match event.handle_option {
                        Some(t) => {
                            net_construction_tool.send(NetConstructionTool {
                                handle: t,
                                message: ReliableServerMessage::ChatMessage(personal_update_text),
                            });
                        }
                        None => {}
                    }
                    continue;
                }
            }
            None => {}
        }

        let mut pawn_name = "";

        match inventory_item_component.in_inventory_of_entity {
            Some(owner_entity) => match pawns.get(owner_entity) {
                Ok(pawn_component) => {
                    pawn_name = &pawn_component.name;
                }
                Err(_) => {
                    warn!("This construction tool's owner isnt a pawn!");
                }
            },
            None => {
                warn!("This construction tool has no owner!");
            }
        }

        let target_cell_id_2 = Vec2Int {
            x: target_cell_id.x,
            y: target_cell_id.z,
        };

        let mut blockers = vec![];

        for (rigid_body, entity_data_component) in rigid_bodies.iter() {
            let pos = rigid_body.translation;

            let cell_id = world_to_cell_id(pos);
            let cell_id_2 = Vec2Int {
                x: cell_id.x,
                y: cell_id.z,
            };

            if cell_id_2 == target_cell_id_2 {
                let name = entity_data_component.entity_name.clone();
                blockers.push(name);
            }
        }

        let mut blockerz = None;

        for blocker in blockers {
            match entity_data.name_to_id.get(&blocker) {
                Some(id) => {
                    let entity_data_properties = entity_data.data.get(*id).unwrap();
                    match entity_data_properties.grid_item {
                        Some(_) => {
                            // Already in GridMapMain.entity_data
                        }
                        None => blockerz = Some(blocker),
                    }
                }
                None => {}
            }
        }

        match blockerz {
            Some(blocker_name) => {
                let personal_update_text = "[font=".to_owned()
                    + FURTHER_ITALIC_FONT
                    + "]"
                    + "Construction blocked by "
                    + &blocker_name
                    + ".[/font]";
                match event.handle_option {
                    Some(t) => {
                        net_construction_tool.send(NetConstructionTool {
                            handle: t,
                            message: ReliableServerMessage::ChatMessage(personal_update_text),
                        });
                    }
                    None => {}
                }
                continue;
            }
            None => {}
        }

        let new_entity;
        let coords = to_doryen_coordinates(target_cell_id.x, target_cell_id.z);

        match gridmap_data.main_name_id_map.get(construction_selection) {
            Some(target_item_id) => {
                // Construction item is map cell.

                let cell_properties = gridmap_data
                    .main_cell_properties
                    .get(&target_item_id)
                    .unwrap();

                let mut new_cell_orientation: i64 = *cell_properties
                    .direction_rotations
                    .data
                    .get(&AdjacentTileDirection::Right)
                    .unwrap() as i64;

                for j in 0..4 {
                    let mut adjacent_cell_id = target_cell_id.clone();
                    let tile_direction;

                    if j == 0 {
                        adjacent_cell_id.x += 1;
                        tile_direction = AdjacentTileDirection::Right;
                    } else if j == 1 {
                        adjacent_cell_id.x -= 1;
                        tile_direction = AdjacentTileDirection::Left;
                    } else if j == 2 {
                        adjacent_cell_id.z += 1;
                        tile_direction = AdjacentTileDirection::Up;
                    } else {
                        adjacent_cell_id.z -= 1;
                        tile_direction = AdjacentTileDirection::Down;
                    }

                    match gridmap_main.grid_data.get(&adjacent_cell_id) {
                        Some(_data) => {
                            // Do more checks here in future.
                        }
                        None => {
                            continue;
                        }
                    }

                    new_cell_orientation = *cell_properties
                        .direction_rotations
                        .data
                        .get(&tile_direction)
                        .unwrap() as i64;

                    break;
                }

                // Spawn cell, check build_gridmap_from_data for more info.
                if target_cell_id.y == 0 {
                    if cell_properties.floor_cell {
                        let personal_update_text = "[font=".to_owned()
                            + FURTHER_ITALIC_FONT
                            + "]"
                            + "Please construct a wall and not a floor here![/font]";
                        match event.handle_option {
                            Some(t) => {
                                net_construction_tool.send(NetConstructionTool {
                                    handle: t,
                                    message: ReliableServerMessage::ChatMessage(
                                        personal_update_text,
                                    ),
                                });
                            }
                            None => {}
                        }
                        continue;
                    }

                    let entity_op = spawn_main_cell(
                        &mut commands,
                        target_cell_id,
                        *target_item_id,
                        new_cell_orientation,
                        &gridmap_data,
                    );

                    if !gridmap_data
                        .non_fov_blocking_cells_list
                        .contains(target_item_id)
                    {
                        fov_map.map.set_transparent(coords.0, coords.1, false);
                    }

                    new_entity = Some(entity_op);
                } else {
                    if !cell_properties.floor_cell {
                        let personal_update_text = "[font=".to_owned()
                            + FURTHER_ITALIC_FONT
                            + "]"
                            + "Please construct a floor and not a wall here![/font]";
                        match event.handle_option {
                            Some(t) => {
                                net_construction_tool.send(NetConstructionTool {
                                    handle: t,
                                    message: ReliableServerMessage::ChatMessage(
                                        personal_update_text,
                                    ),
                                });
                            }
                            None => {}
                        }
                        continue;
                    }
                    new_entity = None;
                }

                let cell_data = CellData {
                    item: *target_item_id,
                    orientation: new_cell_orientation,
                    health: StructureHealth::default(),
                    entity: new_entity,
                };

                gridmap_main
                    .grid_data
                    .insert(target_cell_id, cell_data.clone());

                // Update atmospherics.

                let mut atmospherics = atmospherics_resource
                    .atmospherics
                    .get_mut(get_atmos_index(Vec2Int {
                        x: target_cell_id.x,
                        y: target_cell_id.z,
                    }))
                    .unwrap();

                if target_cell_id.y == 0 {
                    let properties = gridmap_data
                        .main_cell_properties
                        .get(&cell_data.item)
                        .unwrap();
                    atmospherics.blocked = properties.atmospherics_blocker;
                    atmospherics.forces_push_up = properties.atmospherics_pushes_up;
                } else {
                    // Remove vacuum flag from atmos.
                    atmospherics.effects.remove(&EffectType::Floorless);
                }

                gridmap_main.updates.insert(
                    target_cell_id,
                    CellUpdate {
                        entities_received: vec![],
                        cell_data: cell_data,
                    },
                );
            }
            None => {
                // Construction item is entity.
                info!("Building an entity.");

                // Build transform to be passed to the entity's spawn func.
                let world_position = cell_id_to_world(target_cell_id);

                let mut spawn_rotation_option: Option<Quat> = None;
                // Decide the rotation..

                for j in 0..4 {
                    let mut adjacent_cell_id = target_cell_id.clone();
                    let tile_direction;

                    if j == 0 {
                        adjacent_cell_id.x += 1;
                        tile_direction = AdjacentTileDirection::Right;
                    } else if j == 1 {
                        adjacent_cell_id.x -= 1;
                        tile_direction = AdjacentTileDirection::Left;
                    } else if j == 2 {
                        adjacent_cell_id.z += 1;
                        tile_direction = AdjacentTileDirection::Up;
                    } else {
                        adjacent_cell_id.z -= 1;
                        tile_direction = AdjacentTileDirection::Down;
                    }

                    match gridmap_main.grid_data.get(&adjacent_cell_id) {
                        Some(_data) => {
                            // Do more checks here in future.
                        }
                        None => {
                            continue;
                        }
                    }

                    match tile_direction {
                        AdjacentTileDirection::Left => {
                            spawn_rotation_option = Some(Quat::from_xyzw(0., 0., 0., 1.));
                        }
                        AdjacentTileDirection::Right => {
                            spawn_rotation_option = Some(Quat::from_xyzw(0., 0., 0., 1.));
                        }
                        AdjacentTileDirection::Up => {
                            spawn_rotation_option = Some(Quat::from_xyzw(0., 0.707, 0., 0.707));
                        }
                        AdjacentTileDirection::Down => {
                            spawn_rotation_option = Some(Quat::from_xyzw(0., 0.707, 0., 0.707));
                        }
                    }

                    break;
                }

                let spawn_rotation;

                match spawn_rotation_option {
                    Some(s) => {
                        spawn_rotation = s;
                    }
                    None => {
                        spawn_rotation = Quat::from_xyzw(0., 0., 0., 1.);
                    }
                }

                let mut corrected_world_position = world_position.clone();
                corrected_world_position.y = 0.;

                let built_entity_data;

                match entity_data.name_to_id.get(construction_entity_name) {
                    Some(i) => {
                        built_entity_data = entity_data.data.get(*i).unwrap();
                    }
                    None => {
                        continue;
                    }
                };

                let constructable_data;

                match &built_entity_data.grid_item {
                    Some(d) => {
                        constructable_data = d;
                    }
                    None => {
                        continue;
                    }
                }

                let mut spawn_transform = Transform::identity();
                spawn_transform.translation =
                    corrected_world_position + constructable_data.transform_offset.translation;
                spawn_transform.rotation = constructable_data
                    .transform_offset
                    .rotation
                    .mul_quat(spawn_rotation);

                let new_entity = commands.spawn().id();

                default_spawner.send(DefaultSpawnEvent {
                    spawn_data: SpawnData {
                        entity_transform: spawn_transform,
                        correct_transform: true,
                        pawn_data_option: None,
                        held_data_option: None,
                        default_map_spawn: false,
                        properties: HashMap::new(),
                        showcase_data_option: None,
                        entity_name: construction_entity_name.to_string(),
                        entity: new_entity,
                    },
                });

                gridmap_main.entity_data.insert(
                    target_cell_id,
                    EntityGridData {
                        entity: new_entity,
                        entity_name: construction_entity_name.to_string(),
                    },
                );
            }
        }

        let personal_update_text = "[font=".to_owned()
            + FURTHER_ITALIC_FONT
            + "]"
            + "You've constructed a "
            + construction_selection
            + "![/font]";
        match event.handle_option {
            Some(t) => {
                net_construction_tool.send(NetConstructionTool {
                    handle: t,
                    message: ReliableServerMessage::ChatMessage(personal_update_text),
                });
            }
            None => {}
        }

        let public_notification = "[font=".to_owned()
            + FURTHER_ITALIC_FONT
            + "]"
            + pawn_name
            + " has constructed a "
            + construction_selection
            + ".[/font]";

        match event.handle_option {
            Some(t) => {
                for sensed_by_entity in sensable_component.sensed_by.iter() {
                    match handle_to_entity.inv_map.get(sensed_by_entity) {
                        Some(senser_handle) => {
                            if senser_handle != &t {
                                net_construction_tool.send(NetConstructionTool {
                                    handle: *senser_handle,
                                    message: ReliableServerMessage::ChatMessage(
                                        public_notification.clone(),
                                    ),
                                });
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }

        // Send netcode message to all clients who see this tile that it has been updated.
        for (mut senser_component, _connected_player_component) in sensers.iter_mut() {
            if senser_component.fov.is_in_fov(coords.0, coords.1) {
                senser_component.fov.clear_fov();
                let coords =
                    to_doryen_coordinates(senser_component.cell_id.x, senser_component.cell_id.y);
                senser_component.fov.compute_fov(
                    &mut fov_map.map,
                    coords.0,
                    coords.1,
                    FOV_DISTANCE,
                    true,
                );
            }
        }

        let mut rng = rand::thread_rng();
        let random_pick: i32 = rng.gen_range(0..2);
        let sfx_entity;

        if random_pick == 0 {
            sfx_entity = sfx_builder(
                &mut commands,
                *rigid_body_position_component,
                Box::new(Construct1SfxBundle::new),
            );
        } else {
            sfx_entity = sfx_builder(
                &mut commands,
                *rigid_body_position_component,
                Box::new(Construct2SfxBundle::new),
            );
        }

        sfx_auto_destroy(sfx_entity, &mut sfx_auto_destroy_timers);
    }
}
