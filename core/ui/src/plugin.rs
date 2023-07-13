use bevy::prelude::{App, IntoSystemConfigs, Plugin, PreUpdate, Startup, Update};
use networking::messaging::{register_reliable_message, MessageSender};

use crate::{
    button::button_hover_visuals,
    fonts::{init_fonts, Fonts},
    hlist::{freeze_button, hlist_created, hlist_input, FreezeButton},
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
            app.add_systems(PreUpdate, incoming_messages)
                .add_event::<TextTreeInputSelection>();
        } else {
            app.add_systems(
                Update,
                (
                    ui_events.in_set(TextInputLabel::UiEvents),
                    focus_events
                        .before(TextInputLabel::UiEvents)
                        .in_set(TextInputLabel::MousePressUnfocus),
                ),
            )
            .add_systems(
                Update,
                (
                    input_mouse_press_unfocus.before(TextInputLabel::MousePressUnfocus),
                    input_characters,
                    button_hover_visuals,
                    set_text_input_node_text,
                    mouse_scroll_inverted,
                    mouse_scroll,
                    hlist_input,
                    hlist_created,
                    freeze_button.before(hlist_created),
                ),
            )
            .init_resource::<TextInput>()
            .add_event::<UnfocusTextInput>()
            .add_event::<FocusTextInput>()
            .add_event::<SetText>()
            .add_event::<FreezeButton>();
        }
        app.init_resource::<Fonts>()
            .add_systems(Startup, init_fonts);
        register_reliable_message::<UiClientMessage>(app, MessageSender::Client);
        register_reliable_message::<UiServerMessage>(app, MessageSender::Server);
    }
}
