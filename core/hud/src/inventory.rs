use bevy::{prelude::{Res, Input, KeyCode, EventWriter, Resource, EventReader, ResMut, Commands, Entity, NodeBundle, BuildChildren, Color, Component, Query, warn, With, Visibility}, ui::{Style, Size, Val}};
use networking::client::IncomingReliableServerMessage;
use player::{configuration::Boarded, net::PlayerServerMessage};

use crate::hud::HudState;

pub struct OpenInventoryHud {
    pub open : bool,
}

#[derive(Resource)]
pub struct InventoryState {
    pub open : bool,
    pub entity : Entity,
}

pub (crate) fn create_inventory_hud(mut commands : Commands, hud_state : Res<HudState>,    mut client: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
) {
    for message in client.iter() {
        match message.message {
            PlayerServerMessage::Boarded => {

                let entity_id = commands.spawn(InventoryHudRootNode).id();
                commands.entity(hud_state.center_content_node).add_child(entity_id);
                let mut builder = commands.entity(entity_id);
            
                builder
                .insert(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                        ..Default::default()
                    },
                    visibility: Visibility {
                        is_visible: false,
                    },
            
                    background_color: Color::DARK_GRAY.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
            
                });
            
                commands.insert_resource(InventoryState{
                    open: false,
                    entity: entity_id,
                });
            }
            _ => {},
        }
    }

}

#[derive(Component)]
pub struct InventoryHudRootNode;

pub(crate) fn open_inventory_hud(boarded : Res<Boarded>,mut events : EventReader<OpenInventoryHud>, mut state : ResMut<InventoryState>, mut root_node : Query<&mut Visibility, With<InventoryHudRootNode>>,) {

    for event in events.iter() {

        if !boarded.boarded {
            continue;
        }

        state.open = event.open;
        match root_node.get_mut(state.entity) {
            Ok(mut root) => {
                root.is_visible = state.open;
            },
            Err(_) => {
                warn!("Couldnt toggle open inventory , couldnt find root node.");
            },
        }


    }

}

pub(crate) fn inventory_hud_key_press(keys: Res<Input<KeyCode>>, mut event : EventWriter<OpenInventoryHud>, state : Res<InventoryState>,) {
    if keys.just_pressed(KeyCode::I) {
        event.send(OpenInventoryHud{
            open: !state.open,
        });
    }
}
