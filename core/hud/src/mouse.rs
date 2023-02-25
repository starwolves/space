use bevy::{
    prelude::{warn, EventReader, EventWriter, Res, ResMut},
    window::{CursorGrabMode, WindowFocused, Windows},
};
use networking::client::IncomingReliableServerMessage;
use player::{configuration::Boarded, net::PlayerServerMessage};
use resources::hud::HudState;

use crate::expand::ExpandInventoryHud;

pub(crate) fn grab_mouse_on_board(
    mut net: EventReader<IncomingReliableServerMessage<PlayerServerMessage>>,
    mut grab: EventWriter<GrabCursor>,
) {
    for message in net.iter() {
        match &message.message {
            PlayerServerMessage::Boarded => {
                grab.send(GrabCursor);
            }
            _ => (),
        }
    }
}
pub struct GrabCursor;

pub(crate) fn grab_cursor(mut events: EventReader<GrabCursor>, mut windows: ResMut<Windows>) {
    for _ in events.iter() {
        match windows.get_primary_mut() {
            Some(w) => {
                w.set_cursor_grab_mode(CursorGrabMode::Locked);
                w.set_cursor_visibility(false);
            }
            None => {
                warn!("Couldnt find main window.");
            }
        }
    }
}

pub struct ReleaseCursor;

pub(crate) fn release_cursor(mut events: EventReader<ReleaseCursor>, mut windows: ResMut<Windows>) {
    for _ in events.iter() {
        match windows.get_primary_mut() {
            Some(w) => {
                w.set_cursor_grab_mode(CursorGrabMode::None);
                w.set_cursor_visibility(true);
            }
            None => {
                warn!("Couldnt find main window.");
            }
        }
    }
}

pub(crate) fn window_unfocus_event(
    mut events: EventReader<WindowFocused>,
    state: Res<HudState>,
    boarded: Res<Boarded>,
    mut grab: EventWriter<GrabCursor>,
    mut release: EventWriter<ReleaseCursor>,
) {
    for event in events.iter() {
        if !event.focused {
            release.send(ReleaseCursor);
        } else {
            if state.expanded {
                release.send(ReleaseCursor);
            } else {
                if boarded.boarded {
                    grab.send(GrabCursor);
                }
            }
        }
    }
}

pub(crate) fn grab_mouse_hud_expand(
    mut events: EventReader<ExpandInventoryHud>,

    mut grab: EventWriter<GrabCursor>,
    mut release: EventWriter<ReleaseCursor>,
) {
    for event in events.iter() {
        if event.expand {
            release.send(ReleaseCursor);
        } else {
            grab.send(GrabCursor);
        }
    }
}
