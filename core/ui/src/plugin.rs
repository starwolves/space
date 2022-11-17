use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin};

use crate::text_input::{
    focus_events, input_characters, input_mouse_press_unfocus, ui_events, FocusTextInput,
    TextInput, TextInputLabel, UnfocusTextInput,
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(feature = "client") {
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
                .add_event::<FocusTextInput>();
        }
    }
}
