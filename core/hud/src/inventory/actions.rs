use actions::net::{ActionsClientMessage, ActionsServerMessage, TabPressed};
use bevy::{
    prelude::{
        info, warn, AssetServer, BuildChildren, Button, ButtonBundle, Changed, Children, Color,
        Commands, Component, DespawnRecursiveExt, Entity, EventReader, EventWriter, NodeBundle,
        Query, Res, TextBundle, With,
    },
    text::TextStyle,
    ui::{
        AlignItems, BackgroundColor, FlexDirection, Interaction, JustifyContent, Size, Style, Val,
    },
};
use networking::client::{IncomingReliableServerMessage, OutgoingReliableClientMessage};
use player::configuration::Boarded;

use crate::hud::{HudState, LeftContentHud};

use super::build::{InventoryHudState, OpenInventoryHud};

pub(crate) fn slot_item_actions(
    mut net: EventReader<IncomingReliableServerMessage<ActionsServerMessage>>,
    inventory_state: Res<InventoryHudState>,
    hud_state: Res<HudState>,
    mut commands: Commands,
    children_query: Query<&Children>,
    asset_server: Res<AssetServer>,
) {
    if !inventory_state.open || !hud_state.expanded {
        return;
    }
    for message in net.iter() {
        match &message.message {
            ActionsServerMessage::TabData(data) => {
                info!("{:?}", data);

                match children_query.get(hud_state.left_content_node) {
                    Ok(c) => {
                        for child in c.iter() {
                            commands.entity(*child).despawn_recursive();
                        }
                    }
                    Err(_) => {}
                }

                let mut builder = commands.entity(hud_state.left_content_node);

                let mut inventory_hud_color = Color::MIDNIGHT_BLUE;
                inventory_hud_color.set_a(0.9);
                let arizone_font = asset_server.load("fonts/ArizoneUnicaseRegular.ttf");
                let empire_font = asset_server.load("fonts/AAbsoluteEmpire.ttf");

                if data.len() == 0 {
                    continue;
                }

                let item_name = data.get(0).unwrap().item_name.clone();

                builder.with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: inventory_hud_color.into(),
                            ..Default::default()
                        })
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
                                        item_name,
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
                            let mut actions_bg = Color::DARK_GRAY;
                            actions_bg.set_a(0.9);
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(65.), Val::Percent(36.)),
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },

                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    for net_action in data.iter() {
                                        match net_action.entity {
                                            Some(action_server_entity) => {
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
                                                                background_color: actions_bg.into(),

                                                                ..Default::default()
                                                            })
                                                            .insert(SlotItemActionButton {
                                                                server_entity: action_server_entity,
                                                                action_id: net_action.id.clone(),
                                                            })
                                                            .with_children(|parent| {
                                                                parent.spawn(
                                                                    TextBundle::from_section(
                                                                        net_action.text.clone(),
                                                                        TextStyle {
                                                                            font_size: 13.0,
                                                                            color: Color::WHITE,
                                                                            font: empire_font
                                                                                .clone(),
                                                                        },
                                                                    ),
                                                                );
                                                            });
                                                    });
                                            }
                                            None => {}
                                        }
                                    }
                                });
                        });
                });
            }
        }
    }
}

pub(crate) fn item_actions_button_events(
    mut interaction_query: Query<
        (&Interaction, &SlotItemActionButton, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut net: EventWriter<OutgoingReliableClientMessage<ActionsClientMessage>>,
) {
    for (interaction, component, mut bg_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Clicked => {
                let mut gray = Color::MIDNIGHT_BLUE;
                gray.set_a(0.9);
                *bg_color = gray.into();
                net.send(OutgoingReliableClientMessage {
                    message: ActionsClientMessage::TabPressed(TabPressed {
                        id: component.action_id.clone(),
                        entity_option: Some(component.server_entity),
                        cell_option: None,
                        belonging_entity_option: None,
                    }),
                });
            }
            Interaction::Hovered => {
                let mut gray = Color::GRAY;
                gray.set_a(0.9);
                *bg_color = gray.into();
            }
            Interaction::None => {
                let mut gray = Color::DARK_GRAY;
                gray.set_a(0.9);
                *bg_color = gray.into();
            }
        }
    }
}
#[derive(Component)]
pub struct SlotItemActionButton {
    pub server_entity: Entity,
    pub action_id: String,
}
pub(crate) fn hide_actions(
    boarded: Res<Boarded>,
    mut events: EventReader<OpenInventoryHud>,
    query: Query<&Children, With<LeftContentHud>>,
    hud: Res<HudState>,
    mut commands: Commands,
) {
    for event in events.iter() {
        if !boarded.boarded {
            continue;
        }
        if !event.open {
            match query.get(hud.left_content_node) {
                Ok(children) => {
                    for child in children.iter() {
                        commands.entity(*child).despawn_recursive();
                    }
                }
                Err(_) => {
                    warn!("Could not find left content node");
                }
            }
        }
    }
}
