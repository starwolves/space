use bevy::{
    prelude::{
        AssetServer, BuildChildren, ButtonBundle, Color, Commands, Component, DespawnRecursiveExt,
        Entity, EventReader, NodeBundle, Res, ResMut, Resource, TextBundle,
    },
    text::TextStyle,
    ui::{AlignItems, FlexDirection, JustifyContent, Size, Style, Val},
};
use networking::client::IncomingReliableServerMessage;
use ui::{
    fonts::{ARIZONE_FONT, EMPIRE_FONT},
    networking::{TextTreeSelection, UiServerMessage},
};

use crate::{
    expand::ExpandInventoryHud,
    hud::HudState,
    inventory::actions::{ACTIONS_HUD_BG_COLOR, INVENTORY_HUD_BG_COLOR},
    style::button::ButtonSelectionStyle,
};

#[derive(Component)]
pub struct TextTreeInputSelectionRootNode;
#[derive(Resource, Default)]
pub struct TextTreeInputSelectionState {
    pub entity: Option<Entity>,
}

pub(crate) fn hide_text_tree_selection(
    mut events: EventReader<ExpandInventoryHud>,
    mut state: ResMut<TextTreeInputSelectionState>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if !event.expand {
            match state.entity {
                Some(entity) => {
                    commands.entity(entity).despawn_recursive();
                }
                None => {}
            }
            state.entity = None;
        }
    }
}

pub(crate) fn create_text_tree_selection(
    mut events: EventReader<IncomingReliableServerMessage<UiServerMessage>>,
    hud_state: Res<HudState>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<TextTreeInputSelectionState>,
) {
    for message in events.iter() {
        match &message.message {
            UiServerMessage::TextTreeSelection(selection) => {
                let arizone_font = asset_server.load(ARIZONE_FONT);
                let empire_font = asset_server.load(EMPIRE_FONT);

                match state.entity {
                    Some(old_entity) => {
                        commands.entity(old_entity).despawn_recursive();
                    }
                    None => {}
                }

                commands
                    .entity(hud_state.right_content_node)
                    .with_children(|parent| {
                        let root = parent
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                background_color: INVENTORY_HUD_BG_COLOR.into(),
                                ..Default::default()
                            })
                            .insert(TextTreeInputSelectionRootNode)
                            .with_children(|parent| {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.), Val::Percent(3.)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,

                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            selection.text.clone(),
                                            TextStyle {
                                                font_size: 13.0,
                                                color: Color::WHITE,
                                                font: arizone_font.clone(),
                                            },
                                        ));
                                    });
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.), Val::Percent(8.)),

                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(65.), Val::Percent(36.)),
                                            flex_direction: FlexDirection::Column,
                                            ..Default::default()
                                        },
                                        background_color: ACTIONS_HUD_BG_COLOR.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        for entry in selection.entries.iter() {
                                            parent
                                                .spawn(NodeBundle {
                                                    style: Style {
                                                        justify_content: JustifyContent::Center,
                                                        align_items: AlignItems::Center,
                                                        size: Size::new(
                                                            Val::Percent(100.),
                                                            Val::Percent(10.),
                                                        ),
                                                        ..Default::default()
                                                    },

                                                    ..Default::default()
                                                })
                                                .with_children(|parent| {
                                                    parent
                                                        .spawn(ButtonBundle {
                                                            style: Style {
                                                                size: Size::new(
                                                                    Val::Percent(100.),
                                                                    Val::Percent(100.),
                                                                ),
                                                                justify_content:
                                                                    JustifyContent::Center,
                                                                align_items: AlignItems::Center,
                                                                ..Default::default()
                                                            },
                                                            background_color: ACTIONS_HUD_BG_COLOR
                                                                .into(),

                                                            ..Default::default()
                                                        })
                                                        .insert(TextTreeSelectionButton {
                                                            data: selection.clone(),
                                                        })
                                                        .insert(ButtonSelectionStyle)
                                                        .with_children(|parent| {
                                                            parent.spawn(TextBundle::from_section(
                                                                entry.clone(),
                                                                TextStyle {
                                                                    font_size: 13.0,
                                                                    color: Color::WHITE,
                                                                    font: empire_font.clone(),
                                                                },
                                                            ));
                                                        });
                                                });
                                        }
                                    });
                            })
                            .id();
                        state.entity = Some(root);
                    });
            }
            _ => (),
        }
    }
}
#[derive(Component)]
pub struct TextTreeSelectionButton {
    pub data: TextTreeSelection,
}
