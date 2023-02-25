use bevy::{
    prelude::{
        warn, BuildChildren, Commands, Component, Entity, EventReader, NodeBundle, Query, Res, With,
    },
    ui::{Display, FlexDirection, Size, Style, Val},
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

pub struct ExpandedLeftContentHud {
    pub expanded: bool,
}

pub const LEFT_RIGHT_CONTENT_HUD_WIDTH: f32 = 25.;
pub const LEFT_RIGHT_EDGE_HUD_WIDTH: f32 = 10.;
pub const LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH: f32 = 35.;
pub const CONTENT_NODE_EXPANDED_WIDTH: f32 = 80.;
pub const CONTENT_NODE_WIDTH: f32 = 50.;

pub fn create_hud(mut commands: Commands) {
    let mut builder = commands.spawn(HudRootNode);
    let entity = builder.id();

    let mut top_edge = Entity::from_bits(0);
    let mut center = Entity::from_bits(0);
    let mut bottom_edge = Entity::from_bits(0);

    let mut left = Entity::from_bits(0);
    let mut right = Entity::from_bits(0);

    let mut left_content = Entity::from_bits(0);
    let mut center_content = Entity::from_bits(0);
    let mut right_content = Entity::from_bits(0);

    builder
        .insert(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                display: Display::None,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            top_edge = parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(TopEdgeHud)
                .id();
            center = parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(80.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(CenterHud)
                .with_children(|parent| {
                    left = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH),
                                    Val::Percent(100.),
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(LeftEdgeHud)
                        .id();
                    left_content = parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::None,
                                size: Size::new(
                                    Val::Percent(LEFT_RIGHT_CONTENT_HUD_WIDTH),
                                    Val::Percent(100.),
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(LeftContentHud)
                        .id();
                    center_content = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.), Val::Percent(100.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(CenterContentHud)
                        .id();
                    right_content = parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::None,
                                size: Size::new(
                                    Val::Percent(LEFT_RIGHT_CONTENT_HUD_WIDTH),
                                    Val::Percent(100.),
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(RightContentHud)
                        .id();
                    right = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(
                                    Val::Percent(LEFT_RIGHT_EDGE_HUD_EXPANDED_WIDTH),
                                    Val::Percent(100.),
                                ),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(RightEdgeHud)
                        .id();
                })
                .id();
            bottom_edge = parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BottomEdgeHud)
                .id();
        });

    commands.insert_resource(HudState {
        root_entity: entity,
        expanded: false,
        left_content_node: left_content,
        right_content_node: right_content,
        center_content_node: center_content,
        left_edge_node: left,
        right_edge_node: right,
        top_edge_node: top_edge,
        bottom_edge_node: bottom_edge,
    });
}

/// Confirms connection with server.

pub(crate) fn show_hud(
    mut client2: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut query: Query<&mut Style, With<HudRootNode>>,
    hud: Res<HudState>,
) {
    for message in client2.iter() {
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
