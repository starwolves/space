use bevy::{
    prelude::{warn, EventReader, ResMut},
    window::{CursorGrabMode, Windows},
};
use networking::client::IncomingReliableServerMessage;
use player::net::PlayerServerMessage;

use crate::expand::ExpandInventoryHud;

pub(crate) fn grab_mouse_on_board(
    mut net: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut windows: ResMut<Windows>,
) {
    for message in net.iter() {
        match &message.message {
            PlayerServerMessage::Boarded => {
                match windows.get_primary_mut() {
                    Some(w) => {
                        w.set_cursor_grab_mode(CursorGrabMode::Locked);
                    }
                    None => {
                        warn!("Couldnt get primary window.");
                    }
                };
            }
            _ => (),
        }
    }
}

pub(crate) fn grab_mouse_hud_expand(
    mut events: EventReader<ExpandInventoryHud>,
    mut windows: ResMut<Windows>,
) {
    for event in events.iter() {
        match windows.get_primary_mut() {
            Some(w) => {
                if event.expand {
                    w.set_cursor_grab_mode(CursorGrabMode::None);
                } else {
                    w.set_cursor_grab_mode(CursorGrabMode::Locked);
                }
            }
            None => {
                warn!("Couldnt get primary window 2.");
            }
        };
    }
}
