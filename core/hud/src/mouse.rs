use bevy::prelude::{EventReader, EventWriter, Input, MouseButton, Res};
use player::configuration::Boarded;
use resources::{
    hud::{EscapeMenuState, HudState},
    ui::{MainMenuState, TextInput},
};
use ui::{
    cursor::{GrabCursor, ReleaseCursor, WindowFocusBuffer},
    text_input::FocusTextInput,
};

use crate::{
    communication::input::ToggleCommunication,
    inventory::build::{InventoryHudState, OpenHud},
};

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

pub(crate) fn mouse_press_hud_unfocus(
    buttons: Res<Input<MouseButton>>,
    mut event: EventWriter<ToggleCommunication>,
    mut focus: EventReader<FocusTextInput>,
    text_input: Res<TextInput>,
    esc_state: Res<EscapeMenuState>,
    inv_state: Res<InventoryHudState>,
    mainmnu_state: Res<MainMenuState>,
) {
    if esc_state.visible || inv_state.open || mainmnu_state.enabled {
        return;
    }
    if buttons.just_pressed(MouseButton::Left) {
        let mut new_focus = None;
        for f in focus.iter() {
            new_focus = Some(f.entity);
        }

        let focus;
        match text_input.old_focus {
            Some(e) => {
                focus = Some(e);
            }
            None => {
                focus = text_input.focused_input;
            }
        }

        match focus {
            Some(e) => match new_focus {
                Some(x) => {
                    if e != x {
                        event.send(ToggleCommunication);
                    }
                }
                None => {
                    event.send(ToggleCommunication);
                }
            },
            None => {}
        }
    }
}
