use bevy::prelude::EventWriter;
use bevy::prelude::{Commands, EventReader};
use bevy::prelude::{DespawnRecursiveExt, ResMut};
use networking::client::IncomingReliableServerMessage;

use crate::build::{EnableMainMenu, MainMenuState};

/// System that toggles the visiblity of the main menu based on an event.

pub(crate) fn hide_main_menu(
    mut enable_events: EventReader<EnableMainMenu>,
    mut state: ResMut<MainMenuState>,
    mut commands: Commands,
) {
    for event in enable_events.iter() {
        if event.enable == false {
            if !state.enabled {
                return;
            }
            state.enabled = false;
            commands.entity(state.root.unwrap()).despawn_recursive();
            state.root = None;
            commands.entity(state.camera.unwrap()).despawn_recursive();
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
