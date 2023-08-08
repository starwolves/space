use bevy::{
    prelude::{
        info, warn, BuildChildren, Button, ButtonBundle, Changed, Color, Commands, Component,
        DespawnRecursiveExt, Entity, EventReader, EventWriter, NodeBundle, Query, Res, ResMut,
        Resource, TextBundle, With, Without,
    },
    text::TextStyle,
    ui::{AlignItems, FlexDirection, Interaction, JustifyContent, Style, Val},
};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use resources::hud::HudState;
use ui::{
    fonts::{Fonts, ARIZONE_FONT, EMPIRE_FONT},
    net::{TextTreeInput, TextTreeSelection, UiClientMessage, UiServerMessage},
};

use crate::{
    expand::ExpandInventoryHud,
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
    fonts: Res<Fonts>,
    mut state: ResMut<TextTreeInputSelectionState>,
) {
    for message in events.iter() {
        match &message.message {
            UiServerMessage::TextTreeSelection(selection) => {
                let arizone_font = fonts.handles.get(ARIZONE_FONT).unwrap();
                let empire_font = fonts.handles.get(EMPIRE_FONT).unwrap();

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
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
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
                                            width: Val::Percent(100.),
                                            height: Val::Percent(3.),
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
                                        width: Val::Percent(100.),
                                        height: Val::Percent(8.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                });
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(65.),
                                            height: Val::Percent(36.),
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
                                                        width: Val::Percent(100.),
                                                        height: Val::Percent(10.),
                                                        ..Default::default()
                                                    },

                                                    ..Default::default()
                                                })
                                                .with_children(|parent| {
                                                    parent
                                                        .spawn(ButtonBundle {
                                                            style: Style {
                                                                width: Val::Percent(100.),
                                                                height: Val::Percent(100.),
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
                                                            entry: entry.clone(),
                                                        })
                                                        .insert(ButtonSelectionStyle::default())
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

                                        parent
                                            .spawn(ButtonBundle {
                                                style: Style {
                                                    width: Val::Percent(35.),
                                                    height: Val::Percent(30.),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..Default::default()
                                                },
                                                background_color: ACTIONS_HUD_BG_COLOR.into(),

                                                ..Default::default()
                                            })
                                            .insert(ButtonSelectionStyle::default())
                                            .insert(TextTreeSelectionSubmitButton)
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    "Submit".clone(),
                                                    TextStyle {
                                                        font_size: 13.0,
                                                        color: Color::WHITE,
                                                        font: empire_font.clone(),
                                                    },
                                                ));
                                            });
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
    pub entry: String,
}
#[derive(Component)]
pub struct TextTreeSelectionSubmitButton;
#[derive(Resource, Default)]
pub struct TextTreeSelectionState {
    pub selected: Option<SelectedTree>,
}
#[derive(Debug)]
pub struct SelectedTree {
    pub node_entity: Entity,
    pub selection: TextTreeSelection,
    pub entry: String,
}
pub(crate) fn text_tree_select_button(
    interaction_query: Query<
        (Entity, &Interaction, &TextTreeSelectionButton),
        (
            Changed<Interaction>,
            With<Button>,
            With<ButtonSelectionStyle>,
            Without<TextTreeSelectionSubmitButton>,
        ),
    >,
    mut state: ResMut<TextTreeSelectionState>,
    mut style_query: Query<&mut ButtonSelectionStyle>,
) {
    let mut old_entity = None;
    for (entity, interaction, component) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                match &state.selected {
                    Some(e) => {
                        if e.node_entity == entity {
                            continue;
                        }
                        old_entity = Some(e.node_entity);
                    }
                    None => {}
                }
                state.selected = Some(SelectedTree {
                    node_entity: entity,
                    selection: component.data.clone(),
                    entry: component.entry.clone(),
                });

                match style_query.get_mut(entity) {
                    Ok(mut stil) => {
                        stil.selected = true;
                    }
                    Err(_) => {
                        warn!("Couldnt find style.");
                    }
                }
            }
            _ => (),
        }
    }
    match old_entity {
        Some(ent) => match style_query.get_mut(ent) {
            Ok(mut style) => {
                style.selected = false;
            }
            Err(_) => {
                //warn!("Couldnt find previous selection.");
            }
        },
        None => {}
    }
}
pub(crate) fn text_tree_select_submit_button(
    interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<ButtonSelectionStyle>,
            With<TextTreeSelectionSubmitButton>,
        ),
    >,
    state: Res<TextTreeSelectionState>,
    mut net: EventWriter<OutgoingReliableClientMessage<UiClientMessage>>,
) {
    for interaction in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => match &state.selected {
                Some(tree) => {
                    net.send(OutgoingReliableClientMessage {
                        message: UiClientMessage::TextTreeInput(TextTreeInput {
                            entity: tree.selection.entity,
                            id: tree.selection.id.clone(),
                            entry: tree.entry.clone(),
                        }),
                    });
                    info!("Send submit button message.");
                }
                None => {}
            },
            _ => (),
        }
    }
}
