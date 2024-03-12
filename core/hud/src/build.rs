use bevy::log::warn;
use bevy::{
    prelude::{
        BuildChildren, Commands, Component, Event, EventReader, NodeBundle, Query, Res, With,
    },
    ui::{Display, FlexDirection, Style, Val},
};

use networking::client::IncomingReliableServerMessage;
use player::net::PlayerServerMessage;
use resources::hud::HudState;

#[derive(Component)]
pub struct HudRootNode;

#[derive(Component)]
pub struct LeftHudVerticalRow;
#[derive(Component)]
pub struct RightHudVerticalRow;
#[derive(Component)]
pub struct CenterHudVerticalRow;

#[derive(Component)]
pub struct LeftEdgeHud;
#[derive(Component)]
pub struct RightEdgeHud;
#[derive(Component)]
pub struct TopEdgeHud;
#[derive(Component)]
pub struct BottomEdgeHud;
#[derive(Component)]
pub struct CenterHud;
#[derive(Component)]
pub struct ContentHud;
#[derive(Component)]
pub struct LeftContentHud;
#[derive(Component)]
pub struct CenterContentHud;
#[derive(Component)]
pub struct RightContentHud;
#[derive(Event)]
pub struct ExpandedLeftContentHud {
    pub expanded: bool,
}

pub const LEFT_RIGHT_CONTENT_HUD_WIDTH: f32 = 21.;
pub const LEFT_RIGHT_EDGE_HUD_WIDTH: f32 = 8.1;
pub const LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH: f32 = 29.2;
pub const CONTENT_NODE_EXPANDED_WIDTH: f32 = 80.;
pub const CONTENT_NODE_WIDTH: f32 = 50.;

pub fn create_hud(mut commands: Commands) {
    let mut builder = commands.spawn(HudRootNode);
    let entity = builder.id();

    let mut top_edge = None;
    let mut center = None;
    let mut bottom_edge = None;

    let mut left = None;
    let mut right = None;

    let mut left_content = None;
    let mut center_content = None;
    let mut right_content = None;

    builder
        .insert(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            top_edge = Some(
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(10.),
                            flex_direction: FlexDirection::RowReverse,

                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(TopEdgeHud)
                    .id(),
            );
            center = Some(
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(80.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(CenterHud)
                    .with_children(|parent| {
                        left = Some(
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH),
                                        height: Val::Percent(100.),
                                        flex_direction: FlexDirection::ColumnReverse,

                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(LeftEdgeHud)
                                .id(),
                        );
                        left_content = Some(
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        display: Display::None,

                                        width: Val::Percent(LEFT_RIGHT_CONTENT_HUD_WIDTH),
                                        height: Val::Percent(100.),
                                        ..Default::default()
                                    },

                                    ..Default::default()
                                })
                                .insert(LeftContentHud)
                                .id(),
                        );
                        center_content = Some(
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(50.),
                                        height: Val::Percent(100.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(CenterContentHud)
                                .id(),
                        );
                        right_content = Some(
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        display: Display::None,

                                        width: Val::Percent(LEFT_RIGHT_CONTENT_HUD_WIDTH),
                                        height: Val::Percent(100.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(RightContentHud)
                                .id(),
                        );
                        right = Some(
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH),
                                        height: Val::Percent(100.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(RightEdgeHud)
                                .id(),
                        );
                    })
                    .id(),
            );
            bottom_edge = Some(
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.),
                            height: Val::Percent(10.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(BottomEdgeHud)
                    .id(),
            );
        });

    commands.insert_resource(HudState {
        root_entity: entity,
        expanded: false,
        left_content_node: left_content.unwrap(),
        right_content_node: right_content.unwrap(),
        center_content_node: center_content.unwrap(),
        left_edge_node: left.unwrap(),
        right_edge_node: right.unwrap(),
        top_edge_node: top_edge.unwrap(),
        bottom_edge_node: bottom_edge.unwrap(),
    });
}

/// Confirms connection with server.

pub(crate) fn show_hud(
    mut client2: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut query: Query<&mut Style, With<HudRootNode>>,
    hud: Res<HudState>,
) {
    for message in client2.read() {
        match &message.message {
            PlayerServerMessage::InitGame => match query.get_mut(hud.root_entity) {
                Ok(mut style) => {
                    style.display = Display::Flex;
                }
                Err(_) => {
                    warn!("Could not find root entity");
                }
            },
            _ => (),
        }
    }
}
