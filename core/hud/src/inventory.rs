use std::collections::HashMap;

use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Color, Commands, Component, Entity, EventReader,
        EventWriter, Input, KeyCode, NodeBundle, Query, Res, ResMut, Resource, SystemLabel,
        TextBundle, Visibility, With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, UiRect, Val},
};
use entity::{
    entity_types::{EntityType, EntityTypes},
    spawn::EntityBuildData,
};
use inventory::{
    client::slots::AddedSlot, net::InventoryServerMessage, server::inventory::ItemAddedToSlot,
    spawn_item::InventoryItemBuilder,
};
use networking::client::IncomingReliableServerMessage;
use player::{configuration::Boarded, net::PlayerServerMessage};

use crate::{expand::ExpandHud, hud::HudState};

pub struct OpenInventoryHud {
    pub open: bool,
}

#[derive(Resource)]
pub struct InventoryHudState {
    pub open: bool,
    pub root_node: Entity,
    pub slots_node: Entity,
    pub slots: HashMap<u8, Entity>,
}

#[derive(Component)]
pub struct InventorySlotsNode;

pub(crate) fn create_inventory_hud(
    mut commands: Commands,
    hud_state: Res<HudState>,
    mut client: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    asset_server: Res<AssetServer>,
) {
    for message in client.iter() {
        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");

        match message.message {
            PlayerServerMessage::Boarded => {
                let entity_id = commands.spawn(InventoryHudRootNode).id();
                commands
                    .entity(hud_state.center_content_node)
                    .add_child(entity_id);
                let mut root_builder = commands.entity(entity_id);

                let mut slots_node = Entity::from_bits(0);

                root_builder
                    .insert(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        background_color: Color::MIDNIGHT_BLUE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.0), Val::Percent(3.0)),
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                background_color: Color::MIDNIGHT_BLUE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            align_items: AlignItems::Center,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle {
                                            text: Text::from_section(
                                                "Inventory".to_string(),
                                                TextStyle {
                                                    font: arizone_font,
                                                    font_size: 13.,
                                                    color: Color::WHITE,
                                                },
                                            ),
                                            ..Default::default()
                                        });
                                    });
                            });
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(97.5), Val::Percent(95.75)),
                                    justify_content: JustifyContent::Center,
                                    padding: UiRect::new(
                                        Val::Undefined,
                                        Val::Undefined,
                                        Val::Percent(1.25),
                                        Val::Undefined,
                                    ),
                                    ..Default::default()
                                },
                                background_color: Color::DARK_GRAY.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                slots_node = parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(97.5), Val::Percent(100.)),

                                            ..Default::default()
                                        },
                                        background_color: Color::DARK_GRAY.into(),
                                        ..Default::default()
                                    })
                                    .insert(InventorySlotsNode)
                                    .id();
                            });
                    });

                commands.insert_resource(InventoryHudState {
                    open: false,
                    root_node: entity_id,
                    slots_node,
                    slots: HashMap::new(),
                });
            }
            _ => {}
        }
    }
}

#[derive(Component)]
pub struct InventoryHudRootNode;

pub(crate) fn open_inventory_hud(
    boarded: Res<Boarded>,
    mut events: EventReader<OpenInventoryHud>,
    mut state: ResMut<InventoryHudState>,
    mut root_node: Query<&mut Visibility, With<InventoryHudRootNode>>,
    mut expand: EventWriter<ExpandHud>,
) {
    for event in events.iter() {
        if !boarded.boarded {
            continue;
        }

        state.open = event.open;
        match root_node.get_mut(state.root_node) {
            Ok(mut root) => {
                root.is_visible = state.open;
            }
            Err(_) => {
                warn!("Couldnt toggle open inventory , couldnt find root node.");
            }
        }
        expand.send(ExpandHud { expand: state.open });
    }
}

pub(crate) fn inventory_hud_key_press(
    keys: Res<Input<KeyCode>>,
    mut event: EventWriter<OpenInventoryHud>,
    state: Res<InventoryHudState>,
) {
    if keys.just_pressed(KeyCode::I) {
        event.send(OpenInventoryHud { open: !state.open });
    }
}

/// Resource that queues inventory updates. For when we receive them before the client has fully initialized the inventory and UI.
#[derive(Resource, Clone, Default)]
pub struct InventoryUpdatesQueue {
    pub flushed: bool,
    pub item_updates: Vec<ItemAddedToSlot>,
    pub slot_updates: Vec<AddedSlot>,
}

pub struct HudAddInventorySlot {
    pub slot: AddedSlot,
}
#[derive(Clone)]
pub struct HudAddItemToSlot {
    pub item: ItemAddedToSlot,
}

pub(crate) fn inventory_net_updates(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut queue: ResMut<InventoryUpdatesQueue>,
    mut slot_event: EventWriter<HudAddInventorySlot>,
    mut item_event: EventWriter<HudAddItemToSlot>,
    mut added_slot_events: EventReader<AddedSlot>,
) {
    let mut to_be_added_slot_ids = vec![];
    let mut to_be_added_items = vec![];

    if queue.flushed == false {
        queue.flushed = true;

        for slot in queue.slot_updates.clone() {
            slot_event.send(HudAddInventorySlot { slot: slot.clone() });
            to_be_added_slot_ids.push(slot.id);
        }

        for item in queue.item_updates.clone() {
            item_event.send(HudAddItemToSlot { item: item.clone() });
            to_be_added_items.push((item.item_entity, item.slot_id));
        }

        queue.item_updates.clear();
        queue.slot_updates.clear();
    }

    for event in added_slot_events.iter() {
        if to_be_added_slot_ids.contains(&event.id) {
            continue;
        }
        slot_event.send(HudAddInventorySlot {
            slot: event.clone(),
        });
    }

    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(item) => {
                let mut found = false;
                for (entity, slot_id) in to_be_added_items.iter() {
                    if entity == &item.item_entity && slot_id == &item.slot_id {
                        found = true;
                        break;
                    }
                }
                if found {
                    continue;
                }
                item_event.send(HudAddItemToSlot { item: item.clone() });
            }
            _ => (),
        }
    }
}

pub(crate) fn queue_inventory_updates(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut queue: ResMut<InventoryUpdatesQueue>,
    mut added_slot_events: EventReader<AddedSlot>,
) {
    for event in added_slot_events.iter() {
        queue.slot_updates.push(event.clone());
    }
    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(item) => {
                queue.item_updates.push(item.clone());
            }
            _ => (),
        }
    }
}
#[derive(Component)]
pub struct SlotHud;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum InventoryHudLabels {
    UpdateSlot,
    QueueUpdate,
}

pub(crate) fn update_inventory_hud_slot(
    mut state: ResMut<InventoryHudState>,
    mut update_slot: EventReader<HudAddInventorySlot>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in update_slot.iter() {
        let width = (event.slot.slot.size.x as f32 / 16.) * 100.;
        let height = (event.slot.slot.size.y as f32 / 16.) * 100.;
        let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");

        commands.entity(state.slots_node).with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(width), Val::Percent((height * 0.5) + 10.)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                event.slot.slot.name.clone() + ":",
                                TextStyle {
                                    font: arizone_font.clone(),
                                    font_size: 12.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                    // The inventory grid space.
                    let slot_entity = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(90.)),
                                ..Default::default()
                            },
                            background_color: Color::GRAY.into(),
                            ..Default::default()
                        })
                        .insert(SlotHud)
                        .id();
                    state.slots.insert(event.slot.id, slot_entity);
                });
        });
    }
}

pub struct RequeueHudAddItemToSlot {
    pub queued: HudAddItemToSlot,
}

pub(crate) fn requeue_hud_add_item_to_slot(
    mut i_events: EventReader<RequeueHudAddItemToSlot>,
    mut o_events: EventWriter<HudAddItemToSlot>,
) {
    for event in i_events.iter() {
        o_events.send(event.queued.clone());
    }
}

pub fn update_inventory_hud_add_item_to_slot<
    T: InventoryItemBuilder + EntityType + Default + Send + Sync + 'static,
>(
    mut update_item: EventReader<HudAddItemToSlot>,
    mut queue: EventWriter<RequeueHudAddItemToSlot>,
    mut commands: Commands,
    state: Res<InventoryHudState>,
    types: Res<EntityTypes>,
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

        if item_type_identity != identity.to_string() {
            continue;
        }

        let inventory_item_bundle = T::default().get_bundle(&EntityBuildData::default());

        let width = (inventory_item_bundle.inventory_item.slot_size.x as f32 / 16.) * 100.;
        let height = (inventory_item_bundle.inventory_item.slot_size.y as f32 / 16.) * 100.;

        commands.entity(slot_entity).with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(width), Val::Percent(height)),
                    ..Default::default()
                },
                background_color: Color::BEIGE.into(),
                ..Default::default()
            });
        });
    }
}
