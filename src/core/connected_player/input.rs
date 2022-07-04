pub struct InputToggleAutoMove {
    pub entity: Entity,
}

pub struct InputAttackCell {
    pub entity: Entity,
    pub id: Vec3Int,
}
pub struct InputAttackEntity {
    pub entity: Entity,
    pub target_entity_bits: u64,
}

pub struct InputAltItemAttack {
    pub entity: Entity,
}

pub struct InputMouseAction {
    pub entity: Entity,
    pub pressed: bool,
}
pub struct InputSelectBodyPart {
    pub entity: Entity,
    pub body_part: String,
}

pub fn controller_input(
    mut alternative_item_attack_events: EventReader<InputAltItemAttack>,
    mut input_attack_entity: EventReader<InputAttackEntity>,
    mut input_attack_cell: EventReader<InputAttackCell>,
    mut input_mouse_action_events: EventReader<InputMouseAction>,
    mut input_select_body_part: EventReader<InputSelectBodyPart>,
    mut input_toggle_auto_move: EventReader<InputToggleAutoMove>,
    mut humanoids_query: Query<(&Humanoid, &mut ControllerInput)>,
) {
    for event in alternative_item_attack_events.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut controller_input_component) => {
                controller_input_component.alt_attack_mode =
                    !controller_input_component.alt_attack_mode;
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of InputAltItemAttack.");
            }
        }
    }

    for event in input_attack_cell.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut controller_input_component) => {
                controller_input_component.combat_targetted_cell = Some(event.id);
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of input_attack_cell.");
            }
        }
    }

    for event in input_attack_entity.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut played_input_component) => {
                played_input_component.combat_targetted_entity =
                    Some(Entity::from_bits(event.target_entity_bits));
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of InputAttackEntity.");
            }
        }
    }

    for event in input_mouse_action_events.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut played_input_component) => {
                played_input_component.is_mouse_action_pressed = event.pressed;

                if !event.pressed {
                    played_input_component.combat_targetted_entity = None;
                    played_input_component.combat_targetted_cell = None;
                }
            }
            Err(_rr) => {
                warn!("Couldn't find standard_character_component belonging to entity of InputMouseAction.");
            }
        }
    }

    for event in input_select_body_part.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut player_input_component) => {
                player_input_component.targetted_limb = event.body_part.clone();
            }
            Err(_rr) => {
                warn!("Couldnt find PlayerInput entity for input_select_body_part");
            }
        }
    }

    for event in input_toggle_auto_move.iter() {
        match humanoids_query.get_component_mut::<ControllerInput>(event.entity) {
            Ok(mut player_input_component) => {
                player_input_component.auto_move_enabled =
                    !player_input_component.auto_move_enabled;
            }
            Err(_rr) => {
                warn!("Couldnt find PlayerInput entity for input_toggle_auto_move");
            }
        }
    }
}

pub struct InputSprinting {
    pub entity: Entity,
    pub is_sprinting: bool,
}

pub fn player_input_event(
    mut movement_input_event: EventReader<InputMovementInput>,
    mut sprinting_input_event: EventReader<InputSprinting>,
    mut query: Query<&mut ControllerInput>,
) {
    for new_event in movement_input_event.iter() {
        let player_entity = new_event.player_entity;

        let player_input_component_result = query.get_mut(player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                player_input_component.movement_vector = new_event.vector;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (movement_input_event): couldn't find player_entity.");
            }
        }
    }

    for new_event in sprinting_input_event.iter() {
        let player_entity = new_event.entity;

        let player_input_component_result = query.get_mut(player_entity);

        match player_input_component_result {
            Ok(mut player_input_component) => {
                player_input_component.sprinting = new_event.is_sprinting;
            }
            Err(_rr) => {
                warn!("Couldn't process player input (sprinting_input_event): couldn't find player_entity.");
            }
        }
    }
}

use bevy::{
    math::{Vec2, Vec3},
    prelude::{warn, Entity, EventReader, EventWriter, Query, Res, Transform},
};

use crate::{
    core::{
        data_link::data_link::DataLink,
        entity::entity_data::{EntityData, EntityDataResource},
        gridmap::gridmap::Vec3Int,
        humanoid::humanoid::Humanoid,
        inventory::inventory::Inventory,
        inventory_item::item::InventoryItem,
        pawn::pawn::{ControllerInput, Pawn},
    },
    entities::construction_tool_admin::construction_tool::InputConstructionOptionsSelection,
};

pub struct TextTreeInputSelection {
    pub handle: u64,
    pub menu_id: String,
    pub menu_selection: String,
    pub tab_action_id: String,
    pub belonging_entity: Option<u64>,
}

pub struct InputMovementInput {
    pub player_entity: Entity,
    pub vector: Vec2,
}

pub fn text_tree_input_selection(
    mut input_events: EventReader<TextTreeInputSelection>,

    mut input_construction_options_selection: EventWriter<InputConstructionOptionsSelection>,

    pawns: Query<(&Pawn, &Transform, &Inventory, &DataLink)>,
    inventory_items: Query<(&Transform, &InventoryItem)>,

    entity_data_resource: Res<EntityDataResource>,
    entity_datas: Query<&EntityData>,
) {
    for event in input_events.iter() {
        let mut belonging_entity = None;

        match event.belonging_entity {
            Some(bits) => {
                let entity = Entity::from_bits(bits);

                match inventory_items.get(entity) {
                    Ok((
                        inventory_item_rigid_body_position_component,
                        inventory_item_component,
                    )) => match inventory_item_component.in_inventory_of_entity {
                        Some(owner_entity) => match pawns.get(owner_entity) {
                            Ok((
                                pawn_component,
                                rigid_body_position_component,
                                inventory_component,
                                data_link_component,
                            )) => match pawn_component.tab_actions_data.layout.get(&Some(entity)) {
                                Some(layout) => match layout.get(&event.tab_action_id) {
                                    Some(index) => {
                                        let tab_action =
                                            pawn_component.tab_actions.get(index).unwrap();

                                        let pos1: Vec3 =
                                            inventory_item_rigid_body_position_component
                                                .translation
                                                .into();
                                        let pos2: Vec3 =
                                            rigid_body_position_component.translation.into();

                                        match (tab_action.prerequisite_check)(
                                            Some(entity),
                                            Some(bits),
                                            None,
                                            pos1.distance(pos2),
                                            inventory_component,
                                            &entity_data_resource,
                                            &entity_datas,
                                            &data_link_component,
                                        ) {
                                            true => {
                                                belonging_entity = Some(entity);
                                            }
                                            false => {}
                                        }
                                    }
                                    None => {}
                                },
                                None => {}
                            },
                            Err(_rr) => {}
                        },
                        None => {}
                    },
                    Err(_rr) => {}
                }
            }
            None => {}
        }

        if event.menu_id == "textselection::construction_tool_admin/constructionoptionslist"
            && belonging_entity.is_some()
        {
            input_construction_options_selection.send(InputConstructionOptionsSelection {
                handle_option: Some(event.handle),
                menu_selection: event.menu_selection.clone(),
                entity: belonging_entity.unwrap(),
            });
        }
    }
}
