use std::env;

use bevy::prelude::{App, Plugin};
use networking::messaging::{init_reliable_message, MessageSender};

use crate::{
    button::button_hover_visuals,
    networking::{incoming_messages, TextTreeInputSelection, UiClientMessage, UiServerMessage},
    text_input::{
        focus_events, input_characters, input_mouse_press_unfocus, set_text_input_node_text,
        ui_events, FocusTextInput, SetText, TextInput, TextInputLabel, UnfocusTextInput,
    },
};
use bevy::app::CoreStage::PreUpdate;
use bevy::prelude::IntoSystemDescriptor;
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system_to_stage(PreUpdate, incoming_messages)
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
                .add_system(button_hover_visuals)
                .add_event::<SetText>()
                .add_system(set_text_input_node_text);
        }

        init_reliable_message::<UiClientMessage>(app, MessageSender::Client);
        init_reliable_message::<UiServerMessage>(app, MessageSender::Server);
    }
}
