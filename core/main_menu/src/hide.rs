use bevy::prelude::EventReader;
use bevy::prelude::EventWriter;
use bevy::prelude::ResMut;
use entity::despawn::DespawnEntity;
use networking::client::IncomingReliableServerMessage;
use resources::ui::MainMenuState;

use crate::build::EnableMainMenu;

/// System that toggles the visiblity of the main menu based on an event.

pub(crate) fn hide_main_menu(
    mut enable_events: EventReader<EnableMainMenu>,
    mut state: ResMut<MainMenuState>,
    mut despawn: EventWriter<DespawnEntity>,
) {
    for event in enable_events.iter() {
        if event.enable == false {
            if !state.enabled {
                return;
            }
            state.enabled = false;
            despawn.send(DespawnEntity {
                entity: state.root.unwrap(),
            });
            state.root = None;
            despawn.send(DespawnEntity {
                entity: state.camera.unwrap(),
            });
            state.camera = None;
        }
    }
}
use player::net::PlayerServerMessage;

/// Confirms connection with server.

pub(crate) fn confirm_connection(
    mut client2: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut enable_menu_events: EventWriter<EnableMainMenu>,
) {
    for message in client2.iter() {
        match &message.message {
            PlayerServerMessage::InitGame => {
                enable_menu_events.send(EnableMainMenu { enable: false });
            }
            _ => (),
        }
    }
}
