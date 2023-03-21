use bevy::{
    prelude::{EventReader, EventWriter, Query, Res, ResMut, Resource, With},
    window::{CursorGrabMode, PrimaryWindow, Window, WindowFocused},
};
use networking::client::IncomingReliableServerMessage;
use player::{configuration::Boarded, net::PlayerServerMessage};
use resources::hud::HudState;

use crate::inventory::build::OpenHud;

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

pub fn grab_cursor(
    mut events: EventReader<GrabCursor>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,

    state: Res<FocusState>,
) {
    if !state.focused {
        return;
    }
    for _ in events.iter() {
        let mut primary = primary_query.get_single_mut().unwrap();
        primary.cursor.grab_mode = CursorGrabMode::Locked;
        primary.cursor.visible = false;
    }
}

pub struct ReleaseCursor;

pub fn release_cursor(
    mut events: EventReader<ReleaseCursor>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for _ in events.iter() {
        let mut primary = primary_query.get_single_mut().unwrap();
        primary.cursor.grab_mode = CursorGrabMode::None;
        primary.cursor.visible = true;
    }
}
#[derive(Resource, Default)]
pub struct FocusState {
    pub focused: bool,
}

pub(crate) fn focus_state(mut state: ResMut<FocusState>, mut events: EventReader<WindowFocused>) {
    for event in events.iter() {
        state.focused = event.focused;
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
    mut events: EventReader<OpenHud>,

    mut grab: EventWriter<GrabCursor>,
    mut release: EventWriter<ReleaseCursor>,
) {
    for event in events.iter() {
        if event.open {
            release.send(ReleaseCursor);
        } else {
            grab.send(GrabCursor);
        }
    }
}
