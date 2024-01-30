use bevy::{
    prelude::{Event, EventReader, Query, Res, ResMut, Resource, SystemSet, With},
    window::{CursorGrabMode, PrimaryWindow, Window, WindowFocused},
};

#[derive(Resource, Default)]
pub struct FocusState {
    pub focused: bool,
}

/// System label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum CursorSet {
    Perform,
}

#[derive(Event)]
pub struct GrabCursor;

pub fn grab_cursor(
    mut events: EventReader<GrabCursor>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,

    state: Res<FocusState>,
) {
    if !state.focused {
        return;
    }
    for _ in events.read() {
        let mut primary = primary_query.get_single_mut().unwrap();
        primary.cursor.grab_mode = CursorGrabMode::Locked;
        primary.cursor.visible = false;
    }
}
#[derive(Event)]
pub struct ReleaseCursor;

pub fn release_cursor(
    mut events: EventReader<ReleaseCursor>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for _ in events.read() {
        let mut primary = primary_query.get_single_mut().unwrap();
        primary.cursor.grab_mode = CursorGrabMode::None;
        primary.cursor.visible = true;
    }
}

pub(crate) fn focus_state(mut state: ResMut<FocusState>, events: Res<WindowFocusBuffer>) {
    for event in events.buffer.iter() {
        state.focused = event.focused;
    }
}
#[derive(Resource, Default)]
pub struct WindowFocusBuffer {
    pub buffer: Vec<WindowFocused>,
}

pub fn update_window_focus_buffer(
    mut events: EventReader<WindowFocused>,
    mut res: ResMut<WindowFocusBuffer>,
) {
    for e in events.read() {
        res.buffer.push(e.clone());
    }
}

pub(crate) fn clear_window_focus_buffer(mut res: ResMut<WindowFocusBuffer>) {
    res.buffer.clear();
}
