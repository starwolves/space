use actions::net::{ActionsClientMessage, TabData};
use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Color, Commands,
        Component, Entity, Event, EventReader, EventWriter, ImageBundle, Input, MouseButton,
        NodeBundle, Query, Res, ResMut, Resource, With,
    },
    ui::{BackgroundColor, Interaction, PositionType, Style, UiImage, Val},
};
use entity::{
    entity_types::{EntityType, EntityTypes},
    spawn::EntityBuildData,
};
use inventory::{
    net::{InventoryClientMessage, InventoryServerMessage},
    server::inventory::{Inventory, ItemAddedToSlot},
    spawn_item::InventoryItemBuilder,
};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use resources::{hud::HudState, math::Vec2Int};

use crate::inventory::queue::RequeueHudAddItemToSlot;

use super::build::InventoryHudState;

pub(crate) fn requeue_hud_add_item_to_slot(
    mut i_events: EventReader<RequeueHudAddItemToSlot>,
    mut o_events: EventWriter<HudAddItemToSlot>,
) {
    for event in i_events.iter() {
        o_events.send(event.queued.clone());
    }
}

pub const ITEM_SPACE_BG_ALPHA: f32 = 0.7;
pub const ITEM_SPACE_BG_COLOR: Color =
    Color::rgba(60. / 255., 0., 226. / 255., ITEM_SPACE_BG_ALPHA);
pub const ACTIVE_ITEM_SPACE_BG_COLOR: Color = Color::WHITE;

#[derive(Component)]
pub struct SlotItemHud {
    pub position: Vec2Int,
    pub slot_id: u8,
    pub entity: Entity,
}

pub fn update_inventory_hud_add_item_to_slot<
    T: InventoryItemBuilder + EntityType + Default + Send + Sync + 'static,
>(
    mut update_item: EventReader<HudAddItemToSlot>,
    mut queue: EventWriter<RequeueHudAddItemToSlot>,
    mut commands: Commands,
    mut state: ResMut<InventoryHudState>,
    types: Res<EntityTypes>,
    inventory: Res<Inventory>,
    asset_server: Res<AssetServer>,
) {
    for event in update_item.iter() {
        let slot_entity;
        match state.slots.get(&event.item.slot_id) {
            Some(ent) => {
                slot_entity = *ent;
            }
            None => {
                info!("Queueing HudAddItemToSlot..");
                queue.send(RequeueHudAddItemToSlot {
                    queued: event.clone(),
                });
                continue;
            }
        }
        let mut type_id_option = None;
        for (id, n) in types.netcode_types.iter() {
            if &event.item.item_type_id == n {
                type_id_option = Some(id.clone());
                break;
            }
        }
        let item_type_identity;
        match type_id_option {
            Some(i) => {
                item_type_identity = i;
            }
            None => {
                warn!("Couldnt find entity id from netcode type.");
                continue;
            }
        }

        let identity = T::default();

        if item_type_identity != identity.get_identity() {
            continue;
        }

        let slot;
        match inventory.slots.get(&event.item.slot_id) {
            Some(s) => {
                slot = s;
            }
            None => {
                warn!("Couldnt find slot!");
                continue;
            }
        }

        let entity_type = T::default();

        let inventory_item_bundle = entity_type.get_bundle(&EntityBuildData::default());

        let width = (inventory_item_bundle.inventory_item.slot_size.x as f32 / 16.) * 100.;
        let height = ((inventory_item_bundle.inventory_item.slot_size.y as f32 / 16.) * 100.)
            * (slot.size.x as f32 / slot.size.y as f32);

        let mut corrected_item_position = event.item.position.clone();
        corrected_item_position.x += slot.size.x / 2;
        corrected_item_position.y += slot.size.y / 2;
        corrected_item_position.y -= inventory_item_bundle.inventory_item.slot_size.y;

        let x = corrected_item_position.x as f32 * 12.5;
        let y = corrected_item_position.y as f32 * 12.5;

        let item_image = asset_server
            .load("entities/".to_string() + &entity_type.get_clean_identity() + "/item.png");

        commands.entity(slot_entity).with_children(|parent| {
            let mut builder = parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(width),
                    height: Val::Percent(height),
                    position_type: PositionType::Absolute,
                    left: Val::Percent(x),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Percent(y),

                    ..Default::default()
                },
                background_color: ITEM_SPACE_BG_COLOR.into(),
                ..Default::default()
            });
            let data_parent = builder.id();
            state
                .item_to_node
                .insert(event.item.item_entity, data_parent);
            builder
                .insert(SlotItemHud {
                    position: event.item.position.clone(),
                    slot_id: event.item.slot_id,
                    entity: event.item.item_entity,
                })
                .with_children(|parent| {
                    let mut empty_color = Color::BLACK;
                    empty_color.set_a(0.);
                    parent
                        .spawn(ImageBundle {
                            style: Style {
                                width: Val::Percent(100.),
                                height: Val::Percent(100.),
                                ..Default::default()
                            },
                            image: UiImage::from(item_image),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        height: Val::Percent(100.),
                                        ..Default::default()
                                    },
                                    background_color: empty_color.into(),
                                    ..Default::default()
                                })
                                .insert(SlotItemButtonHud { data_parent });
                        });
                });
        });
    }
}
#[derive(Clone, Event)]
pub struct HudAddItemToSlot {
    pub item: ItemAddedToSlot,
}

#[derive(Component)]
pub struct SlotItemButtonHud {
    pub data_parent: Entity,
}

#[derive(Default, Resource)]
pub struct HoveringSlotItem {
    // Holds server entity of item that is hovered.
    pub option: Option<Entity>,
}

pub(crate) fn right_mouse_click_item(
    buttons: Res<Input<MouseButton>>,
    state: Res<HoveringSlotItem>,
    mut actions_net: EventWriter<OutgoingReliableClientMessage<ActionsClientMessage>>,
    inventory_state: Res<InventoryHudState>,
    hud_state: Res<HudState>,
) {
    if !inventory_state.open || !hud_state.expanded {
        return;
    }
    if buttons.just_pressed(MouseButton::Right) {
        match state.option {
            Some(e) => {
                actions_net.send(OutgoingReliableClientMessage {
                    message: ActionsClientMessage::TabData(TabData {
                        action_taker_item: None,
                        target_cell_option: None,
                        target_entity_option: Some(e),
                    }),
                });
            }
            None => {}
        }
    }
}

pub(crate) fn slot_item_button_events(
    interaction_query: Query<
        (&Interaction, &SlotItemButtonHud),
        (Changed<Interaction>, With<Button>),
    >,
    mut slot_items_query: Query<(&SlotItemHud, &mut BackgroundColor)>,
    inventory: Res<Inventory>,
    mut inventory_net: EventWriter<OutgoingReliableClientMessage<InventoryClientMessage>>,
    mut state: ResMut<HoveringSlotItem>,
) {
    for (interaction, component) in interaction_query.iter() {
        match slot_items_query.get_mut(component.data_parent) {
            Ok((slot_item_hud, mut background_color)) => match *interaction {
                Interaction::Pressed => {
                    match inventory.active_item {
                        Some(item) => {
                            if item == slot_item_hud.entity {
                                continue;
                            }
                        }
                        None => {}
                    }
                    info!("Sending RequestSetActiveItem");
                    inventory_net.send(OutgoingReliableClientMessage {
                        message: InventoryClientMessage::RequestSetActiveItem(slot_item_hud.entity),
                    });
                }
                Interaction::Hovered => {
                    state.option = Some(slot_item_hud.entity);
                    background_color.0.set_a(1.);
                }
                Interaction::None => {
                    state.option = None;
                    background_color.0.set_a(ITEM_SPACE_BG_ALPHA);
                }
            },
            Err(_) => {
                warn!("Couldnt find parent entity node.");
            }
        }
    }
}

/// Chang bg color of active item to white and change color back to old one.
pub fn change_active_item(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    inventory: Res<InventoryHudState>,
    mut node_query: Query<&mut BackgroundColor, With<SlotItemHud>>,
) {
    for event in net.iter() {
        match event.message {
            InventoryServerMessage::SetActiveItem(entity) => {
                info!("Receive SetActiveItem 1");

                match inventory.active_item {
                    Some(old_focus) => match inventory.item_to_node.get(&old_focus) {
                        Some(old_focus_node) => match node_query.get_mut(*old_focus_node) {
                            Ok(mut bg) => {
                                bg.0 = ITEM_SPACE_BG_COLOR;
                            }
                            Err(_) => {
                                warn!("Couldnt find old focus node entity.");
                                continue;
                            }
                        },
                        None => {
                            warn!("Couldnt find 0ld focus in item_to_node map.");
                            continue;
                        }
                    },
                    None => {}
                }
                match inventory.item_to_node.get(&entity) {
                    Some(e) => match node_query.get_mut(*e) {
                        Ok(mut bg) => bg.0 = ACTIVE_ITEM_SPACE_BG_COLOR,
                        Err(_) => {
                            warn!("Couldnt find new focus node entity.");
                            continue;
                        }
                    },
                    None => {
                        warn!("Couldnt find entity in item_to_node map.");
                        continue;
                    }
                }
            }
            _ => (),
        }
    }
}
