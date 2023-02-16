use bevy::{prelude::{Resource, Entity, Commands, Component, NodeBundle, BuildChildren, info, EventReader}, ui::{Style, Size, Val}};
use networking::client::IncomingReliableServerMessage;
use player::net::PlayerServerMessage;

#[derive(Resource)]
pub struct HudState {
    pub expanded : bool,
    pub root_entity : Entity,
    pub left_node : Entity,
    pub right_node : Entity,
    pub center_node : Entity,
}
#[derive(Component)]
pub struct HudRootNode;

#[derive(Component)]
pub struct LeftHudVerticalRow;
#[derive(Component)]
pub struct RightHudVerticalRow;
#[derive(Component)]
pub struct CenterHudVerticalRow;

pub fn create_hud(mut commands : Commands,  mut client: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>){
    for message in client.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {
                let mut builder  =commands.spawn(HudRootNode);
                let entity =builder.id();
                let mut center_entity = Entity::from_bits(0);
                let mut right_entity = Entity::from_bits(0);
                let mut left_entity = Entity::from_bits(0);
            
                builder.insert(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
            
                    ..Default::default()
                })
                .with_children(|parent| {
                    let row_bundle = NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(33.3333), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    };
                    left_entity = parent.spawn(row_bundle.clone()).insert(LeftHudVerticalRow).id();
                    center_entity = parent.spawn(row_bundle.clone()).insert(CenterHudVerticalRow).id();
                    right_entity = parent.spawn(row_bundle).insert(RightHudVerticalRow).id();
                });
            
                info!("inserted hud state");
                commands.insert_resource(HudState {
                    root_entity: entity,
                    expanded: false,
                    left_node:left_entity,
                    right_node: right_entity,
                    center_node: center_entity,
                });
            }
, _ => ()
        }
    }
   
}
