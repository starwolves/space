use std::collections::HashMap;

use bevy::color::Srgba;
use bevy::log::warn;
use bevy::{
    prelude::{
        BuildChildren, Color, Commands, Component, Entity, Event, EventReader, EventWriter,
        NodeBundle, Query, Res, ResMut, Resource, SystemSet, TextBundle, Visibility, With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val},
};

use player::configuration::Boarded;
use resources::{hud::HudState, input::InputBuffer, ui::TextInput};
use ui::fonts::{Fonts, ARIZONE_FONT};

use crate::{
    build::ExpandedLeftContentHud, expand::ExpandInventoryHud, input::binds::TOGGLE_INVENTORY,
};

use super::slots::InventorySlotsNode;

pub const INVENTORY_SLOTS_BG_COLOR: Color = Color::srgba(0.25, 0.25, 0.25, 0.9);

pub(crate) fn create_inventory_hud(
    mut commands: Commands,
    hud_state: Res<HudState>,
    fonts: Res<Fonts>,
) {
    let arizone_font = fonts.handles.get(ARIZONE_FONT).unwrap();

    let mut inventory_hud_color = bevy::color::palettes::css::MIDNIGHT_BLUE;
    inventory_hud_color = Srgba {
        alpha: 1.0,
        ..Srgba::from(inventory_hud_color)
    };

    let entity_id = commands.spawn(InventoryHudRootNode).id();
    commands
        .entity(hud_state.center_content_node)
        .add_child(entity_id);
    let mut root_builder = commands.entity(entity_id);

    let mut slots_node = None;

    root_builder
        .insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            background_color: inventory_hud_color.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(3.),
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
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
                                        font: arizone_font.clone(),
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
                        width: Val::Percent(97.5),
                        height: Val::Percent(95.75),
                        justify_content: JustifyContent::Center,
                        padding: UiRect::new(
                            Val::Px(0.),
                            Val::Px(0.),
                            Val::Percent(1.25),
                            Val::Px(0.),
                        ),
                        ..Default::default()
                    },
                    background_color: INVENTORY_SLOTS_BG_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    slots_node = Some(
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(97.5),
                                    height: Val::Percent(100.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .insert(InventorySlotsNode)
                            .id(),
                    );
                });
        });

    commands.insert_resource(InventoryHudState {
        open: false,
        root_node: entity_id,
        slots_node: slots_node.unwrap(),
        slots: HashMap::new(),

        active_item: None,
        item_to_node: HashMap::new(),
    });
}

#[derive(Component)]
pub struct InventoryHudRootNode;

pub(crate) fn open_inventory_hud(
    boarded: Res<Boarded>,
    mut events: EventReader<OpenInventoryHud>,
    mut state: ResMut<InventoryHudState>,
    mut expand: EventWriter<ExpandInventoryHud>,
    mut expand2: EventWriter<ExpandedLeftContentHud>,
    mut expand3: EventWriter<OpenHud>,

    mut root_node: Query<&mut Visibility, With<InventoryHudRootNode>>,
) {
    for event in events.read() {
        if !boarded.boarded {
            continue;
        }
        match root_node.get_mut(state.root_node) {
            Ok(mut root) => {
                if !state.open {
                    *root = Visibility::Inherited;
                } else {
                    *root = Visibility::Hidden;
                }
            }
            Err(_) => {
                warn!("Couldnt toggle open inventory , couldnt find root node.");
            }
        }
        state.open = event.open;
        expand3.send(OpenHud { open: state.open });
        expand.send(ExpandInventoryHud { expand: state.open });
        if !state.open {
            expand2.send(ExpandedLeftContentHud {
                expanded: state.open,
            });
        }
    }
}

/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum OpenHudSet {
    Process,
    ExpandInventory,
}

pub fn open_hud(
    boarded: Res<Boarded>,
    mut events: EventReader<OpenHud>,
    mut state: ResMut<HudState>,
) {
    for event in events.read() {
        if !boarded.boarded {
            continue;
        }
        state.expanded = event.open;
    }
}

pub(crate) fn inventory_hud_key_press(
    keys: Res<InputBuffer>,
    mut event2: EventWriter<OpenInventoryHud>,

    state: Res<InventoryHudState>,
    focus: Res<TextInput>,
) {
    if keys.just_pressed(TOGGLE_INVENTORY) && focus.focused_input.is_none() {
        event2.send(OpenInventoryHud { open: !state.open });
    }
}

#[derive(Event)]
pub struct OpenHud {
    pub open: bool,
}
#[derive(Event)]
pub struct OpenInventoryHud {
    pub open: bool,
}

#[derive(Resource)]
pub struct InventoryHudState {
    pub open: bool,
    pub root_node: Entity,
    pub slots_node: Entity,
    pub slots: HashMap<u8, Entity>,

    pub active_item: Option<Entity>,
    pub item_to_node: HashMap<Entity, Entity>,
}
