use std::env;

use bevy::prelude::{App, Plugin};
use resources::labels::PreUpdateLabels;

use crate::{
    button::button_hover_visuals,
    networking::{incoming_messages, TextTreeInputSelection},
    text_input::{
        focus_events, input_characters, input_mouse_press_unfocus, ui_events, FocusTextInput,
        TextInput, TextInputLabel, UnfocusTextInput,
    },
};
use bevy::app::CoreStage::PreUpdate;
use bevy::prelude::IntoSystemDescriptor;
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system_to_stage(
                PreUpdate,
                incoming_messages
                    .after(PreUpdateLabels::NetEvents)
                    .label(PreUpdateLabels::ProcessInput),
            )
            .add_event::<TextTreeInputSelection>();
        } else {
            app.add_system(ui_events.label(TextInputLabel::UiEvents))
                .add_system(
                    focus_events
                        .before(TextInputLabel::UiEvents)
                        .label(TextInputLabel::MousePressUnfocus),
                )
                .add_system(input_mouse_press_unfocus.before(TextInputLabel::MousePressUnfocus))
                .init_resource::<TextInput>()
                .add_system(input_characters)
                .add_event::<UnfocusTextInput>()
                .add_event::<FocusTextInput>()
                .add_system(button_hover_visuals);
        }
    }
}
