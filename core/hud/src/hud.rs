use bevy::{
    prelude::{
        warn, BuildChildren, Commands, Component, Entity, EventReader, NodeBundle, Query, Res,
        SystemLabel, With,
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum HudLabels {
    CreateHud,
}

pub fn create_hud(mut commands: Commands) {
    let mut builder = commands.spawn(HudRootNode);
    let entity = builder.id();

    let mut top_edge = Entity::from_bits(0);
    let mut center = Entity::from_bits(0);
    let mut bottom_edge = Entity::from_bits(0);

    let mut left = Entity::from_bits(0);
    let mut content = Entity::from_bits(0);
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
                                size: Size::new(Val::Percent(10.), Val::Percent(100.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(LeftEdgeHud)
                        .id();
                    content = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(80.), Val::Percent(100.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(ContentHud)
                        .with_children(|parent| {
                            left_content = parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(25.), Val::Percent(100.)),
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
                                        size: Size::new(Val::Percent(25.), Val::Percent(100.)),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(RightContentHud)
                                .id();
                        })
                        .id();
                    right = parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(10.), Val::Percent(100.)),
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
