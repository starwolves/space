use bevy_app::{EventReader, EventWriter};
use bevy_ecs::{
    entity::Entity,
    system::{Query, Res},
};
use bevy_math::Vec3;
use bevy_rapier3d::prelude::RigidBodyPositionComponent;

use crate::space::{
    core::{
        connected_player::events::TextTreeInputSelection,
        data_link::components::DataLink,
        entity::{components::EntityData, resources::EntityDataResource},
        inventory::components::Inventory,
        inventory_item::components::InventoryItem,
        pawn::components::Pawn,
    },
    entities::construction_tool_admin::events::InputConstructionOptionsSelection,
};

pub fn text_tree_input_selection(
    mut input_events: EventReader<TextTreeInputSelection>,

    mut input_construction_options_selection: EventWriter<InputConstructionOptionsSelection>,

    pawns: Query<(&Pawn, &RigidBodyPositionComponent, &Inventory, &DataLink)>,
    inventory_items: Query<(&RigidBodyPositionComponent, &InventoryItem)>,

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
                                                .position
                                                .translation
                                                .into();
                                        let pos2: Vec3 = rigid_body_position_component
                                            .position
                                            .translation
                                            .into();

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

        if event.menu_id == "constructiontoolco" && belonging_entity.is_some() {
            input_construction_options_selection.send(InputConstructionOptionsSelection {
                handle: event.handle,
                menu_selection: event.menu_selection.clone(),
                entity: belonging_entity.unwrap(),
            });
        }
    }
}
