use bevy::{
    prelude::{
        warn, AssetServer, BuildChildren, Color, Commands, Component, Entity, EventReader,
        EventWriter, Input, KeyCode, NodeBundle, Query, Res, ResMut, Resource, TextBundle,
        Visibility, With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, UiRect, Val},
};
use inventory::{
    inventory::{Inventory, ItemAddedToSlot, Slot},
    net::InventoryServerMessage,
};
use networking::client::IncomingReliableServerMessage;
use player::{configuration::Boarded, net::PlayerServerMessage};

use crate::{expand::ExpandHud, hud::HudState};

pub struct OpenInventoryHud {
    pub open: bool,
}

#[derive(Resource)]
pub struct InventoryState {
    pub open: bool,
    pub root_node: Entity,
    pub slots_node: Entity,
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

                commands.insert_resource(InventoryState {
                    open: false,
                    root_node: entity_id,
                    slots_node,
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
    mut state: ResMut<InventoryState>,
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
    state: Res<InventoryState>,
) {
    if keys.just_pressed(KeyCode::I) {
        event.send(OpenInventoryHud { open: !state.open });
    }
}

/// Resource that queues inventory updates. For when we receive them before the client has fully initialized the inventory and UI.
#[derive(Resource, Default)]
pub struct InventoryUpdatesQueue {
    pub flushed: bool,
    pub item_updates: Vec<ItemAddedToSlot>,
    pub slot_updates: Vec<Slot>,
}

pub struct HudAddInventorySlot {
    pub slot: Slot,
}
pub struct HudAddItemToSlot {
    pub item: ItemAddedToSlot,
}

pub(crate) fn inventory_net_updates(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut queue: ResMut<InventoryUpdatesQueue>,
    mut slot_event: EventWriter<HudAddInventorySlot>,
    mut item_event: EventWriter<HudAddItemToSlot>,
) {
    if queue.flushed == false {
        queue.flushed = true;

        for slot in queue.slot_updates.clone() {
            slot_event.send(HudAddInventorySlot { slot });
        }

        for item in queue.item_updates.clone() {
            item_event.send(HudAddItemToSlot { item });
        }

        queue.item_updates.clear();
        queue.slot_updates.clear();
    }

    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(item) => {
                item_event.send(HudAddItemToSlot { item: item.clone() });
            }
            InventoryServerMessage::AddedSlot(slot) => {
                slot_event.send(HudAddInventorySlot { slot: slot.clone() });
            }
        }
    }
}

pub(crate) fn queue_inventory_updates(
    mut net: EventReader<IncomingReliableServerMessage<InventoryServerMessage>>,
    mut queue: ResMut<InventoryUpdatesQueue>,
) {
    for message in net.iter() {
        match &message.message {
            InventoryServerMessage::ItemAddedToSlot(item) => {
                queue.item_updates.push(item.clone());
            }
            InventoryServerMessage::AddedSlot(slot) => {
                queue.slot_updates.push(slot.clone());
            }
        }
    }
}

pub(crate) fn update_inventory_hud(
    state: Res<InventoryState>,
    inventory: Res<Inventory>,
    mut update_slot: EventReader<HudAddInventorySlot>,
    mut update_item: EventReader<HudAddItemToSlot>,
    mut commands: Commands,
) {
    for event in update_slot.iter() {
        let width = (event.slot.size.x as f32 / 16.) * 100.;
        let height = (event.slot.size.y as f32 / 16.) * 100.;

        let slot_node = commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(width), Val::Percent(height * 0.5)),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                background_color: Color::GRAY.into(),
                ..Default::default()
            })
            .with_children(|parent| {})
            .id();

        let mut slots_node = commands.entity(state.slots_node);
        slots_node.add_child(slot_node);
    }
}
