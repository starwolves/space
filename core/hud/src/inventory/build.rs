use std::collections::HashMap;

use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Color, Commands, Component, Entity, EventReader,
        EventWriter, Input, KeyCode, NodeBundle, Query, Res, ResMut, Resource, TextBundle,
        Visibility, With,
    },
    text::{Text, TextStyle},
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, UiRect, Val},
};
use player::configuration::Boarded;
use resources::{hud::HudState, ui::TextInput};
use ui::fonts::ARIZONE_FONT;

use crate::{expand::ExpandInventoryHud, hud::ExpandedLeftContentHud};

use super::slots::InventorySlotsNode;

pub const INVENTORY_SLOTS_BG_COLOR: Color = Color::rgba(0.25, 0.25, 0.25, 0.9);

pub(crate) fn create_inventory_hud(
    mut commands: Commands,
    hud_state: Res<HudState>,
    asset_server: Res<AssetServer>,
) {
    let arizone_font = asset_server.load(ARIZONE_FONT);

    let mut inventory_hud_color = Color::MIDNIGHT_BLUE;
    inventory_hud_color.set_a(0.9);

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
            visibility: Visibility::Hidden,
            background_color: inventory_hud_color.into(),
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
                    background_color: INVENTORY_SLOTS_BG_COLOR.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    slots_node = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(97.5), Val::Percent(100.)),

                                ..Default::default()
                            },
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
    mut root_node: Query<&mut Visibility, With<InventoryHudRootNode>>,
    mut open_hud: EventWriter<OpenHud>,
) {
    for event in events.iter() {
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
        info!("open_inventory_hud {}", state.open);

        expand.send(ExpandInventoryHud { expand: state.open });
        expand2.send(ExpandedLeftContentHud {
            expanded: state.open,
        });
        open_hud.send(OpenHud { open: state.open });
    }
}

pub(crate) fn open_hud(
    boarded: Res<Boarded>,
    mut events: EventReader<OpenHud>,
    mut state: ResMut<HudState>,
) {
    for event in events.iter() {
        if !boarded.boarded {
            continue;
        }
        state.expanded = event.open;
    }
}

pub(crate) fn inventory_hud_key_press(
    keys: Res<Input<KeyCode>>,
    mut event2: EventWriter<OpenInventoryHud>,

    state: Res<InventoryHudState>,
    focus: Res<TextInput>,
) {
    if keys.just_pressed(KeyCode::I) && focus.focused_input.is_none() {
        event2.send(OpenInventoryHud { open: !state.open });
    }
}

pub struct OpenHud {
    pub open: bool,
}
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
