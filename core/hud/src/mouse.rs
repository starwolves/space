use bevy::prelude::{EventReader, EventWriter, Input, MouseButton, Res};
use player::configuration::Boarded;
use resources::{hud::HudState, ui::TextInput};
use ui::{
    cursor::{GrabCursor, ReleaseCursor, WindowFocusBuffer},
    text_input::UnfocusTextInput,
};

use crate::{communication::input::ToggleCommunication, inventory::build::OpenHud};

pub(crate) fn window_unfocus_event(
    events: Res<WindowFocusBuffer>,
    state: Res<HudState>,
    boarded: Res<Boarded>,
    mut grab: EventWriter<GrabCursor>,
    mut release: EventWriter<ReleaseCursor>,
) {
    for event in events.buffer.iter() {
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

pub fn grab_mouse_hud_expand(
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

/// Manages focus of text input.

pub(crate) fn input_mouse_press_unfocus(
    buttons: Res<Input<MouseButton>>,
    text_input: Res<TextInput>,
    mut unfocus: EventWriter<UnfocusTextInput>,
    mut event: EventWriter<ToggleCommunication>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        match text_input.focused_input {
            Some(e) => {
                unfocus.send(UnfocusTextInput {
                    entity_option: Some(e),
                });
                event.send(ToggleCommunication);
            }
            None => {}
        }
    }
}
