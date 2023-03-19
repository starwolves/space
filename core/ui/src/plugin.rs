use bevy::prelude::{App, CoreSet, IntoSystemConfig, Plugin};
use networking::messaging::{register_reliable_message, MessageSender};

use crate::{
    button::button_hover_visuals,
    fonts::{init_fonts, Fonts},
    net::{UiClientMessage, UiServerMessage},
    scrolling::{mouse_scroll, mouse_scroll_inverted},
    text_input::{
        focus_events, incoming_messages, input_characters, input_mouse_press_unfocus,
        set_text_input_node_text, ui_events, FocusTextInput, SetText, TextInputLabel,
        TextTreeInputSelection, UnfocusTextInput,
    },
};
use resources::{is_server::is_server, ui::TextInput};
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(incoming_messages.in_base_set(CoreSet::PreUpdate))
                .add_event::<TextTreeInputSelection>();
        } else {
            app.add_system(ui_events.in_set(TextInputLabel::UiEvents))
                .add_system(
                    focus_events
                        .before(TextInputLabel::UiEvents)
                        .in_set(TextInputLabel::MousePressUnfocus),
                )
                .add_system(input_mouse_press_unfocus.before(TextInputLabel::MousePressUnfocus))
                .init_resource::<TextInput>()
                .add_system(input_characters)
                .add_event::<UnfocusTextInput>()
                .add_event::<FocusTextInput>()
                .add_system(button_hover_visuals)
                .add_event::<SetText>()
                .add_system(set_text_input_node_text)
                .add_system(mouse_scroll_inverted)
                .add_system(mouse_scroll);
        }
        app.init_resource::<Fonts>().add_startup_system(init_fonts);
        register_reliable_message::<UiClientMessage>(app, MessageSender::Client);
        register_reliable_message::<UiServerMessage>(app, MessageSender::Server);
    }
}
