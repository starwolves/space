use bevy::{prelude::{Resource, Entity, Commands, Component, NodeBundle, BuildChildren, EventReader}, ui::{Style, Size, Val, FlexDirection}};
use networking::client::IncomingReliableServerMessage;
use player::net::PlayerServerMessage;

#[derive(Resource)]
pub struct HudState {
    pub expanded : bool,
    pub root_entity : Entity,
    pub left_content_node : Entity,
    pub right_content_node : Entity,
    pub center_content_node : Entity,
    pub left_edge_node : Entity,
    pub right_edge_node : Entity,
    pub top_edge_node : Entity,
    pub bottom_edge_node : Entity,
}
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

pub fn create_hud(mut commands : Commands,  mut client: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>){
    for message in client.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {
                let mut builder  =commands.spawn(HudRootNode);
                let entity =builder.id();

                let mut top_edge = Entity::from_bits(0);
                let mut center = Entity::from_bits(0);
                let mut bottom_edge = Entity::from_bits(0);

                let mut left = Entity::from_bits(0);
                let mut content = Entity::from_bits(0);
                let mut right = Entity::from_bits(0);

                let mut left_content = Entity::from_bits(0);
                let mut center_content = Entity::from_bits(0);
                let mut right_content = Entity::from_bits(0);
            
                builder.insert(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    top_edge = parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(TopEdgeHud).id();
                    center = parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(80.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(CenterHud).with_children(|parent| {

                        left = parent.spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(10.), Val::Percent(100.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        }).insert(LeftEdgeHud).id();
                        content = parent.spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(80.), Val::Percent(100.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        }).insert(ContentHud).with_children(|parent| {

                            let style = NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(33.333333), Val::Percent(100.)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            };

                            left_content = parent.spawn(style.clone()).insert(LeftContentHud).id();
                            center_content = parent.spawn(style.clone()).insert(CenterContentHud).id();
                            right_content = parent.spawn(style).insert(RightContentHud).id();

                        }).id();
                        right = parent.spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(10.), Val::Percent(100.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        }).insert(RightEdgeHud).id();

                    }).id();
                    bottom_edge = parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Percent(10.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    }).insert(BottomEdgeHud).id();
                });
            
                commands.insert_resource(HudState {
                    root_entity: entity,
                    expanded: false,
                    left_content_node:left_content,
                    right_content_node: right_content,
                    center_content_node: center_content,
                    left_edge_node: left,
                    right_edge_node: right,
                    top_edge_node: top_edge,
                    bottom_edge_node: bottom_edge,
                });
            },
            _ => ()
        }
    }
   
}
